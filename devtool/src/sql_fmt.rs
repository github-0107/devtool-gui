use sqlformat::{FormatOptions, Indent, QueryParams};

#[derive(Default)]
pub struct SqlFormater<'a> {
    sql: &'a str,
    space: u8,
    uppercase: bool,
}

impl<'a> SqlFormater<'a> {
    pub fn with(mut self, sql: &'a str, space: u8, uppercase: bool) -> Self {
        self.sql = sql;
        self.space = space;
        self.uppercase = uppercase;
        self
    }

    pub fn format(&self) -> String {
        let options = FormatOptions {
            indent: Indent::Spaces(self.space),
            uppercase: self.uppercase,
            lines_between_queries: 1,
        };
        sqlformat::format(self.sql, &QueryParams::None, options)
    }
}
