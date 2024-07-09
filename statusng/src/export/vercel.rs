use crate::error::StatusError;

pub struct Vercel<'a> {
    token: &'a str,
}

impl<'a> Vercel<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }

    pub fn put(&self, data: &[u8], path: &str, max_age: u32) -> Result<(), StatusError> {
        let url = format!("https://blob.vercel-storage.com/{path}");
        let authorization = format!("Bearer {}", self.token);

        ureq::put(&url)
            .set("Access", "public")
            .set("Authorization", &authorization)
            .set("X-API-Version", "7")
            .set("X-Content-Type", "application/json")
            .set("X-Cache-Control-Max-Age", &max_age.to_string())
            .set("X-Add-Random-Suffix", "0")
            .send_bytes(data)?;

        Ok(())
    }
}
