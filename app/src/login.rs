use yew::prelude::*;

#[derive(Debug)]
pub struct LoginForm {
    link: ComponentLink<Self>,
    props: LoginFormProps,
    form_data: LoginFormData,
}

#[derive(Debug, Default)]
struct LoginFormData {
    username: String,
}

#[derive(Debug, Clone, Properties)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            form_data: Default::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Login => {
                // TODO submit fetch request, return user's ID
                self.props.login.emit(self.form_data.username.clone());
            }
            UpdateUsername(new_username) => {
                self.form_data.username = new_username;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let on_submit = self.link.callback(|e: yew::events::FocusEvent| {
            e.prevent_default();
            Login
        });
        let on_input = self.link.callback(|e: InputData| UpdateUsername(e.value));
        html! {
            <form onsubmit=on_submit>
                <h2>{"Login"}</h2>
                <label for="username">{"Username: "}</label>
                <input
                    id="username"
                    type="text"
                    name="username"
                    value=self.form_data.username.clone()
                    oninput=on_input
                />
                <button>{"Submit"}</button>
            </form>
        }
    }
}
