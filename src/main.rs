fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from(" World!");

    let concatenate_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenate_string);
}

fn concatenate_strings(first_slice: &str, second_slice: &str) -> String {
    let mut result = String::new();
    result.push_str(first_slice);
    result.push_str(second_slice);
    result
}