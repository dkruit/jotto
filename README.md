## Two Implementation to find solutions for the Jotto problem, as covered by Stand-up Maths on [YouTube](https://www.youtube.com/watch?v=c33AZBnRHks&t=1506s).

The problem is to find 5 five-letter words, which have 25 unique letters from a list of words. There is a Python, Rust and Kotlin implementation, which follow the same algorithm.

## Quickstart
1. Place a text file with words in the project directory. Make sure the `INPUTFILE` constant at the top of the scripts matches the filename. The words list from the video can be found [here](https://github.com/dwyl/english-words/blob/master/words_alpha.txt).

To run the Python script, which will typically take a few minutes:

2. Install tqdm: `pip install tqdm`
3. Execute the script: `python3 find_solutions.py`
4. The results can be found in the file `solutions.csv`

To run the Rust script, which will typically take around half a minute:

2. Install rust: `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
3. If necessary restart the shell or add Cargo to you PATH: `source "$HOME/.cargo/env"`
4. Compile the script: `rustc -O find_solutions.rs`
5. Run the script: `./find_solutions`
6. The results can be found in the file `rust_solutions.csv`

To run the Kotlin script:

2. Install a kotlin compile (version 2.X)
3. Compile the script: `kotlinc find_solutions.kt -include-runtime -d find_solutions.jar`
4. Run the script: `java -jar find_solutions.jar`
5. The results can be found in the file `rust_solutions.csv`
