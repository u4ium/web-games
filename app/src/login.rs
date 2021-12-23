use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct LoginForm {
    form_data: LoginFormData,
}

#[derive(Debug, Default)]
struct LoginFormData {
    username: String,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LoginFormProps {
    pub login: Callback<String>,
}

pub enum LoginMessage {
    Login,
    UpdateUsername(String),
}
use LoginMessage::*;

impl Component for LoginForm {
    type Message = LoginMessage;
    type Properties = LoginFormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            form_data: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Login => {
                // TODO submit fetch request, return user's ID
                ctx.props().login.emit(self.form_data.username.clone());
            }
            UpdateUsername(new_username) => {
                self.form_data.username = new_username;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit = ctx.link().callback(|e: yew::events::FocusEvent| {
            e.prevent_default();
            Login
        });
        let on_input = ctx.link().batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| UpdateUsername(input.value()))
        });
        html! {
            <form onsubmit={on_submit}>
                <h2>{"Login"}</h2>
                <label for="username">{"Username: "}</label>
                <input
                    id="username"
                    type="text"
                    name="username"
                    value={self.form_data.username.clone()}
                    oninput={on_input}
                />
                <button>{"Submit"}</button>
            </form>
        }
    }
}
