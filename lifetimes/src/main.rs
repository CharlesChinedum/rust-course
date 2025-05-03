fn next_langauge<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true
        }
    }

    languages.last().unwrap()
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_langauge(&languages, "go");

    println!("{}", result);
}
