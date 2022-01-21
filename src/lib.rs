use crate::data::{Region, RegionInfo};
use crate::error::{Error, Result};
use reqwest::{Client, IntoUrl, Url};

pub mod data;
pub mod error;

#[derive(Clone)]
pub struct TubeDBClient {
    client: Client,
    base_url: Url,
}

impl TubeDBClient {
    pub fn new(base_url: impl IntoUrl) -> Result<TubeDBClient> {
        Ok(TubeDBClient {
            client: Client::new(),
            base_url: base_url.into_url().map_err(|_| Error::InvalidUrl)?,
        })
    }

    pub async fn region_list(&self) -> Result<Vec<Region>> {
        let url = self
            .base_url
            .join("/tsdb/region_list")
            .expect("Must be valid");
        let result = self.client.get(url).send().await?.text().await?;
        Region::parse_list(&result)
    }

    pub async fn region_json(&self, region: impl AsRef<str>) -> Result<RegionInfo> {
        let url = self
            .base_url
            .join("/tsdb/region.json")
            .expect("Must be valid");
        let params = [("region", region.as_ref())];
        let result = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await?
            .text()
            .await?;

        if "region not found" == &result {
            Err(Error::InvalidRegion(region.as_ref().to_string()))
        } else {
            Ok(serde_json::from_str::<RegionInfo>(&result)?)
        }
    }
}

// TODO: Mock a tube-db instance
// #[cfg(test)]
// mod tests {
//     use crate::TubeDBClient;
//
//     #[tokio::test]
//     async fn region_list() {
//         let client = TubeDBClient::new("http://localhost:8080").unwrap();
//         let result = client.region_list().await;
//         assert!(result.is_ok());
//         println!("Result: {:?}", result.unwrap());
//     }
//
//     #[tokio::test]
//     async fn region_json_ok() {
//         let client = TubeDBClient::new("http://localhost:8080").unwrap();
//         let result = client.region_json("nature40").await;
//         assert!(result.is_ok());
//         println!("Result: {:?}", result.unwrap());
//     }
//
//     #[tokio::test]
//     async fn region_json_illegal_region() {
//         let client = TubeDBClient::new("http://localhost:8080").unwrap();
//         let result = client.region_json("ASDF").await;
//         assert!(result.is_err());
//         if let Err(crate::error::Error::InvalidRegion(..)) = result {
//         } else {
//             panic!("Invalid region not detected")
//         }
//     }
// }
