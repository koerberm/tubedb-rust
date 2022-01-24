use crate::data::{AuthInfo, Region, RegionInfo};
use crate::error::{Error, Result};
use reqwest::{Client, IntoUrl, RequestBuilder, Response, Url};

pub mod data;
pub mod error;

#[derive(Clone)]
pub struct TubeDBClient {
    client: Client,
    auth_info: AuthInfo,
    base_url: Url,
}

impl TubeDBClient {
    pub fn new(base_url: impl IntoUrl, auth_info: AuthInfo) -> Result<TubeDBClient> {
        Ok(TubeDBClient {
            client: Client::new(),
            auth_info,
            base_url: base_url.into_url().map_err(|_| Error::InvalidUrl)?,
        })
    }

    fn build_url(&self, path: &str) -> Url {
        self.base_url.join(path).expect("Invalid request path")
    }

    async fn send_request(&self, request: RequestBuilder) -> Result<Response> {
        Ok(self.auth_info.send_request(request).await?)
    }

    pub async fn region_list(&self) -> Result<Vec<Region>> {
        let request = self.client.get(self.build_url("/tsdb/region_list"));
        let result = self.send_request(request).await?.text().await?;
        Region::parse_list(&result)
    }

    pub async fn region_json(&self, region: impl AsRef<str>) -> Result<RegionInfo> {
        let params = [("region", region.as_ref())];
        let request = self.client.get(self.build_url("/tsdb/region.json")).query(&params);
        let result = self.send_request(request)
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
//     use crate::data::{AuthInfo};
//     use crate::TubeDBClient;
//
//     const SERVER_URL: &str = "http://vhrz1078.hrz.uni-marburg.de:8100";
//     const USERNAME: &str = "user";
//     const PASSWORD: &str = "pass";
//
//     #[tokio::test]
//     async fn region_list() {
//         let ai = AuthInfo::Digest{username: USERNAME.to_string(),  password: PASSWORD.to_string()};
//         let client = TubeDBClient::new(SERVER_URL, ai).unwrap();
//         let result = client.region_list().await;
//         assert!(result.is_ok());
//         println!("Result: {:?}", result.unwrap());
//     }
//
//     #[tokio::test]
//     async fn region_json_ok() {
//         let ai = AuthInfo::Digest{username: USERNAME.to_string(),  password: PASSWORD.to_string()};
//         let client = TubeDBClient::new(SERVER_URL, ai).unwrap();
//         let result = client.region_json("nature40").await;
//         assert!(result.is_ok());
//         println!("Result: {:?}", result.unwrap());
//     }
//
//     #[tokio::test]
//     async fn region_json_illegal_region() {
//         let ai = AuthInfo::Digest{username: USERNAME.to_string(),  password: PASSWORD.to_string()};
//         let client = TubeDBClient::new(SERVER_URL, ai).unwrap();
//         let result = client.region_json("ASDF").await;
//         assert!(result.is_err());
//         if let Err(crate::error::Error::InvalidRegion(..)) = result {
//         } else {
//             panic!("Invalid region not detected")
//         }
//     }
// }
