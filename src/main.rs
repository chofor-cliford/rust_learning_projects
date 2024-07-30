fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    // let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

     // Initialize counts for each vowel
    let mut count_a = 0;
    let mut count_e = 0;
    let mut count_i = 0;
    let mut count_o = 0;
    let mut count_u = 0;

    // Iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' => count_a += 1,
            'e' => count_e += 1,
            'i' => count_i += 1,
            'o' => count_o += 1,
            'u' => count_u += 1,
            _ => continue,
        }
    }

    // Print the count for each vowel
    println!("Count of 'a': {}", count_a);
    println!("Count of 'e': {}", count_e);
    println!("Count of 'i': {}", count_i);
    println!("Count of 'o': {}", count_o);
    println!("Count of 'u': {}", count_u);

     // Function to find the longest word in a sentence
    fn longest_word(sentence: &str) -> &str {
        sentence.split_whitespace()
            .max_by_key(|word| word.len())
            .unwrap()
    }

    // Invoke the function with the sentence variable
    let longest = longest_word(&sentence);
    println!("The longest word is: {}", longest);

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').rev().collect::<Vec<&str>>();
    println!("{:?}", words);

    // let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);
    for word in words {
        println!("{}", word);
    }
}
