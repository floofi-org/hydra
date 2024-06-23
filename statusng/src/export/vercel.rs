pub struct Vercel<'a> {
    token: &'a str
}

impl<'a> Vercel<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
}
