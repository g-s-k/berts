//! A library to read a [beets](https://github.com/beetbox/beets) music database.

#![deny(clippy::pedantic)]

#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
pub use rusqlite::Error;
#[cfg(not(target_arch = "wasm32"))]
use rusqlite::{Connection, OpenFlags};

mod tests;

macro_rules! def_sqlite_struct {
    ( $(#[$outer:meta])* $name:ident [ $( $(#[$inner:meta])* $field:ident: $typ:ty $(; $func:ident)?, )* ]
    ) => {
        $(#[$outer])*
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct $name {
            $( $(#[$inner])* pub $field: $typ ),*
        }

        #[cfg(not(target_arch = "wasm32"))]
        impl $name {
            #[allow(unused_assignments)]
            /// Bind the metadata for a single entry.
            pub fn from_row(db_row__: &::rusqlite::Row) -> Self {
                let mut field_idx__ = 0;

                $(
                    let $field = def_field!(db_row__.get(field_idx__) $(, $func)?);
                    field_idx__ += 1;
                )*

                Self {
                    $( $field ),*
                }
            }
        }
    };

    ( $(#[$outer:meta])* $name:ident $table:ident $fields:tt ) => {
        def_sqlite_struct! {
            $(#[$outer])*
            $name $fields
        }

        def_sqlite_struct!{
            $name stringify!($table)
        }
    };

    ( $name:ident $table:expr ) => {
        #[cfg(not(target_arch = "wasm32"))]
        impl $name {
            #[doc = "Bind each of the entries in the `"]
            #[doc = $table]
            #[doc = "` table."]
            pub fn read_all(c: &::rusqlite::Connection) ->
                ::std::result::Result<::std::vec::Vec<Self>, ::rusqlite::Error>
            {
                let mut stmt = c.prepare(concat!("SELECT * FROM ", $table))?;
                let rows = stmt.query_map(::rusqlite::NO_PARAMS, Self::from_row)?;

                let mut v = ::std::vec::Vec::new();
                for row in rows {
                    v.push(row?);
                }

                Ok(v)
            }
        }
    };
}

macro_rules! def_field {
    ( $defn:expr, $func:ident ) => {
        $func($defn)
    };
    ( $defn:expr ) => {
        $defn
    };
}

#[allow(clippy::needless_pass_by_value)]
fn blob_to_path(v: Vec<u8>) -> PathBuf {
    String::from(String::from_utf8_lossy(&v)).into()
}

fn optional_blob_to_path(v: Option<Vec<u8>>) -> Option<PathBuf> {
    v.map(blob_to_path)
}

fn is_num_zero<T: Default + PartialEq>(n: &T) -> bool {
    n == &T::default()
}

def_sqlite_struct! {
    /// All of the fields present on an "attribute" in the beets schema.
    Attribute [
        id: u32,
        entity_id: u32,
        key: String,
        value: String,
    ]
}

def_sqlite_struct! {
    /// All of the fields that an album has in the beets schema.
    Album albums [
        id: u32,
        /// This is converted lossily - any invalid UTF-8 will be
        /// [transcribed as the replacement character.](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)
        artpath: Option<PathBuf>; optional_blob_to_path,
        #[serde(skip)]
        added: f64,
        albumartist: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumartist_sort: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumartist_credit: String,
        album: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        genre: String,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        year: u16,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        month: u8,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        day: u8,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        disctotal: u32,
        comp: bool,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_albumid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_albumartistid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumtype: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        label: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_releasegroupid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        asin: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        catalognum: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        script: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        language: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        country: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumstatus: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumdisambig: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        rg_album_gain: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        rg_album_peak: Option<f64>,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        r128_album_gain: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        original_year: u16,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        original_month: u8,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        original_day: u8,
 ]
}

def_sqlite_struct! {
    /// All of the fields that an "item" (track) has in the beets schema.
    Item items [
        id: u32,
        /// This is converted lossily - any invalid UTF-8 will be
        /// [transcribed as the replacement character.](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)
        path: PathBuf; blob_to_path,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        album_id: Option<u32>,
        title: String,
        artist: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        artist_sort: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        artist_credit: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        album: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumartist: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumartist_sort: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumartist_credit: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        genre: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        lyricist: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        composer: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        composer_sort: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        arranger: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        grouping: String,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        year: u16,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        month: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        day: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        track: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        tracktotal: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        disc: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        disctotal: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        lyrics: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        comments: String,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        bpm: u32,
        comp: bool,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_trackid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_albumid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_artistid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_albumartistid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_releasetrackid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumtype: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        label: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        acoustid_fingerprint: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        acoustid_id: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        mb_releasegroupid: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        asin: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        catalognum: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        script: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        language: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        country: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumstatus: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        media: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        albumdisambig: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        disctitle: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        encoder: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        rg_track_gain: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        rg_track_peak: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        rg_album_gain: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        rg_album_peak: Option<f64>,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        r128_track_gain: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        r128_album_gain: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        original_year: u16,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        original_month: u8,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        original_day: u8,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        initial_key: Option<String>,
        length: f64,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        bitrate: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        format: String,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        samplerate: u32,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        bitdepth: u16,
        #[serde(skip_serializing_if = "is_num_zero", default)]
        channels: u8,
        #[serde(skip, default)]
        mtime: f64,
        #[serde(skip, default)]
        added: f64,
    ]
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_all(db_path: PathBuf) -> Result<(Vec<Album>, Vec<Item>), Error> {
    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    Ok((Album::read_all(&conn)?, Item::read_all(&conn)?))
}
