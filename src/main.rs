use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
}

#[derive(Default)]
struct Model {
    result: String,
    username: String,
    password: String,
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
                true
            }
            Msg::UpdatePassword(e) => {
                // web_sys::console::log_1(&format!("Password: {}", e).into());
                self.password = e;
                true
            }
            Msg::Submit => {
                self.result = format!("Username: {}\nPassword: {}", self.username, self.password);
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
                        <label for="username">
                        {"Username"}
                        <input oninput={username_change} id="username" name="username" type="text" class="text-input" placeholder="Username" aria-required="true"/>
                        </label>
                    </span>
                    <span>
                        <label for="password">
                            {"Password"}
                            <input oninput={password_change} id="password" name="password" type="password" class="text-input" placeholder="Password" aria-required="true"/>
                        </label>
                    </span>
                    <span>
                        <input type="submit" class="submit" value="Log in"/>
                    </span>
                    <p class="result" role="alert"> {&self.result} </p>
                </form>
                <footer>
                    { "There will be some stuff in here." }
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
