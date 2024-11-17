use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::view::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::views::login::login_form::LoginForm;

    LoginScreen = {{LoginScreen}} {
        width: Fill, height: Fill,

        align: {
            x: 0.5,
            y: 0.5
        },
        <LoginForm> {}
    }
}

#[derive(LiveHook, Live, Widget)]
pub struct LoginScreen {
    #[deref]
    view: View,
}

impl Widget for LoginScreen {

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        self.view.handle_event(_cx, _event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}