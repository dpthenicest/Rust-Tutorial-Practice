struct Film {
    title: String,
    director: String,
    studio: String,
}

struct Book {
    title: String,
    author: String,
    publisher: String,
}

trait Catalog {
    fn describe(&self) {
        println!("We need more information about this type of media.");
    }
}

impl Catalog for Film {
    fn describe(&self) {
        println!(
            "{} was directed by {} through {} studios",
            self.title, self.director, self.studio
        );
    }
}

impl Catalog for Book {
    fn describe(&self) {
        println!(
            "{} was written by {} and published by {}",
            self.title, self.author, self.publisher
        );
    }
}

struct Album {
    title: String,
    artist: String,
    label: String,
}

impl Catalog for Album {}

fn main() {
    let capt_marvel = Film {
        title: String::from("Captain Marvel"),
        director: String::from("Anna Boden and Ryan Fleck"),
        studio: String::from("Marvel"),
    };

    capt_marvel.describe();

    let elantris = Book {
        title: String::from("Elantris"),
        author: String::from("Brandon Sanderson"),
        publisher: String::from("Tor Bools")
    };

    elantris.describe();

    let let_it_be = Album {
        title: String::from("Let it be"),
        artist: String::from("Beatles"),
        label: String::from("Apple")
    };

    let_it_be.describe();
}

/*
   Object-Oriented Progamming in Rust:
   1. Data is stored in Enums or Structs.
   2. Behaviour is stored seperately in Traits.

   Trait:
   - Stores behaviour.
   - Can define default behaviour for a method.

   Implementing a Trait:
   - Use the impl and for keywords.

   Using a Trait:
   - Create an instance of a struct.
   - Call a trait method on the instance.
*/
