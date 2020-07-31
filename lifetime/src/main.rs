/* fn main() {
    let string1 = String::from("abcd");
    let result;

    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let v = vec![1, 2, 3, 4, 5];

    v.iter().map(|x| 2 * x).for_each(|i| println!("{}", i));
}
 */
use std::fmt::Display;

fn longest_with_an_accouncement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

fn longest<'a>(_x: &str, _y: &str) -> String {
    let result = String::from("really long string");
    result
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attension pleas: {}", accouncement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1]
        }
    }

    &s[..]
}