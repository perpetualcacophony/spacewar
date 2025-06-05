use bevy::prelude::*;

#[derive(Clone, Debug, Resource)]
pub struct KeyBinds {
    pub zoom: Vec<KeyPair>,
    pub rotation_speed: Vec<KeyPair>,
    pub accelerate: Vec<KeyCode>,
    pub trajectory_length: Vec<KeyPair>,
    pub reset: Vec<KeyCode>,
    pub toggle_debug_menu: Vec<KeyCode>,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            zoom: vec![KeyPair::COMMA_PERIOD],
            rotation_speed: vec![KeyPair::ARROWS_LR, KeyPair::KEY_AD],
            accelerate: vec![KeyCode::ArrowUp, KeyCode::KeyW],
            trajectory_length: vec![KeyPair::BRACKETS],
            reset: vec![KeyCode::KeyR],
            toggle_debug_menu: vec![KeyCode::F3],
        }
    }
}

macro_rules! keybinds_iter_getters {
    ($($field:ident $type:ty)*) => {
        $(
            pub fn $field(&self) -> impl Iterator<Item = $type> {
                self.$field.iter().copied()
            }
        )*
    };
}

impl KeyBinds {
    keybinds_iter_getters! {
        zoom KeyPair
        rotation_speed KeyPair
        accelerate KeyCode
        trajectory_length KeyPair
        reset KeyCode
        toggle_debug_menu KeyCode
    }
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
