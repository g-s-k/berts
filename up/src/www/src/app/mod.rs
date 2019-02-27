use std::collections::HashSet;

use failure::Error;
use stdweb::{__internal_console_unsafe, _js_impl, console, js};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    Task,
};

use beet_db::{Album, Item};

mod filter;
mod tracks;

use filter::Filter;
use tracks::TrackList;

pub enum Msg {
    RequestFailed,
    FetchAlbums,
    AlbumsFetched(Result<Vec<Album>, Error>),
    FetchItems,
    ItemsFetched(Result<Vec<Item>, Error>),
    SelectAlbums(HashSet<u32>),
    SelectItems(HashSet<u32>),
    DeselectItem(u32),
    ClearSelection,
    SetCurrent(u32),
    SetShuffle(bool),
}

pub struct App {
    link: ComponentLink<App>,
    fetch_service: FetchService,
    fetch_tasks: Vec<FetchTask>,
    albums: Vec<Album>,
    items: Vec<Item>,
    selected: HashSet<u32>,
    current: Option<Item>,
    shuffle: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_self(Msg::FetchAlbums);
        link.send_self(Msg::FetchItems);

        Self {
            link,
            fetch_service: FetchService::new(),
            fetch_tasks: Vec::new(),
            albums: Vec::new(),
            items: Vec::new(),
            selected: HashSet::new(),
            current: None,
            shuffle: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestFailed => self.prune_fetches(),
            Msg::FetchAlbums => {
                let req = Request::get("http://localhost:8337/album")
                    .body(Nothing)
                    .unwrap();
                let task = self
                    .fetch_service
                    .fetch(req, self.link.send_back(album_fetch_cback));
                self.fetch_tasks.push(task);
            }
            Msg::AlbumsFetched(Ok(a)) => {
                self.albums = a;
                self.prune_fetches();
            }
            Msg::AlbumsFetched(Err(e)) => {
                console!(error, format!("Fetch error: {:#?}", e));
                self.prune_fetches();
            }
            Msg::FetchItems => {
                let req = Request::get("http://localhost:8337/item")
                    .body(Nothing)
                    .unwrap();
                let task = self
                    .fetch_service
                    .fetch(req, self.link.send_back(item_fetch_cback));
                self.fetch_tasks.push(task);
            }
            Msg::ItemsFetched(Ok(a)) => {
                self.items = a;
                self.prune_fetches();
                self.init_selected();
            }
            Msg::ItemsFetched(Err(e)) => {
                console!(error, format!("Fetch error: {:#?}", e));
                self.prune_fetches();
            }
            Msg::SelectAlbums(a_ids) => {
                for Item { id, album_id, .. } in &self.items {
                    match album_id {
                        Some(album_id) if a_ids.contains(album_id) => {
                            self.selected.insert(*id);
                        }
                        _ => (),
                    }
                }
            }
            Msg::SelectItems(ids) => {
                self.selected.extend(ids);
            }
            Msg::DeselectItem(id) => {
                self.selected.remove(&id);
            }
            Msg::ClearSelection => {
                self.selected.clear();
                self.current.take();
            }
            Msg::SetCurrent(s_id) => {
                self.current = self
                    .items
                    .iter()
                    .find(|Item { id, .. }| *id == s_id)
                    .cloned();
            }
            Msg::SetShuffle(b) => self.shuffle = b,
        }

        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let selected_tracks = self
            .items
            .iter()
            .filter(|Item { id, .. }| self.selected.contains(id))
            .collect::<Vec<_>>();

        let meta = if let Some(item) = &self.current {
            html! {
                <>
                    <span>{ &item.title }</span>
                    <br />
                    <span>{ &item.artist }</span>
                    <br />
                    <span>{ &item.album }</span>
                </>
            }
        } else {
            html! { {"No track playing"} }
        };

        let shuffle = self.shuffle;

        html! {
            <>
                <Filter:
                    albums=&self.albums,
                    items=&self.items,
                    select_album=Msg::SelectAlbums,
                    select_item=Msg::SelectItems,
                />
                <div class="Playlist", >
                    <div class="TopBar", >
                        <div class="Player", >
                            <div>{ "No album art available" }</div>
                            <audio controls="",>
                                { "Your browser does not support the HTML5 " }
                                <code>{ "audio" }</code>
                                { " tag." }
                            </audio>
                        </div>
                        <div class="Info", >
                            <div class="Metadata", >
                                {meta}
                            </div>
                            <div class="Controls", >
                                <button onclick=|_| Msg::ClearSelection, >
                                    { "Clear playlist" }
                                </button>
                                <label>
                                    <input
                                        type="checkbox",
                                        checked=shuffle,
                                        name="shuffle",
                                        onclick=|_| Msg::SetShuffle(!shuffle),
                                    />
                                    <span>{ "Shuffle" }</span>
                                </label>
                            </div>
                        </div>
                    </div>
                    <TrackList:
                        is_fetching={ !self.fetch_tasks.is_empty() },
                        items={ selected_tracks },
                        deselect=Msg::DeselectItem,
                        play_now=Msg::SetCurrent,
                    />
                </div>
            </>
        }
    }
}

impl App {
    fn prune_fetches(&mut self) {
        self.fetch_tasks.retain(Task::is_active);
    }

    fn init_selected(&mut self) {
        self.selected = self
            .items
            .iter()
            .rev()
            .take(250)
            .map(|Item { id, .. }| *id)
            .collect();
    }
}

fn album_fetch_cback(response: Response<Json<Result<Vec<Album>, Error>>>) -> Msg {
    let (meta, Json(data)) = response.into_parts();

    if meta.status.is_success() {
        Msg::AlbumsFetched(data)
    } else {
        Msg::RequestFailed
    }
}

fn item_fetch_cback(response: Response<Json<Result<Vec<Item>, Error>>>) -> Msg {
    let (meta, Json(data)) = response.into_parts();

    if meta.status.is_success() {
        Msg::ItemsFetched(data)
    } else {
        Msg::RequestFailed
    }
}
