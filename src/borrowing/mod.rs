use crate::utils::print::print_start;

pub fn run() {
    print_start("Borrowing");
    works();
    still_works_covariance();
}

fn works() {
    let a = "Allo";
    let d;
    {
        let b = "bonjour";
        let c = lifetime(&a, &b);

        // Here's the magic to lifetimes, c lives as long as a lives
        // So there's no use-after-free because a is in the parent scope
        // <3 this language
        d = c;
    }

    println!("{}", d); // This actually works because C's lifetime === a's lifetime
}

fn still_works_covariance() {
    let a = "hello";
    let d;
    {
        let b = "hi";
        let c = lifetime(&b, &a);
        d = c;
    }

    // Now this is a tricky, yet fun one. I asked a question about this on
    // stackoverflow (https://stackoverflow.com/questions/60822978/rust-about-lifetimes#60822978)
    // This is a case of contravariance, where the longest-common-lived lifetime of both str is
    // **Drum rolls** 'static ! :D
    println!("{}", d);
}

fn lifetime<'a, 'b>(test: &'a str, test2: &'b str) -> &'a str {
    println!("{}, {}", test, test2);
    return "returned value of lifetime";
}