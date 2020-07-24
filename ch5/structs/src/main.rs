fn main() {
   // defining and using structs
   
   let mut user1 = User {
       email: String::from("someone@example.com"),
       username: String::from("someusername123"),
       active: true,
       sign_in_count: 1,
   };

   user1.email = String::from("anotheremail@example.com");

   // struct update syntax:
   let user2 = User {
       email: String::from("another@example.com"),
       username: String::from("anotherusername567"),
       ..user1 // tells compiler to get remaining fields from user1
   };

   // tuple struct
   let black = Color(0, 0, 0);
   let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// note in the above code that we stored strings as the 'owned' 
// String type, and not the borrowed-type '&str' - this is because
// we want the struct to have ownership over its attributes. 
// The below code fails to run because it introduces the possibility
// for the struct to have a dangling reference - without using 
// the lifetimes (chapter 10)

struct User2 {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
