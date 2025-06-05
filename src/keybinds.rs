use bevy::prelude::*;

macro_rules! keybinds_struct {
    ($($field:ident: $type:ty [$($default:path),+])*) => {
        #[derive(Clone, Debug, Resource)]
        pub struct KeyBinds {
            $(
                $field: Box<[$type]>
            ),*
        }

        // make getters
        impl KeyBinds {
            $(
                pub fn $field(&self) -> impl Iterator<Item = $type> {
                    self.$field.iter().copied()
                }
            )*
        }

        impl Default for KeyBinds {
            fn default() -> Self {
                Self {
                    $(
                        $field: vec![$( $default ),+].into_boxed_slice()
                    ),*
                }
            }
        }
    };
}

keybinds_struct! {
    zoom: KeyPair [KeyPair::COMMA_PERIOD]
    rotation_speed: KeyPair [KeyPair::ARROWS_LR, KeyPair::KEY_AD]
    accelerate: KeyCode [KeyCode::ArrowUp, KeyCode::KeyW]
    trajectory_length: KeyPair [KeyPair::BRACKETS]
    reset: KeyCode [KeyCode::KeyR]
    toggle_debug_menu: KeyCode [KeyCode::F3]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyPair(pub KeyCode, pub KeyCode);

macro_rules! keypair_getters {
    ($($a:ident $b:ident)*) => {
        $(
        pub fn $a(self) -> KeyCode {
            self.0
        }

        pub fn $b(self) -> KeyCode {
            self.1
        }
        )*
    };
}

impl KeyPair {
    pub const BRACKETS: Self = Self(KeyCode::BracketLeft, KeyCode::BracketRight);
    pub const ARROWS_LR: Self = Self(KeyCode::ArrowLeft, KeyCode::ArrowRight);
    pub const COMMA_PERIOD: Self = Self(KeyCode::Comma, KeyCode::Period);
    pub const KEY_AD: Self = Self(KeyCode::KeyA, KeyCode::KeyD);

    keypair_getters! {
        left right
        down up
        less more
        shorter longer
    }
}
