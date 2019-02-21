use std::str::FromStr;

use beet_db::Item;

mod tests;

#[derive(Debug)]
pub struct Error;

#[derive(Debug, Default, PartialEq)]
pub struct Query {
    keys: KeyGroup,
    sort: Vec<Sort>,
}

impl Query {
    pub fn match_item(&self, item: &Item) -> bool {
        self.keys.match_item(item)
    }
}

impl FromStr for Query {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new = Self::default();

        for token in s.split(' ') {
            if token.ends_with('+') || token.ends_with('-') {
                match token.parse::<Sort>() {
                    Ok(sort) => new.sort.push(sort),
                    Err(err) => return Err(err),
                }
            } else {
                match token.parse::<Keyword>() {
                    Ok(key) => new.keys.keys.push(key),
                    Err(err) => return Err(err),
                }
            }
        }

        Ok(new)
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Sort {
    field: String,
    ascending: bool,
}

impl FromStr for Sort {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field = s[..s.len() - 1].to_string();
        let ascending = s.ends_with('+');
        Ok(Self { field, ascending })
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct KeyGroup {
    keys: Vec<Keyword>,
    all: bool,
}

impl KeyGroup {
    fn match_item(&self, item: &Item) -> bool {
        let f = |key: &Keyword| key.match_item(item);

        if self.all {
            self.keys.iter().all(f)
        } else {
            self.keys.iter().any(f)
        }
    }
}

impl Default for KeyGroup {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            all: true,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Keyword {
    text: String,
    field: Option<String>,
    key_type: Type,
    negated: bool,
}

impl Keyword {
    fn match_item(&self, item: &Item) -> bool {
        let year = format!("{}", item.year);
        let month = format!("{}", item.month);
        let day = format!("{}", item.day);
        let track = format!("{}", item.track);
        let tracktotal = format!("{}", item.tracktotal);
        let disc = format!("{}", item.disc);
        let disctotal = format!("{}", item.disctotal);
        let bitrate = format!("{}", item.bitrate);

        let txt = match self.field.as_ref().map(String::as_str) {
            Some("title") => vec![&item.title],
            Some("album") => vec![&item.album],
            Some("artist") => vec![&item.artist, &item.artist_sort, &item.artist_credit],
            Some("albumartist") => vec![
                &item.albumartist,
                &item.albumartist_sort,
                &item.albumartist_credit,
            ],
            Some("genre") => vec![&item.genre],
            Some("lyricist") => vec![&item.lyricist],
            Some("composer") => vec![&item.composer, &item.composer_sort],
            Some("arranger") => vec![&item.arranger],
            Some("grouping") => vec![&item.grouping],
            Some("year") => vec![&year],
            Some("month") => vec![&month],
            Some("day") => vec![&day],
            Some("track") => vec![&track],
            Some("tracktotal") => vec![&tracktotal],
            Some("disc") => vec![&disc],
            Some("disctotal") => vec![&disctotal],
            Some("catalognum") => vec![&item.catalognum],
            Some("format") => vec![&item.format],
            Some("bitrate") => vec![&bitrate],
            None => vec![
                &item.title,
                &item.album,
                &item.artist,
                &item.artist_sort,
                &item.artist_credit,
                &item.albumartist,
                &item.albumartist_sort,
                &item.albumartist_credit,
                &item.genre,
                &item.comments,
            ],
            _ => vec![],
        };

        self.negated != match self.key_type {
            Type::Basic => {
                let lower = self.text.to_lowercase();
                txt.iter().any(|s| s.to_lowercase().contains(&lower))
            }
            Type::Path => unimplemented!(),
            // _ => unreachable!(),
        }
    }
}

impl FromStr for Keyword {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new = Self::default();
        let mut curr_str = s.trim();

        if curr_str.starts_with('^') || curr_str.starts_with('-') {
            new.negated = true;
            curr_str = &curr_str[1..];
        }

        if let Some(idx) = curr_str.find(':') {
            match &curr_str[..idx] {
                "path" => new.key_type = Type::Path,
                // TODO: add regex support here
                other => new.field = Some(other.to_string()),
            }
            curr_str = &curr_str[idx + 1..];
        }

        // TODO: add num and date range support here
        new.text = curr_str.to_string();

        Ok(new)
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    Basic,
    Path,
    // Regex,
    // NumRange,
    // DateRange,
}

impl Default for Type {
    fn default() -> Self {
        Type::Basic
    }
}
