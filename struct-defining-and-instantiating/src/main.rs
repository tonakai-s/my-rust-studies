fn main() {
    //The fields don't need to follow the same order.
    let mut user1: User = User {
        active: true,
        username: String::from("renas"),
        email: String::from("renas@gmail.com"),
        sing_in_count: 1,
    };

    //Note wich to change the value, the struct instance need to be muttable.
    user1.email = String::from("newrenas@gmail.com");

    //Creating a new User instance using a factory function.
    let _user_created_with_factory: User = user_factory(
        String::from("renasfact@gmail.com"),
        String::from("renasfact"),
    );

    let _user_with_update_syntax = User {
        username: String::from("uptrenas"),
        email: String::from("uptrenas@gmail.com"),
        //IMPORTANT!!!
        //This inform Rust to create the rest of the fields with the same value
        //contained on the user1 instance.
        //Because their values are primitive types, it's implement the Copy, instead Move.
        //But if we do with the types that implement Move, user1 is no longer available to user.
        ..user1
    };
}

#[allow(dead_code)]
//Structure is formed by fields with name and their type.
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn user_factory(email: String, username: String) -> User {
    User {
        active: true,
        //Because username and email fields have the same name of the parameters.
        //we don't need to repeat their.
        username,
        email,
        sing_in_count: 1,
    }
}
