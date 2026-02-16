fn main() {
    let word = "hello";
    let mut reversed = String::from("");
    let chars: Vec<char> = word.chars().collect();
    let mut drow: Vec<char> = word.chars().collect();
    let word_len = chars.len();

    for position in 0..word_len {
        let current_letter = word_len - position -1;
        drow[position] = chars[current_letter];
    }
    
    for letter in drow {
        reversed.push_str(&letter.to_string())
    }
    println!("{}", reversed)

}
