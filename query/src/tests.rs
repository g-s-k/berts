#![cfg(test)]

use super::*;

#[test]
fn sort_only() -> Result<(), Error> {
    assert_eq!(
        "artist+".parse::<Query>()?,
        Query {
            keys: KeyGroup::default(),
            sort: vec![Sort {
                field: "artist".to_string(),
                ascending: true
            }]
        }
    );

    assert_eq!(
        "artist- year+".parse::<Query>()?,
        Query {
            keys: KeyGroup::default(),
            sort: vec![
                Sort {
                    field: "artist".to_string(),
                    ascending: false
                },
                Sort {
                    field: "year".to_string(),
                    ascending: true
                },
            ]
        }
    );

    Ok(())
}
