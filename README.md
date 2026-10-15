<h1 align="center">Keebiter</h1>

*<h5 align="center">My first self made Mechanical Keyboard built through **iteration**</h5>*
-------
<br>

## What is this?
**A 60% Alice Style Tactile Mechanical Keyboard**
- **Hot Swappable Switches:** through kailh socker
- **61 keys:** No dedicated arrow keys, numpads or F-x keys
- **Rgb light:** individual Addressable RGBs
- **Firmware:** RMK Firmware

## Why?
It started 2 mnths ago in the beginning of august, i was kinda new in vim and wanted to become a complete keyboard guy and then i saw the YSWS in hackclub named keeb that would fund me if i make the keyboard, at the beginning, i had no idea how tf a mechanical keyboard works but slowly got to know all things and built a keyboard yay. It would been built in ~2 weeks had i not had my exams and other projects :/

-------

### Renders:
![](/imgs/readme_final_assembly.png)
![](/imgs/readme_final_main_schematics.png)
![](/imgs/readme_final_switches_schematics.png)
![](/imgs/readme_final_rgbs_schematics.png)
![](/imgs/readme_final_pcb_fcu.png)
![](/imgs/readme_final_pcb_bcu.png)
![](/imgs/readme_final_pcb_3d.png)
![](/imgs/case_11.png)
![](/imgs/case_12.png)

-------

### Development And Output Files
- Layout: [layout_v1.json](/layout/layout_v1.json)
- Schematics and PCB: [kicad/](/kicad/)
- Gerber file: [gerber.zip](/gerber.zip)
- Onshape exports: [onshape_exports/](/onshape_exports/)
- BOM: [BOM.csv](/BOM.csv)
- Onshape Document: [link](https://cad.onshape.com/documents/9aab3062eb3c9d6b80a1ab5a/w/c58e3f2f51a15de1421f10c6/e/e458f6ddd190f7c08079581c)
- Firmware: [firmware/](/firmware/)

## BOM
[BOM.csv](/BOM.csv)
| Part                             | Quantity               | Shown Price    | Link                                                                                            |
|----------------------------------|------------------------|----------------|-------------------------------------------------------------------------------------------------|
| PCB                              | 5(minimum site allows) | $40            | JLCPCB - [img](/imgs/pcb.png)                                                                   |
| Raspberry Pi Pico                | 1                      | $4             | https://robu.in/product/raspberry-pi-pico/                                                      |
| 1N4148 Diodes                    | 61                     | $1.5           | https://robu.in/product/1n4148-surface-mount-zener-diode-pack-of-30/                            |
| Capacitor 470uF                  | 1                      |                |                                                                                                 |
| Capacitors 100nF                 | 12                     |                |                                                                                                 |
| Resistor 300 ohm                 | 1                      |                |                                                                                                 |
| SK6812 MINI-E                    | 61                     | $5.5           | https://www.etstore.in/products/e9974?variant=48993209319675                                    |
| GATERON G Pro 3.0 Brown Switches | 75(2 packs - each 35)  | $41            | https://stackskb.com/store/gateron-hotswap-sockets/ - [img](/imgs/switches.png)                 |
| Kailh hot swap socket            | 61                     | $6.3           | https://stackskb.com/store/gateron-hotswap-sockets/                                             |
| Keycaps set                      | 61                     | $79            | [img](/imgs/keycaps.png)                                                                        |
| Stabalizers 2u                   | 4                      | $4.8           | https://www.gateron.com/products/gateron-pcb-mounted-stabilizer?VariantsId=10208                |
| Stabalizer 3u                    | 1                      | $3             | https://shockport.ca/products/cherry-pcb-mount-stabilizers                                      |
| Gasket strip roll                | 1                      | $2.3           | https://www.amazon.in/Aicon-Sponge-Rubber-Adhesive-Insulation/dp/B0DRTJMKD6/                    |
| Heatset insert                   | 6                      | $0.5           | https://onlyscrews.in/products/m2-x-4mm-brass-threaded-inserts                                  |
| Screws                           | 6                      | $0.5           | https://onlyscrews.in/products/m2-x-10mm-phillips-round-head-laptop-screw-dia-2mm-length-10mm   |
| Usb Cable                        | 1                      | $2.7           | https://robu.in/product/5v-3a-usb-to-micro-usb-power-cable-with-led-indicator-for-raspberry-3b/ |
| KeyCaps And Shaft Puller         | 1                      | $4.2           | https://www.amazon.in/Eatech-Accessories-Unloading-Keyboard-Switches/dp/B088BYS5RW              |
| 3d printed Case and Plate        | 1                      | Not calculated |                                                                                                 |
| Total                            |                        | $193           |                                                                                                 |

## Journal
See [JOURNAL.md](/JOURNAL.md)

## License
It's under [MIT license](./LICENSE), so copy it, modify it, sell it, become a billionaire, i expect nothing in return. But if it breaks, explodes, catches fire, or opens a portal to nether, that’s between you and your hardware. I was never here >~<

## AI usage
Used ai to know more about how a mechanical keyboard works under the hood, measurements i should use, design inspiration, component links and what would be best for me
