use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::views::login::login_screen::*;

    App = {{App}} {
        ui: <Window> {
            body = {
                <View> {
                    width: Fill, height: Fill,
                    flow: Overlay,

                    login_screen_view = <View> {
                        login_screen = <LoginScreen> {}
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx:&mut Cx) {
        // Order matters here, as some widget definitions depend on others.
        // `makepad_widgets` must be registered first,
        // then `global_styles`, `shared`,
        // then other modules wigets.
        makepad_widgets::live_design(cx);
        crate::global_styles::live_design(cx);
        crate::views::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        log!("App started");
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}