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
use beet_query::Query;

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
    SelectAlbum(u32),
    SelectItem(u32),
    ClearSelection,
}

pub struct App {
    link: ComponentLink<App>,
    fetch_service: FetchService,
    fetch_tasks: Vec<FetchTask>,
    query: String,
    albums: Vec<Album>,
    items: Vec<Item>,
    selected: HashSet<u32>,
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
            selected: HashSet::new(),
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
                self.init_selected();
            }
            Msg::ItemsFetched(Err(e)) => {
                console!(error, format!("Fetch error: {:#?}", e));
                self.prune_fetches();
            }
            Msg::SelectAlbum(a_id) => {
                for Item { id, album_id, .. } in &self.items {
                    match album_id {
                        Some(album_id) if *album_id == a_id => {
                            self.selected.insert(*id);
                        }
                        _ => (),
                    }
                }
            }
            Msg::SelectItem(id) => {
                self.selected.insert(id);
            }
            Msg::ClearSelection => self.selected.clear(),
        }

        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let filter_list = if self.query.is_empty() {
            html! { <div class="EmptyFilterList", >{ "No filter applied" }</div> }
        } else {
            let q = self.query.parse::<Query>().unwrap();

            let mut filtered_albums = self
                .albums
                .iter()
                .filter(|album| q.match_album(album))
                .take(15)
                .peekable();

            let album_list = if filtered_albums.peek().is_none() {
                html! { <li class="EmptyFilterSection", >{ "No matches." }</li> }
            } else {
                html! {
                    <>
                    { for filtered_albums
                      .map(|album| {
                          let id = album.id;
                          let title = format!("{} - {} [{}]", album.albumartist, album.album, album.year);
                          html! {
                              <li>
                                  <span onclick=|_| Msg::SelectAlbum(id), title={ title }, >
                                      { &album.album }
                                  </span>
                              </li>
                          }
                      }) }
                    </>
                }
            };

            let mut filtered_items = self
                .items
                .iter()
                .filter(|item| q.match_item(item))
                .take(50)
                .peekable();

            let item_list = if filtered_items.peek().is_none() {
                html! { <li class="EmptyFilterSection", >{ "No matches." }</li> }
            } else {
                html! {
                    <>
                    { for filtered_items
                      .map(|item| {
                          let id = item.id;
                          html! {
                              <li>
                                  <span onclick=|_| Msg::SelectItem(id), >
                                      { &item.title }
                                  </span>
                              </li>
                          }
                      }) }
                    </>
                }
            };

            html! {
                <ul class="FilterList", >
                    <li class="FilterHeader", >{ "Albums" }</li>
                    { album_list }
                    <li class="FilterHeader", >{ "Tracks" }</li>
                    { item_list }
                </ul>
            }
        };

        let selected_tracks = self
            .items
            .iter()
            .filter(|Item { id, .. }| self.selected.contains(id))
            .collect::<Vec<_>>();

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
                        <button onclick=|_| Msg::ClearSelection, >{ "Clear playlist" }</button>
                    </div>
                    <div class="PaneDivider", />
                    <TrackList: is_fetching={ !self.fetch_tasks.is_empty() }, items={ selected_tracks }, />
                </div>
            </div>
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
