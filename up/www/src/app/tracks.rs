use yew::prelude::*;

use beet_db::Item;

pub struct TrackList {
    items: Vec<Item>,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props {
    pub items: Vec<Item>,
}

impl Component for TrackList {
    type Message = ();
    type Properties = Props;

    fn create(Props { items }: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { items }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, Props { items }: Self::Properties) -> ShouldRender {
        self.items = items;
        true
    }
}

impl Renderable<TrackList> for TrackList {
    fn view(&self) -> Html<Self> {
        let track_list = self.items.iter().map(|item| {
            html! {
                <tr>
                    <td>{ &item.title }</td>
                    <td>{ &item.artist }</td>
                    <td>{ &item.album }</td>
                </tr>
            }
        });

        html! {
            <div class="TrackList", >
                <table>
                    <tbody>
                        { for track_list }
                    </tbody>
                </table>
            </div>
        }
    }
}
