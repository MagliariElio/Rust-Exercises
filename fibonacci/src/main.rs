use rand::Rng;

/// @author: MagliariElio

fn main() {
    let mut list:[u32;10] = [0; 10];
    let mut rng = rand::thread_rng();

    // the list is only used for educational purpose
    for i in 1..10 {
        list[i] = rng.gen_range(10..17);
        println!(" - {} = {}", list[i], fibonacci(list[i]));
    }
}


fn fibonacci(a: u32) -> u32 {
    return if a <= 2 {
        1
    } else {
        fibonacci(a - 1) + fibonacci(a - 2)
    }
}