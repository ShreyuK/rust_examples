fn main() {
    let result;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        result = longest(&str1, &str2);
        println!("The longest string is: {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}