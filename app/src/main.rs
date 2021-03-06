extern crate engines;

use yew::prelude::*;

mod login;
use login::LoginForm;

mod chess;
use chess::{Black, ChessBoard, White};

pub enum AppMessage {
    Login(String),
    Logout,
}
use AppMessage::*;

#[derive(Debug)]
pub struct App {
    user: Option<String>,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { user: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Login(user_name) => self.user = Some(user_name),
            Logout => self.user = None,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let login = ctx.link().callback(|username: String| Login(username));
        let logout = ctx.link().callback(|_| Logout);
        html! {
            <main class="app">
                <h1>{ "Web Games" }</h1>
                {match &self.user {
                    Some(username) => html! {
                        <div>
                            <p>
                                {format!("Welcome to web games, {}!", username)}
                            </p>

                            <ChessBoard
                                players={vec![White, Black]}
                                show_moves={true}
                                // ai={1}
                            />

                            <br />
                            <hr />
                            <br />

                            <button onclick={logout}>{"Logout"}</button>
                        </div>
                    },
                    None => html! {
                        <LoginForm login={login} />
                    }
                }}
            </main>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
