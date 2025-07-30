use std::fmt::Formatter;
use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

#[allow(dead_code)]
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            false => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

pub fn test_my_string() {
    let len1 = size_of::<MyString>();
    let len2 = size_of::<MiniString>();
    println!("Len:\n\t MyString {},\n\t MiniString {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一首简简单单的小情歌，唱出我们的附和".into();

    println!("s1: {:?},\ns2: {:?}", s1, s2);

    println!(
        "s1 {}[{} bytes, {} chars], s2 {}[{} bytes, {} chars]",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));
}
