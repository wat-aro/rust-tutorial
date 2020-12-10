use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_parat(&self, announcement: &str) -> &str {
        println!("Attenstion please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel.split(',').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longet_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Accouncement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
