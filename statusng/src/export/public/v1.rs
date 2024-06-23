use std::fs;

use serde::Serialize;

use crate::error::StatusError;
use crate::export::private::PrivateAPI;
use crate::export::vercel::Vercel;
use crate::models::service::ServiceStatus;

#[derive(Serialize, Debug)]
pub struct PublicAPIv1<'a> {
    _notice: &'a str,
    ping: f32,
    code: ServiceStatus,
    image: &'a str,
    text: &'a str,
    outages: Vec<(Option<()>, &'a str)>,
}

impl<'a> PublicAPIv1<'a> {
    pub fn from_private_api(private_api: &'a PrivateAPI) -> Self {
        let outages = private_api
            .services
            .iter()
            .filter_map(|i| match &i.status {
                ServiceStatus::Online => None,
                _ => Some((None, i.label.as_str())),
            })
            .collect();

        Self {
            _notice: "The status page API version 1 is deprecated and will be removed in a future update. Please use the API version 2. For more details, read [TBA].",
            ping: private_api.ping,
            code: private_api.global,
            image: private_api.global.as_image_string(),
            text: private_api.global.get_description(),
            outages
        }
    }

    pub fn sync(self, token: &str) -> Result<(), StatusError> {
        let data = serde_json::to_string(&self)?;
        let vercel = Vercel::new(token);

        fs::write("./out-public-v1.json", &data)?;
        vercel.put(&data, "public/api-beta.json", 360)?;

        Ok(())
    }
}
