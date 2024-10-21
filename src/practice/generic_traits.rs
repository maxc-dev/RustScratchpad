/*
    https://www.youtube.com/watch?v=T0Xfltu4h3A
    https://www.youtube.com/watch?v=m_phdVlkr6U
 */

pub fn main() {
    let dog = Dog::new("Dog");
    let cat = Cat::new("Cat");

    print_animal_dyn(&dog);
    print_animal_dyn(&cat);

    //print_animal_and_pets_where(dog);
    //print_animal_and_pets_where(cat);

    // create a vector of trait objects
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];
    for animal in animals {
        print_animal_dyn(animal);
    }
}

trait Animal {
    fn speak(&self) -> String;
    fn eat(&self) -> String;
    fn legs(&self) -> u8 {
        4
    }
}

trait Pet {
    fn play(&self) -> String;
}

struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Dog {
        Dog {
            name: name.to_string(),
        }
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says woof", self.name)
    }

    fn eat(&self) -> String {
        format!("{} is eating", self.name)
    }
}

impl Pet for Dog {
    fn play(&self) -> String {
        format!("{} is playing fetch", self.name)
    }
}

struct Cat {
    name: String,
}

impl Cat {
    fn new(name: &str) -> Cat {
        Cat {
            name: name.to_string(),
        }
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says meow", self.name)
    }

    fn eat(&self) -> String {
        format!("{} is eating", self.name)
    }

    fn legs(&self) -> u8 {
        4
    }
}

impl Pet for Cat {
    fn play(&self) -> String {
        format!("{} is playing with a ball of yarn", self.name)
    }
}

/*
    This type of generic function is called a monomorphization because it generates a specific
     implementation of the function for each type that is used with it.
     Under the hood, Rust creates a new version of the function for each type that is used with it.
      -> Such as print_animal::<Dog>(dog) and print_animal::<Cat>(cat)
       - Sometimes known as static dispatch or compile-time polymorphism

     Good for performance because it doesn't use dynamic dispatch
     But it increases the size of the binary, and it can be a problem if you have a lot of types
 */
fn print_animal<T: Animal>(animal: T) {
    println!("{}", animal.speak());
    println!("{}", animal.eat());
    println!("Has {} legs", animal.legs());
}

/*
    This type of generic function is called dynamic dispatch because it uses trait objects.
    It is slower than monomorphization because it uses a pointer to the trait object.

    But it is more flexible because it can accept any type that implements the trait.
    Good for when the types are unknown at compile time
 */
fn print_animal_dyn(animal: &dyn Animal) {
    println!("{}", animal.speak());
    println!("{}", animal.eat());
    println!("Has {} legs", animal.legs());
}

/*
    This function uses multiple traits
 */
fn print_animal_and_pets<T: Animal + Pet>(animal: T) {
    println!("{}", animal.speak());
    println!("{}", animal.play());
}

/*
    This function uses a where clause to specify the traits
 */
fn print_animal_and_pets_where<T>(animal: T)
where
    T: Animal + Pet,
{
    println!("{}", animal.speak());
    println!("{}", animal.play());
}

