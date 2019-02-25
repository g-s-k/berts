use yew::prelude::*;

use beet_db::Item;

pub struct TrackList<'a> {
    is_fetching: bool,
    items: Vec<&'a Item>,
    deselect: Option<Callback<u32>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props<'a> {
    pub is_fetching: bool,
    pub items: Vec<&'a Item>,
    pub deselect: Option<Callback<u32>>,
}

pub enum Msg {
    Deselect(u32),
}

impl Component for TrackList<'static> {
    type Message = Msg;
    type Properties = Props<'static>;

    fn create(
        Props {
            is_fetching,
            items,
            deselect,
        }: Self::Properties,
        _: ComponentLink<Self>,
    ) -> Self {
        Self {
            is_fetching,
            items,
            deselect,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Deselect(id) => {
                if let Some(ref mut callback) = self.deselect {
                    callback.emit(id);
                }
            }
        }

        false
    }

    fn change(
        &mut self,
        Props {
            is_fetching, items, ..
        }: Self::Properties,
    ) -> ShouldRender {
        let should = is_fetching != self.is_fetching || items != self.items;

        if should {
            self.is_fetching = is_fetching;
            self.items = items;
        }

        should
    }
}

impl Renderable<TrackList<'static>> for TrackList<'static> {
    fn view(&self) -> Html<Self> {
        let track_list = self.items.iter().map(|item| {
            let id = item.id;
            html! {
                <tr class="TrackEntry", >
                    <td>
                        <span class="rm-btn", onclick=|_| Msg::Deselect(id), />
                    </td>
                    <td>{ &item.title }</td>
                    <td>{ &item.artist }</td>
                    <td>{ &item.album }</td>
                    <td>{ &item.genre }</td>
                    <td>{ &item.year }</td>
                    <td />
                </tr>
            }
        });

        let contents = if self.is_fetching {
            html! {
                <div class="EmptyTrackList", >
                { "Fetching track list from server" }
                </div>
            }
        } else if self.items.is_empty() {
            html! {
                <div class="EmptyTrackList", >
                    <span>{ "Playlist is empty."}</span>
                    <br />
                    <span>{"Add tracks by entering a query on the left and clicking on the entries that match." }</span>
                </div>
            }
        } else {
            html! {
                <table>
                    <thead>
                        <tr>
                            <th class="row-delete", />
                            <th class="row-title", >{ "Title" }</th>
                            <th class="row-artist", >{ "Artist" }</th>
                            <th class="row-album", >{ "Album" }</th>
                            <th class="row-genre", >{ "Genre" }</th>
                            <th class="row-year", >{ "Year" }</th>
                            <th />
                        </tr>
                    </thead>
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
