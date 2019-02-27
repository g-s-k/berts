use std::collections::HashSet;

use stdweb::{_js_impl, js};
use yew::prelude::*;

use beet_db::{Album, Item};
use beet_query::Query;

const EXAMPLE_1: &str = "foo bar baz";
const EXAMPLE_2: &str = "albumartist:EPROM";

pub enum Msg {
    Input(String),
    Clear,
    SelectAll,
    SelectAlbum(u32),
    SelectItem(u32),
}

pub struct Filter {
    query: String,
    albums: Vec<Album>,
    items: Vec<Item>,
    select_album: Option<Callback<HashSet<u32>>>,
    select_item: Option<Callback<HashSet<u32>>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props {
    pub albums: Vec<Album>,
    pub items: Vec<Item>,
    pub select_album: Option<Callback<HashSet<u32>>>,
    pub select_item: Option<Callback<HashSet<u32>>>,
}

impl Component for Filter {
    type Message = Msg;
    type Properties = Props;

    fn create(
        Props {
            albums,
            items,
            select_album,
            select_item,
        }: Self::Properties,
        _: ComponentLink<Self>,
    ) -> Self {
        Self {
            albums,
            items,
            select_album,
            select_item,
            query: String::new(),
        }
    }

    fn change(&mut self, Props { albums, items, .. }: Self::Properties) -> ShouldRender {
        let should = albums != self.albums || items != self.items;

        if should {
            self.albums = albums;
            self.items = items;
        }

        should
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clear => {
                self.query.clear();
                js! { document.getElementById("searchbar").focus() }
                true
            }
            Msg::Input(s) => {
                self.query = s;
                true
            }
            Msg::SelectAll => {
                let hs: HashSet<_> = self.filter_albums().map(|Album { id, .. }| *id).collect();
                if let Some(ref mut callback) = self.select_album {
                    callback.emit(hs);
                }

                let hs: HashSet<_> = self.filter_items().map(|Item { id, .. }| *id).collect();
                if let Some(ref mut callback) = self.select_item {
                    callback.emit(hs);
                }

                false
            }
            Msg::SelectAlbum(id) => {
                if let Some(ref mut callback) = self.select_album {
                    let mut hs = HashSet::new();
                    hs.insert(id);
                    callback.emit(hs);
                }

                false
            }
            Msg::SelectItem(id) => {
                if let Some(ref mut callback) = self.select_item {
                    let mut hs = HashSet::new();
                    hs.insert(id);
                    callback.emit(hs);
                }

                false
            }
        }
    }
}

impl Renderable<Self> for Filter {
    fn view(&self) -> Html<Self> {
        let filter_list = if self.query.is_empty() {
            html! {
                <div class="EmptyFilterList", >
                     { "No filter applied." }
                    <p>
                        { "Try typing something in above, e.g. " }
                        <code onclick=|_| Msg::Input(EXAMPLE_1.to_string()), >
                            { EXAMPLE_1 }
                        </code>
                        <br />
                        { "or " }
                        <code onclick=|_| Msg::Input(EXAMPLE_2.to_string()), >
                            { EXAMPLE_2 }
                        </code>
                    </p>
                </div>
            }
        } else {
            let mut filtered_albums = self.filter_albums().take(15).peekable();

            let album_list = if filtered_albums.peek().is_none() {
                html! { <i>{ "No matches." }</i> }
            } else {
                html! {
                    <ul class="FilterList", >
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
                    </ul>
                }
            };

            let mut filtered_items = self.filter_items().take(50).peekable();

            let item_list = if filtered_items.peek().is_none() {
                html! { <i>{ "No matches." }</i> }
            } else {
                html! {
                    <ul class="FilterList", >
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
                    </ul>
                }
            };

            html! {
                <>
                    <h5>{ "Albums" }</h5>
                    { album_list }
                    <h5>{ "Tracks" }</h5>
                    { item_list }
                </>
            }
        };

        html! {
            <div class="SideNav", >
                <div class="input", >
                    <input
                        id="searchbar",
                        placeholder="Enter query...",
                        oninput=|e| Msg::Input(e.value),
                        value=&self.query,
                    />
                    <i onclick=|_| Msg::SelectAll, title="Add all matches", >{ "+" }</i>
                    <i onclick=|_| Msg::Clear, title="Clear search", >{ "×" }</i>
                </div>
            { filter_list }
            </div>
        }
    }
}

impl Filter {
    fn filter_albums(&self) -> impl Iterator<Item = &Album> {
        let q = self.query.parse::<Query>().unwrap();

        self.albums.iter().filter(move |album| q.match_album(album))
    }

    fn filter_items(&self) -> impl Iterator<Item = &Item> {
        let q = self.query.parse::<Query>().unwrap();

        self.items.iter().filter(move |item| q.match_item(item))
    }
}
