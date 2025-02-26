#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
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
        director: String::from("Good Direcotr"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    println!("{:#?}", good_movie);
    println!("{:#?}", bad_book);
    println!("{:#?}", audiobook);
}
