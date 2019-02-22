use failure::Error;
use stdweb::{__internal_console_unsafe, _js_impl, console, js};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    Task,
};

use beet_db::{Album, Item};

mod tracks;

use tracks::TrackList;

pub enum Msg {
    Input(String),
    Clear,
    RequestFailed,
    FetchAlbums,
    AlbumsFetched(Result<Vec<Album>, Error>),
    FetchItems,
    ItemsFetched(Result<Vec<Item>, Error>),
}

pub struct App {
    link: ComponentLink<App>,
    fetch_service: FetchService,
    fetch_tasks: Vec<FetchTask>,
    query: String,
    albums: Vec<Album>,
    items: Vec<Item>,
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
            query: String::new(),
            albums: Vec::new(),
            items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clear => {
                self.query.clear();
                js! { document.getElementById("searchbar").focus() }
            }
            Msg::Input(s) => {
                self.query = s;
            }
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
            }
            Msg::ItemsFetched(Err(e)) => {
                console!(error, format!("Fetch error: {:#?}", e));
                self.prune_fetches();
            }
        }

        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let filter_list = if self.query.is_empty() {
            html! { <div class="EmptyFilterList", >{ "No filter applied" }</div> }
        } else {
            html! { <ul class="FilterList", >
                     </ul> }
        };

        html! {
            <div class="SplitPane", >
                <div class="SideNav", >
                    <div class="input", >
                        <input
                            id="searchbar",
                            placeholder="Enter query...",
                            oninput=|e| Msg::Input(e.value),
                            value=&self.query,
                        />
                        <i onclick=|_| Msg::Clear, >{ "×" }</i>
                    </div>
                { filter_list }
                </div>
                <div class="Collection", >
                    <div class="ArtView", >
                    </div>
                    <div class="PaneDivider", />
                    <TrackList: items={ &self.items }, />
                </div>
            </div>
        }
    }
}

impl App {
    fn prune_fetches(&mut self) {
        self.fetch_tasks.retain(Task::is_active);
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
