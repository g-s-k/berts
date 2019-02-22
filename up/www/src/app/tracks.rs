use yew::prelude::*;

use beet_db::Item;

pub struct TrackList {
    is_fetching: bool,
    items: Vec<Item>,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props {
    pub is_fetching: bool,
    pub items: Vec<Item>,
}

impl Component for TrackList {
    type Message = ();
    type Properties = Props;

    fn create(Props { is_fetching, items }: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { is_fetching, items }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, Props { is_fetching, items }: Self::Properties) -> ShouldRender {
        self.is_fetching = is_fetching;
        self.items = items;
        true
    }
}

impl Renderable<TrackList> for TrackList {
    fn view(&self) -> Html<Self> {
        let track_list = self.items.iter().map(|item| {
            html! {
                <tr class="TrackEntry", >
                    <td>{ &item.title }</td>
                    <td>{ &item.artist }</td>
                    <td>{ &item.album }</td>
                </tr>
            }
        });

        let contents = if self.is_fetching {
            html! {
                <div class="EmptyTrackList", >
                { "Fetching track list from server" }
                </div>
            }
        } else {
            html!{
                <table>
                    <tbody>
                        { for track_list }
                    </tbody>
                </table>
            }
        };

        html! {
            <div class="TrackList", >
                { contents }
            </div>
        }
    }
}
