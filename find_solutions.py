from typing import List
from tqdm import tqdm
from time import perf_counter
import csv

ALPHABET = ["q", "x", "j", "z", "v", "f", "w", "b", "k", "g", "p", "m", "h",
            "d", "c", "y", "t", "l", "n", "u", "r", "o", "i", "s", "e", "a"]
INPUTFILE = "words_alpha.txt"
OUTPUTFILE = "solutions.csv"


def word_has_unique_letters(word: str):
    """Checks if a word contains unique letters"""
    for i, letter_1 in enumerate(word[:-1]):
        for letter_2 in word[i+1:]:
            if letter_1 == letter_2:
                return False
    return True


def words_have_unique_letters(words: List[str], new_word: str):
    """Checks if the letters in two words of the same length are unique."""
    for old_word in words:
        for letter_1 in old_word:
            for letter_2 in new_word:
                if letter_1 == letter_2:
                    return False
    return True


def first_letter_in_word(word: str):
    for letter in ALPHABET:
        if letter in word:
            break
    return letter


def remove_letters_in_solution(solution):
    letters = ALPHABET.copy()
    for word in solution:
        for letter in word:
            letters.remove(letter)
    return letters


def read_input_to_dict(file: str):
    """
    Reads the words from the input file.
    For each letter from ALPHABET there is a dictionary item.
    Each word is assigned to the first letter in ALHPABET it contains.
    All words which contain a q are added to "q", all words which contain an "x" but not a "q" to "x", etc..
    """
    output = {}
    for letter in ALPHABET:
        output[letter] = []

    with open(file) as f:
        for i, line in enumerate(f.readlines()):
            word = line[:-1]
            if len(word) == 5:
                if word_has_unique_letters(word):
                    output[first_letter_in_word(word)].append(word)
    return output


def find_solutions(words):
    solutions = []

    for i_1, letter_1 in enumerate(ALPHABET[:2]):
        print(f"Finding solutions with a word containing the letter {letter_1} (iteration {i_1 + 1}/2)")

        for word_1 in tqdm(words[letter_1]):
            solution = [word_1]
            remaining_letters = remove_letters_in_solution(solution)

            for i_2, letter_2 in enumerate(remaining_letters[:2]):
                if i_1 == 1 and i_2 == 0:
                    continue

                for word_2 in words[letter_2]:
                    solution = [word_1]
                    if words_have_unique_letters(solution, word_2):
                        solution = [word_1, word_2]
                        remaining_letters = remove_letters_in_solution(solution)

                    for i_3, letter_3 in enumerate(remaining_letters[:2]):
                        if len(solution) < 2:
                            break

                        if i_2 == 1 and i_3 == 0:
                            continue

                        for word_3 in words[letter_3]:
                            solution = [word_1, word_2]
                            if words_have_unique_letters(solution, word_3):
                                solution = [word_1, word_2, word_3]
                                remaining_letters = remove_letters_in_solution(solution)

                            for i_4, letter_4 in enumerate(remaining_letters[:2]):
                                if len(solution) < 3:
                                    break

                                if i_3 == 1 and i_4 == 0:
                                    continue

                                for word_4 in words[letter_4]:
                                    solution = [word_1, word_2, word_3]
                                    if words_have_unique_letters(solution, word_4):
                                        solution = [word_1, word_2, word_3, word_4]
                                        remaining_letters = remove_letters_in_solution(solution)

                                    for i_5, letter_5 in enumerate(remaining_letters[:2]):
                                        if len(solution) < 4:
                                            break

                                        if i_4 == 1 and i_5 == 0:
                                            continue

                                        for word_5 in words[letter_5]:
                                            solution = [word_1, word_2, word_3, word_4]
                                            if words_have_unique_letters(solution, word_5):
                                                solution = [word_1, word_2, word_3, word_4, word_5]
                                                solutions.append(solution)
    return solutions


def save_solutions(filename: str, solutions: List[List[str]]):
    with open(filename, "w") as f:
        writer = csv.writer(f, delimiter=',')
        for solution in solutions:
            writer.writerow(solution)


def main():
    t1 = perf_counter()
    words = read_input_to_dict(INPUTFILE)

    solutions = find_solutions(words)
    save_solutions(OUTPUTFILE, solutions)
    t2 = perf_counter()

    print(f"Found {len(solutions)} solutions, took {t2-t1:.3f} seconds.")

if __name__ == "__main__":
    main()
