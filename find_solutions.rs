use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;
use std::path::Path;
use std::fs::File;

static ALPHABET: &str = "qxjzvfwbkgpmhdcytlnuroisea";
static INPUTFILE: &str = "words_alpha.txt";
static OUTPUTFILE: &str = "rust_solutions.csv";

fn main() -> Result<(), Error> {
    let filepath = Path::new(INPUTFILE);
    let mut words_map: HashMap<char, Vec<String>> = HashMap::new();

    // Read file
    let file = match File::open(&filepath) {
        Err(why) => panic!("couldn't open file '{}': {}", filepath.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line?;
        if word.len() == 5 && word_has_unique_letters(&word) {
            let letter = first_letter_in_word(&word);
            words_map.entry(letter).or_insert(Vec::new()).push(word.clone());

        }
    }
    let solutions = find_solutions(&words_map);
    let n_solutions = solutions.len();
    println!("Found {} solutions", n_solutions);
    return Ok(())
}

fn word_has_unique_letters(word: &String) -> bool {
    for (i, letter_1) in word.get(..word.len()-1).expect("REASON").chars().enumerate(){
        for letter_2 in word.get(i+1..).expect("REASON").chars(){
            if letter_1 == letter_2{
                return false;
            }
        }
    }
    return true;
}

fn words_have_unique_letters(old_words: &Vec<String>, new_word: &String) -> bool {
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

fn remove_letters_in_solution(solution: &Vec<String>) -> String {
    let mut letters = String::from(ALPHABET);
    for word in solution.iter(){
        for letter in word.chars(){
            letters.retain(|c| c != letter)
        }
    }
    return letters;
}

fn find_solutions(words_map: &HashMap<char, Vec<String>>) -> Vec<Vec<String>> {
    let mut solutions: Vec<Vec<String>> = Vec::new();

    for (i_1, letter_1) in ALPHABET.get(..2).expect("REASON").chars().enumerate() {
        println!("Finding solutions with a word containing the letter {} (iteration {}/2)",
                 letter_1, i_1+1);

        for word_1 in &words_map[&letter_1] {
            let mut solution = vec!(word_1.clone());
            let mut remaining_letters = remove_letters_in_solution(&solution);

            for (i_2, letter_2) in remaining_letters.clone().get(..2).expect("Reason").chars()
                .enumerate() {
                if i_1 == 1 && i_2 == 0 { continue; }

                for word_2 in &words_map[&letter_2] {
                    solution = vec!(word_1.clone());
                    if words_have_unique_letters(&solution, word_2) {
                        solution.push(word_2.clone());
                        remaining_letters = remove_letters_in_solution(&solution);
                    }

                    for (i_3, letter_3) in remaining_letters.clone().get(..2).expect("Reason")
                        .chars().enumerate() {
                        if solution.len() < 2 { break }
                        if i_2 == 1 && i_3 == 0 { continue }

                        for word_3 in &words_map[&letter_3] {
                            solution = vec!(word_1.clone(), word_2.clone());
                            if words_have_unique_letters(&solution, word_3) {
                                solution.push(word_3.clone());
                                remaining_letters = remove_letters_in_solution(&solution);
                            }

                            for (i_4, letter_4) in remaining_letters.clone()
                                .get(..2).expect("Reason")
                                .chars().enumerate() {
                                if solution.len() < 3 { break }
                                if i_3 == 1 && i_4 == 0 { continue }

                                for word_4 in &words_map[&letter_4] {
                                    solution = vec!(word_1.clone(), word_2.clone(), word_3.clone());
                                    if words_have_unique_letters(&solution,
                                                                 word_4) {
                                        solution.push(word_4.clone());
                                        remaining_letters = remove_letters_in_solution(&solution);
                                    }

                                    for (i_5, letter_5) in remaining_letters.clone()
                                        .get(..2).expect("Reason")
                                        .chars().enumerate() {
                                        if solution.len() < 4 { break }
                                        if i_4 == 1 && i_5 == 0 { continue }

                                        for word_5 in &words_map[&letter_5] {
                                            solution = vec!(word_1.clone(),
                                                            word_2.clone(),
                                                            word_3.clone(),
                                                            word_4.clone());
                                            if words_have_unique_letters(&solution,
                                                                         word_5) {
                                                solution.push(word_5.clone());
                                                solutions.push(solution.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

    }
    return solutions
}
