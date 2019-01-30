macro_rules! def_sqlite_struct {
    ( $name:ident [ $( $field:ident: $typ:ty, )* ] ) => {
        #[derive(Debug)]
        pub struct $name {
            $( pub $field: $typ ),*
        }

        impl $name {
            pub fn from_row(db_row__: &::rusqlite::Row) -> Self {
                let mut field_idx__ = 0;

                $(
                    let $field = db_row__.get(field_idx__);
                    field_idx__ += 1;
                )*

                drop(field_idx__);

                Self {
                    $( $field ),*
                }
            }
        }
    };
    ( $name:ident $table:ident $fields:tt ) => {
        def_sqlite_struct! {
            $name $fields
        }

        impl $name {
            pub fn read_all(c: ::rusqlite::Connection) ->
                ::std::result::Result<::std::vec::Vec<Self>, ::rusqlite::Error>
            {
                let mut stmt = c.prepare(concat!("SELECT * FROM ", stringify!($table)))?;
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

// Is this needed?
def_sqlite_struct! {
    Attribute [
        id: u32,
        entity_id: u32,
        key: String,
        value: String,
    ]
}

def_sqlite_struct! {
    Album albums [
        id: u32,
        artpath: String,
        added: f64,
        albumartist: String,
        albumartist_sort: String,
        albumartist_credit: String,
        album: String,
        genre: String,
        year: u16,
        month: u8,
        day: u8,
        disctotal: u32,
        comp: bool,
        mb_albumid: String,
        mb_albumartistid: String,
        albumtype: String,
        label: String,
        mb_releasegroupid: String,
        asin: String,
        catalognum: String,
        script: String,
        language: String,
        country: String,
        albumstatus: String,
        albumdisambig: String,
        rg_album_gain: f64,
        rg_album_peak: f64,
        r128_album_gain: u32,
        original_year: u16,
        original_month: u8,
        original_day: u8,
 ]
}

def_sqlite_struct! {
    Item items [
        id: u32,
        path: String,
        album_id: u32,
        title: String,
        artist: String,
        artist_sort: String,
        artist_credit: String,
        album: String,
        albumartist: String,
        albumartist_sort: String,
        albumartist_credit: String,
        genre: String,
        lyricist: String,
        composer: String,
        composer_sort: String,
        arranger: String,
        grouping: String,
        year: u16,
        month: u32,
        day: u32,
        track: u32,
        tracktotal: u32,
        disc: u32,
        disctotal: u32,
        lyrics: String,
        comments: String,
        bpm: u32,
        comp: bool,
        mb_trackid: String,
        mb_albumid: String,
        mb_artistid: String,
        mb_albumartistid: String,
        mb_releasetrackid: String,
        albumtype: String,
        label: String,
        acoustid_fingerprint: String,
        acoustid_id: String,
        mb_releasegroupid: String,
        asin: String,
        catalognum: String,
        script: String,
        language: String,
        country: String,
        albumstatus: String,
        media: String,
        albumdisambig: String,
        disctitle: String,
        encoder: String,
        rg_track_gain: f64,
        rg_track_peak: f64,
        rg_album_gain: f64,
        rg_album_peak: f64,
        r128_track_gain: u32,
        r128_album_gain: u32,
        original_year: u16,
        original_month: u8,
        original_day: u8,
        initial_key: String,
        length: f64,
        bitrate: u32,
        format: String,
        samplerate: u16,
        bitdepth: u16,
        channels: u8,
        mtime: f64,
        added: f64,
    ]
}
