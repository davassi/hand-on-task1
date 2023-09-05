



fn concatenate_strings(slice1 : &str, slice2 : &str) -> String {
    let mut result: String = String::new();
    result.push_str(slice1);
    result.push_str(slice2);
    result
}

fn main() -> () {
    let string1 = "First part".to_string();
    let string2 = ", and Second part".to_string();
    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("Concatenating '{}' with '{}' -> '{}'", string1, string2, concatenated_string);
}
