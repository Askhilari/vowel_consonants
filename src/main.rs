use std::io;

fn main() {
    println!("Enter the string you want to find vowels for!");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read");
    let s = s.trim();
    if s.chars().all(char::is_alphabetic){
        find_vowel(&s);
    }
    else {
        println!("String not valid");
    }
}

fn find_vowel(s: &str) {
    let vowel = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    let slen = s.len();
    let mut vowelcount = 0;
    let vowel_len = vowel.len();
    for i in 1..slen+1{
        let word = &s[i-1..i];
        for j in 0..vowel_len{
            if vowel[j] == word{
                println!("vowel {} found", word);
                vowelcount += 1;
            }
        }
    }
    println!("Number of vowels is {}", vowelcount);
    let consonant_count = slen - vowelcount;
    println!("Number of consonants is {}", consonant_count);
}
