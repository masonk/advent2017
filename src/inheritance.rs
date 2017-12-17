fn main () {
    struct Animal {
        lifespan: u32
    }

    struct Mammal {
        claws: bool
    }

    struct Human {
        terrible: bool
    }

    trait Mobile {

    }

    trait Starve : Mortal {
        fn starve(&self) {
            println!("I'm starving!!!.... {}", self.death_scream());
        }
    }

    trait Mortal {
       fn death_scream(&self) ->  &'static str {
            "ahEAahh"
       }
    }

    impl Mortal for Mammal {
        fn death_scream(&self) -> &'static str {
            "ahEAahh"
        }
    }

    impl Mortal for Human {}
    impl Starve for Human {}

    let bob = Human { terrible: true };
    bob.starve();
}