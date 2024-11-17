use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;


    LoginForm = {{LoginForm}} {
        width: Fit, height: Fit,
        spacing: 10,
        flow: Down,

        padding: {
            left: 20,
            right: 20,
            top: 20,
            bottom: 20,
        }

        title = <Label> {
            text: "Welcome to Login",
            draw_text: {
                text_style: {
                    font_size: 12.0
                }
            }
        }

        sub_title = <Label> {
            text: "Project Robius Community",
            draw_text: {
                text_style: {
                    font_size: 16.0
                }
            }
        }

        form = <View> {
            width: Fit, height: Fit,
            flow: Down,
            spacing: 10,

            username = <TextInput> {
                empty_message: "Username",
            }

            password = <TextInput> {
                empty_message: "Password",
            }

            login_button = <Button> {
                width: Fill,
                text: "Login",
            }
        }
    }
}

#[derive(LiveHook, Live, Widget)]
pub struct LoginForm {
    #[deref]
    view: View,
}


impl Widget for LoginForm {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        self.view.handle_event(_cx, _event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
