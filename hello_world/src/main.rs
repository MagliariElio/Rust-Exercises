//use std::ops::Add;

/// @author: MagliariElio

fn main() {
    let s1 = "Hello";                         // static string
    let s2 = "World!";                        // static string
    let s = format!("{} {}", s1, s2);        // ! means that it returns an unit type,
                                                    // it's similar to void in other languages
    println!("\n{}\n", s);          // s represents the concatenation of s1 and s2

    another_way_of_concatenation();
}


fn another_way_of_concatenation() {
    let mut s1:String = String::new();              // dynamic string
    s1.push_str("My");
    let s2:String = String::from("name");        // dynamic string
    let s3:String = "is".to_string();
    let s4:&str = "Elio";

    // s2 = "Oile".to_string();    // this is an error because s2 is not mutable

    // with & I'm taking the reference to the string in read mode
    let s:String = s1 + " " + &s2 + " " + &s3 + " " + s4;
    println!(" - {}", s);

    // trying to replace the string s1
    s1 = "I am ".to_string();
    let s8 = s1 + s4;
    // let s8 = s1.add(s4);
    println!(" - {}\n", s8);

}