use md5::{Digest, Md5};

#[derive(Default)]
pub struct Encrypter {
    ostr: String,
}

impl Encrypter {
    pub fn with(mut self, ostr: String) -> Self {
        self.ostr = ostr;
        self
    }

    pub fn encrypt(&self) -> (String, String, String, String) {
        let mut hasher = Md5::new();
        hasher.update(self.ostr.as_bytes());

        let result = hasher.finalize();

        let upper32 = self.to_hex(&result[..], true);

        let lower32 = self.to_hex(&result[..], false);

        let upper16 = self.to_hex(&result[4..12], true);

        let lower16 = self.to_hex(&result[4..12], false);

        (upper32, lower32, upper16, lower16)
    }

    fn to_hex(&self, rs: &[u8], is_upper: bool) -> String {
        let mut hex = String::new();
        for x in rs {
            if is_upper {
                hex.push_str(format!("{:X}", x).as_str());
            } else {
                hex.push_str(format!("{:x}", x).as_str());
            }
            
        }
        hex
    }
}
