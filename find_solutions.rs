use std::collections::HashMap;

static ALPHABET: &str = "qxjzvfwbkgpmhdcytlnuroisea";

fn main() {
    let word1 = "fjord";
    let word2 = "vibex";
    let word3 = "freax";

    let old_words: &Vec<&str> = &vec![word1];

    let is_valid = word_has_unique_letters(&word1);
    println!("{} validity: {}", word1, is_valid);

    let uniqe_letters = words_have_unique_letters(&old_words, &word3);
    println!("{} and {} do not share letters: {}", old_words[0], word3, uniqe_letters);

    let uniqe_letters = words_have_unique_letters(&old_words, &word2);
    println!("{} and {} do not share letters: {}", old_words[0], word2, uniqe_letters);

    let first_letter = first_letter_in_word(&word3);
    println!("First letter of {} is {}", word3, first_letter);

    let first_letter = first_letter_in_word(&word2);
    println!("First letter of {} is {}", word2, first_letter);

    let removed_fjord = remove_letters_in_solution(&old_words);
    println!("Alphabet without {}: {}", old_words[0], removed_fjord);

    let removed_fjord = remove_letters_in_solution(&vec![word2]);
    println!("Alphabet without {}: {}", word2, removed_fjord);

    let removed_fjord = remove_letters_in_solution(&vec![word2, word3]);
    println!("Alphabet without {} and {}: {}", word2, word3, removed_fjord);

    let words_map = read_input_to_hashmap(&vec![word1, word2, word3]);
    for (letter, word_list) in words_map {
        println!("Words with {}:", letter);
        for word in word_list{
            println!("{}", word);
        }
    }

}

fn word_has_unique_letters(word: &str) -> bool {
    let mut result: bool = true;
    for (i, letter_1) in word.get(..word.len()-1).expect("REASON").chars().enumerate(){
        for letter_2 in word.get(i+1..).expect("REASON").chars(){
            if letter_1 == letter_2{
                result = false;
            }
        }
    }
    return result;
}

fn words_have_unique_letters(old_words: &Vec<&str>, new_word: &str) -> bool {
    let mut result = true;
    for old_word in old_words.iter(){
        for letter_1 in old_word.chars(){
            for letter_2 in new_word.chars(){
                if letter_1 == letter_2{
                    result = false;
                }
            }
        }
    }
    return result;
}

fn first_letter_in_word(word: &str) -> char {
    for letter in ALPHABET.chars(){
        for letter_from_word in word.chars() {
            if letter == letter_from_word{
                return letter;
            }
        }
    }
    return '_'
}

fn remove_letters_in_solution(solution: &Vec<&str>) -> String {
    let mut letters = String::from(ALPHABET);
    for word in solution.iter(){
        for letter in word.chars(){
            letters.retain(|c| c != letter)
        }
    }
    return letters;
}

fn read_input_to_hashmap(words: &Vec<&str>) -> HashMap<char, Vec<String>> {
    let mut words_map: HashMap<char, Vec<String>> = HashMap::new();

    for word in words.iter(){
        let letter = first_letter_in_word(&word);
        words_map.entry(letter).or_insert(Vec::new()).push(word.to_string());
    }

    return words_map;
}
