use stdweb::{_js_impl, js};
use yew::prelude::*;

use beet_db::{Album, Item};
use beet_query::Query;

pub enum Msg {
    Input(String),
    Clear,
    SelectAlbum(u32),
    SelectItem(u32),
}

pub struct Filter {
    query: String,
    albums: Vec<Album>,
    items: Vec<Item>,
    select_album: Option<Callback<u32>>,
    select_item: Option<Callback<u32>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props {
    pub albums: Vec<Album>,
    pub items: Vec<Item>,
    pub select_album: Option<Callback<u32>>,
    pub select_item: Option<Callback<u32>>,
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
        let mut should = false;

        if albums != self.albums {
            self.albums = albums;
            should = true;
        }

        if items != self.items {
            self.items = items;
            should = true;
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
            Msg::SelectAlbum(id) => {
                if let Some(ref mut callback) = self.select_album {
                    callback.emit(id);
                }

                false
            }
            Msg::SelectItem(id) => {
                if let Some(ref mut callback) = self.select_item {
                    callback.emit(id);
                }

                false
            }
        }
    }
}

impl Renderable<Self> for Filter {
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

        html! {
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
        }
    }
}
