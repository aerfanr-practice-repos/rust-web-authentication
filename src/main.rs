use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
    ShowPass
}

#[derive(Default)]
struct Model {
    result: String,
    username: String,
    password: String,
    show_pass: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::UpdateUsername(e) => {
                // web_sys::console::log_1(&format!("Username: {}", e).into());
                self.username = e;
                false
            }
            Msg::UpdatePassword(e) => {
                // web_sys::console::log_1(&format!("Password: {}", e).into());
                self.password = e;
                false
            }
            Msg::Submit => {
                self.result = format!("Username: {}\nPassword: {}", self.username, self.password);
                true
            }
            Msg::ShowPass => {
                self.show_pass = !self.show_pass;
                web_sys::console::log_1(&format!("Password shown: {:?}", self.show_pass).into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        macro_rules! callback {
            ( $t:expr ) => {
                ctx.link().batch_callback(|e: InputEvent| {
                    let target = e.target();
                    let input =
                        target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());
                    input.map(|input| $t(input.value()))
                })
            };
        }

        let username_change = callback!(Msg::UpdateUsername);
        let password_change = callback!(Msg::UpdatePassword);
        let on_submit = ctx.link().callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        let on_show_pass = ctx.link().callback(|_| Msg::ShowPass);

        let pass_type = match self.show_pass {
            true => "text",
            false => "password",
        };
        let pass_icon = match self.show_pass {
            true => "icon icon-eye-blocked",
            false => "icon icon-eye",
        };

        html! {
            <div class="root" role="main">
                <header>
                    <h1>
                        {"Interesting website"}
                    </h1>
                </header>
                <form onsubmit={on_submit} title="Login">
                    <h2>
                        {"Log in"}
                    </h2>
                    <span>
                        <label class="sides" for="username">
                            {"Username"}
                        </label>
                        <input oninput={username_change} id="username" name="username" type="text" class="text-input" placeholder="Username" aria-required="true"/>
                    </span>
                    <span>
                        <span class="sides">
                            <label for="password">
                                {"Password"}
                            </label>
                            <button type="button" onclick={on_show_pass} class="show-pass" tabindex="-1">
                                <i class={pass_icon} aria-hidden="true"></i>
                                {"Show"}
                            </button>
                        </span>
                        <input oninput={password_change} id="password" name="password" type={ pass_type } class="text-input" placeholder="Password" aria-required="true"/>
                    </span>
                    <span>
                        <input type="submit" class="submit" value="Log in"/>
                    </span>
                    <p class="result" role="alert"> {&self.result} </p>
                </form>
                <footer>
                    { "Icons from \"IcoMoon - Free (CC-BY)\"" }
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
