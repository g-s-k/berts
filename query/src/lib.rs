use std::str::FromStr;

mod tests;

#[derive(Debug)]
pub struct Error;

#[derive(Debug, Default, PartialEq)]
pub struct Query {
    keys: KeyGroup,
    sort: Vec<Sort>,
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
            curr_str = &curr_str[idx + 1..];
            match &curr_str[..idx] {
                "path" => new.key_type = Type::Path,
                // TODO: add regex support here
                other => new.field = Some(other.to_string()),
            }
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
