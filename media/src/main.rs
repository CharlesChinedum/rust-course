#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("{} by {}", title, author),
            Media::Movie { title, director } => format!("{} by {}", title, director),
            Media::Audiobook { title } => format!("{}", title),
            Media::Podcast(episode_number) => format!("Podcast #{}", episode_number),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // Good
            MightHaveAValue::AValue(&self.items[index])
        } else {
            MightHaveAValue::NoValue
        }
    }
}

enum MightHaveAValue<'a> {
    AValue(&'a Media),
    NoValue,
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // match catalog.items.get(100) {
    //     Some(value) => {
    //         println!("Item {:#?}", value);
    //     }
    //     Option::None => {
    //         println!("Item not found");
    //     }
    // }

    let item = catalog.get_by_index(0);

    // match item {
    //     MightHaveAValue::AValue(value) => {
    //         println!("Item {:#?}", value);
    //     }
    //     MightHaveAValue::NoValue => {
    //         println!("Item not found");
    //     }
    // }

    if let MightHaveAValue::AValue(value) = item {
        println!("Item in pattern match: {:#?}", value);
    } else {
        println!("Item not found in pattern match");
    }

    // println!("Item {:#?}", item);
}
