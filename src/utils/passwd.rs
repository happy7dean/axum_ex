use uuid::Uuid;
use sha2::{Sha512, Digest};
use base64ct::{
    Base64,
    Encoding,
};

pub struct Passwd {
    pub wd: String,
    pub salt: String
}

impl Passwd {
    fn new () -> Self {
        Passwd {
            wd: "".into(),
            salt: "".into(),
        }
    }
}

pub fn passwd_hash (psswd:&String) -> Passwd {
    let mut passwd = Passwd::new();
    passwd.salt = Uuid::new_v4().to_string();
    let mut hasher = Sha512::new();
    hasher.update(psswd);
    hasher.update(&passwd.salt);
    let hash = hasher.finalize();
    passwd.wd = Base64::encode_string(&hash);
    return passwd;
}
