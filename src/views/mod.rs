use makepad_widgets::*;

/// Login module
pub mod login;
/// Home module
pub mod home;

pub fn live_design(cx: &mut Cx) {
    login::live_design(cx);
}