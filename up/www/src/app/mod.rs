use yew::prelude::*;

pub enum Msg {
    Input(String),
    Clear,
}

pub struct App {
    query: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            query: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clear => self.query.clear(),
            Msg::Input(s) => self.query = s,
        }

        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="SplitPane", >
                <div class="SideNav", >
                <div class="input", >
                <input
                    placeholder="Enter query...",
                    oninput=|e| Msg::Input(e.value),
                    value=&self.query,
                />
                <i onclick=|_| Msg::Clear, >{ "×" }</i>
                </div>
                </div>
                <div class="Collection", >
                    <div class="ArtView", >
                    </div>
                    <div class="PaneDivider", />
                    <div class="TrackList", >
                    </div>
                </div>
            </div>
        }
    }
}
