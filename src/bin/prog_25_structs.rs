#[allow(dead_code)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Here we take parameters with the same name of the fields and still
// repeat them as in `username : username`.
fn build_user_v0(mail: String, uname: String) -> User {
    User {
        active: true,
        username: uname,
        email: mail,
        sign_in_count: 1,
    }
}

// «Field Init Shorthand»
// We can avoid repetition if the parameter and the field name match
fn build_user_v1(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Creat a new struct using «Struct Update Syntax»
// This function takes and return ownership of an user.
fn inc_sign_in_count(u : User) -> User {
  let update = User {
    sign_in_count: u.sign_in_count + 1,
    // The «..u» must come last
    ..u
  };
  // The «Struct Update Syntax» partially moves data because the struct contains
  // strings which do nome implement the Copy Trait. So, `u` cannot be used
  // after the let above. Next statement/ is a compilation error:
  // borrow of partially moved value: `u`

  // println!("{:#?}", u);

  // We can still access fields which were not moved:
  println!("Not moved: u.active = {:#?}", u.active);

  update
}

fn main() {
  let mut user1 = build_user_v0(String::from("diego@company-a.com"),
                                String::from("Diego Dias"));
  // Update email
  user1.email = String::from("diego@company-b.com");
  // Increment sign in counter
  let updated_user = inc_sign_in_count(user1);

  println!("{:#?}", updated_user);

  let user2 = build_user_v1(String::from("creuza@example.com"),
                            String::from("Creuza"),);

  println!("{:#?}", user2);

}