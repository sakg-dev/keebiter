use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo};

pub(crate) const COL: usize = 15; // between 7 and 15
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 2;

// using a!(No) either for gaps(keys not present physically) or for keys that should do nothing on if clicked
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [ k!(Escape),   k!(Kc1),  k!(Kc2),   k!(Kc3),  k!(Kc4),   k!(Kc5),   k!(Kc6),   k!(Kc7),  k!(Kc8),   k!(Kc9),   k!(Kc0),   k!(Minus), k!(Equal),       k!(Backslash),    a!(No)        ],
            [ k!(Tab),      k!(Q),    k!(W),     k!(E),    k!(R),     k!(T),     a!(No),    k!(Y),    k!(U),     k!(I),     k!(O),     k!(P),     k!(LeftBracket), k!(RightBracket), k!(Backspace) ],
            [ k!(CapsLock), k!(A),    k!(S),     k!(D),    k!(F),     k!(G),     a!(No),    k!(H),    k!(J),     k!(K),     k!(L),     a!(No),    k!(Semicolon),   k!(Quote),        k!(Enter)     ],
            [ k!(LShift),   k!(Z),    k!(X),     k!(C),    k!(V),     k!(B),     a!(No),    k!(N),    k!(M),     k!(Comma), k!(Dot),   a!(No),    k!(Slash),       a!(No),           k!(RShift)    ],
            [ k!(LCtrl),    a!(No),   a!(No),    k!(LAlt), k!(Space), mo!(1),    k!(LGui),  a!(No),   k!(Space), a!(No),    a!(No),    a!(No),    a!(No),          a!(No),           k!(RCtrl)     ]
        ]),
        layer!([
            [ k!(Grave),    k!(F1),   k!(F2),    k!(F3),   k!(F4),    k!(F5),    k!(F6),    k!(F7),   k!(F8),    k!(F9),    k!(F10),   k!(F11),   k!(F12),         a!(No),           a!(No)        ],
            [ a!(No),       a!(No),   a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    a!(No),    a!(No),          a!(No),           k!(Delete)    ],
            [ a!(No),       a!(No),   a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    k!(Left), k!(Down),  k!(Up),    k!(Right), a!(No),    a!(No),          a!(No),           a!(No)        ],
            [ a!(No),       a!(No),   a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    a!(No),    a!(No),          a!(No),           a!(No)        ],
            [ a!(No),       a!(No),   a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    a!(No),   a!(No),    a!(No),    a!(No),    a!(No),    a!(No),          a!(No),           a!(No)        ]
        ])
    ]
}
// TODO:
// for every layer, 15th, a!(No) is special key, write custom thing like one tap-silent voice, long press - sleep etc etc
// When converting Esc to Grave in fn, we want shifted grave too, research how to implement that..
