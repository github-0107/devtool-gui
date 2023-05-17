#[derive(Default)]
pub struct Formatter<'a> {
    json: &'a str,
}

impl<'a> Formatter<'a> {
    
    pub fn with(mut self, json: &'a str) -> Self {
        self.json = json;
        self
    }

    pub fn pretty(&self) -> json::Result<String> {
        let jv = json::parse(self.json)?;
        Ok(jv.pretty(4))
    }
}
