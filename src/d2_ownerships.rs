pub fn ownerships() {
     let mut s = String::from("Hello");
 
     // Pass the reference of `s` to the `change_string` function
     change_string(&mut s);
 
     println!("The string is now: {}", s);
 }
 
 // Define a function that takes a mutable reference to a String
pub fn change_string(some_string: &mut String) {
    // Modify the String referenced by `some_string`
    some_string.push_str(", World");
}

