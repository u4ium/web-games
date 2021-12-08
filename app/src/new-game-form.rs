use yew::prelude::*;

#[derive(Debug)]
pub struct NewGameForm {
    link: ComponentLink<Self>,
}

impl Component for NewGameForm {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn update(&mut self, _: Self::Message) -> bool {
        false
    }
    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
    fn view(&self) -> Html {}
}
