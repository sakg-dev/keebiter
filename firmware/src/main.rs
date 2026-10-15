#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::flash::{Async, Flash};
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::{DMA_CH0, USB};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, dma};
use keymap::{COL, ROW};
use panic_probe as _;
use rmk::config::{BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::host::HostService;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::processor::builtin::wpm::WpmProcessor;
use rmk::usb::UsbTransport;
use rmk::watchdog::Rp2040Watchdog;
use rmk::{KeymapData, initialize_keymap_and_storage, run_all};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>; // DMS: Direct Memory Access
});

const FLASH_SIZE: usize = 2 * 1024 * 1024; // in bytes

#[embassy_executor::main] // creates a new executor instance.
async fn main(_spawner: Spawner) {
    info!("RMK initialized!");

    let p = embassy_rp::init(Default::default()); // Initialize peripherals
    // let serial_number = rmk::config::rp2040_serial_number(p.FLASH.reborrow()); TODO: this function not found in the crate.

    let usb_driver = Driver::new(p.USB, Irqs); // Create the usb driver, from the HAL

    // pin configs: one(rows or cols) needs to be inputs while other outputs so that when presses, they get connected and flow electricity
    let (row_pins, col_pins) = config_matrix_pins_rp!(
        peripherals: p, 
        input: [PIN_1, PIN_2, PIN_4, PIN_5, PIN_6], // ROWS
        output: [PIN_7, PIN_9, PIN_10, PIN_11, PIN_12, PIN_14, PIN_15, PIN_16, PIN_17, PIN_19, PIN_20, PIN_21, PIN_22, PIN_24, PIN_25] // COLS
    );

    // Use internal flash to emulate eeprom
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0, Irqs); // Flash Driver

    let keyboard_device_config = DeviceConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "Sak G",
        product_name: "SakG's Keeb",
        // serial_number,
        ..DeviceConfig::default()
    };

    let rmk_config = RmkConfig {
        device_config: keyboard_device_config,
        ..Default::default()
    };

    // iinitializing storage and keymap
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let storage_config = StorageConfig::default();
    let mut behavior_config = BehaviorConfig::default();
    let per_key_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &per_key_config,
    ).await;

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::new(); // debouncing is a method in which we wait for a time(debouncing time) to let the signal stabalize and register an intentional signal, avoiding repeated unnacasary clicks
    let mut matrix = Matrix::<_, _, _, ROW, COL, true>::new(row_pins, col_pins, debouncer);
    let mut keyboard = Keyboard::new(&keymap);
    let host_service = HostService::new(&keymap, &rmk_config);

    let mut usb_transport = UsbTransport::new(usb_driver, rmk_config.device_config).with_host_service(&host_service);
    let mut wpm_processor = WpmProcessor::new();

    let mut watchdog_runner = Rp2040Watchdog::default_runner(embassy_rp::watchdog::Watchdog::new(p.WATCHDOG));

    info!("Running the keebiter");
    run_all!(matrix, storage, usb_transport, wpm_processor, keyboard, watchdog_runner).await;
}
