use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct RevealStatus(pub bool);

impl From<bool> for RevealStatus {
    fn from(val: bool) -> Self {
        Self { 0: val }
    }
}

impl Into<bool> for RevealStatus {
    fn into(self) -> bool {
        self.0
    }
}
