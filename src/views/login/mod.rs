use makepad_widgets::*;

pub mod login_screen;
pub mod login_form;

pub fn live_design(cx: &mut Cx) {
    login_form::live_design(cx);
    login_screen::live_design(cx);
}