use crate::error::{Error, Result};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

fn parse_line<'a, const N: usize>(line: &'a str) -> Result<[&'a str; N]> {
    let mut res: [&'a str; N] = [Default::default(); N];
    let mut max = 0_usize;
    for (i, elm) in line.split(';').take(N).enumerate() {
        res[i] = elm.trim();
        max += 1;
    }
    if max < N {
        Err(Error::Parse(format!("Could not parse line {}", line)))
    } else {
        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub identifier: String,
    pub name: String,
}

impl Region {
    pub(crate) fn parse_list(list: &str) -> Result<Vec<Region>> {
        list.lines()
            .filter(|&s| !s.trim().is_empty())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<Region>>>()
    }
}

impl TryFrom<&str> for Region {
    type Error = crate::error::Error;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let values = parse_line::<2>(line)?;
        Ok(Region {
            identifier: values[0].to_string(),
            name: values[1].to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct RegionInfo {
    #[serde(rename = "id")]
    pub identifier: String,
    pub name: String,
    pub view_year_range: ViewYearRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ViewYearRange {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub start: i64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub end: i64,
}

#[cfg(test)]
mod tests {
    use crate::data::{Region, RegionInfo};
    use crate::error::Error;

    #[test]
    fn test_region_ok() -> Result<(), Error> {
        let res = Region::try_from("DE;Germany")?;
        assert_eq!(
            Region {
                identifier: "DE".to_string(),
                name: "Germany".to_string(),
            },
            res
        );
        Ok(())
    }

    #[test]
    fn test_region_trim_ok() -> Result<(), Error> {
        let res = Region::try_from("DE ; Germany ")?;
        assert_eq!(
            Region {
                identifier: "DE".to_string(),
                name: "Germany".to_string(),
            },
            res
        );
        Ok(())
    }

    #[test]
    fn test_region_trim_nok() -> Result<(), Error> {
        let res = Region::try_from("DE:Germany");
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_region_list_ok() -> Result<(), Error> {
        let res = Region::parse_list("DE;Germany\nFR;France ")?;
        assert_eq!(
            vec![
                Region {
                    identifier: "DE".to_string(),
                    name: "Germany".to_string(),
                },
                Region {
                    identifier: "FR".to_string(),
                    name: "France".to_string(),
                }
            ],
            res
        );
        Ok(())
    }

    #[test]
    fn test_region_list_empty() -> Result<(), Error> {
        let res = Region::parse_list("")?;
        assert!(res.is_empty());
        Ok(())
    }

    #[test]
    fn test_region_list_empty_spaces() -> Result<(), Error> {
        let res = Region::parse_list(" ")?;
        assert!(res.is_empty());
        Ok(())
    }

    #[test]
    fn test_deserialize_region_info() -> Result<(), Error> {
        let result = serde_json::from_str::<RegionInfo>("{\"id\":\"nature40\",\"name\":\"Nature 4.0\",\"view_year_range\":{\"start\":\"2017\",\"end\":\"2021\"}}")?;
        assert_eq!(
            RegionInfo {
                identifier: "nature40".to_string(),
                name: "Nature 4.0".to_string(),
                view_year_range: super::ViewYearRange {
                    start: 2017,
                    end: 2021
                }
            },
            result
        );
        Ok(())
    }
}
