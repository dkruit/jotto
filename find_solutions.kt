import kotlin.time.*

val ALPHABET = "qxjzvfwbkgpmhdcytlnuroisea"
val INPUTFILE = "words_alpha.txt"
val OUTPUTFILE = "solutions.csv"

fun wordHasUniqueLetters(word: String): Boolean{
    // Checks if the letters in a word are unique
    for (i in 0..<word.length-1){
        if (word.get(i) in word.subSequence(i+1, word.length)){
            return false
        }
    }
    return true
}

fun wordsHaveUniqueLetters(words: List<String>, word: String): Boolean {
    for (oldWord in words) {
        for (letter in oldWord){
            if (letter in word) {
                return false
            }
        }
    }
    return true
}

fun firstLetterInWord(word: String): Char {
    var firstLetter = '_'
    for (letter in ALPHABET) {
        if (letter in word) {
            firstLetter = letter
            break
        }
    }
    return firstLetter
}

fun removeLettersInSolutionFromAlphabet(solution: List<String>): String {
    var remaining_letters = ALPHABET
    for (word in solution) {
        for (letter in word) {
            remaining_letters = remaining_letters.replace(letter.toString(), "")
        }
    }
    return remaining_letters
}

fun readInputToMap(file: String): Map<Char, ArrayList<String>> {
    var result: MutableMap<Char, ArrayList<String>> = mutableMapOf()

    for (letter in ALPHABET) {
        var emptyList: ArrayList<String> = arrayListOf()
        result.put(letter, emptyList)
    }

    val file = java.io.File(file)
    file.bufferedReader().forEachLine {
        val word = it
        if (word.length == 5 && wordHasUniqueLetters(word)) {
            result[firstLetterInWord(word)]?.add(word)
        }
    }
    return result
}

fun findSolutions(words: Map<Char, ArrayList<String>>): List<List<String>> {
    var solutions: MutableList<List<String>> = mutableListOf()

    for (i1 in 0..1) {
        val letter1 = ALPHABET[i1]
        println("Finding solutions with a word containing the letter $letter1 (iteration ${i1 + 1}/2)")

        for (word1 in words[letter1].orEmpty()) {
            var solution = mutableListOf(word1)
            var remaining_letters = removeLettersInSolutionFromAlphabet(solution)

            for (i2 in 0..1) {
                if (i1==1 && i2==0) { continue }
                val letter2 = remaining_letters[i2]

                for (word2 in words[letter2].orEmpty()) {
                    solution = mutableListOf(word1)

                    if (wordsHaveUniqueLetters(solution, word2)) {
                        solution.add(word2)
                        remaining_letters = removeLettersInSolutionFromAlphabet(solution)
                    }

                    for (i3 in 0..1) {
                        if (solution.size != 2) { break }
                        if (i2==1 && i3==0) { continue }
                        val letter3 = remaining_letters[i3]
                        for (word3 in words[letter3].orEmpty()) {
                            solution = mutableListOf(word1, word2)

                            if (wordsHaveUniqueLetters(solution, word3)) {
                                solution.add(word3)
                                remaining_letters = removeLettersInSolutionFromAlphabet(solution)
                            }

                            for (i4 in 0..1) {
                                if (solution.size != 3) { break }
                                if (i3==1 && i4==0) { continue }
                                val letter4 = remaining_letters[i4]

                                for (word4 in words[letter4].orEmpty()) {
                                    solution = mutableListOf(word1, word2, word3)
                                    if (wordsHaveUniqueLetters(solution, word4)) {
                                        solution.add(word4)
                                        remaining_letters = removeLettersInSolutionFromAlphabet(solution)
                                    }

                                    for (i5 in 0..1) {
                                        if (solution.size != 4) { break }
                                        if (i4==1 && i5==0) { continue }
                                        val letter5 = remaining_letters[i5]

                                        for (word5 in words[letter5].orEmpty()) {
                                            solution = mutableListOf(word1, word2, word3, word4)
                                            if (wordsHaveUniqueLetters(solution, word5)) {
                                                solution.add(word5)
                                                solutions.add(solution)
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

fun saveSolutions(filePath: String, solutions: List<List<String>>) {
    val fileWriter = java.io.File(filePath).bufferedWriter()

    fileWriter.use { out ->
        for (solution in solutions) {
            val line = solution.joinToString(separator=",")
            out.write(line)
            out.newLine()
        }
    }
}

fun main(){
    val timeSource = TimeSource.Monotonic
    val t1 = timeSource.markNow()

    var wordsMap = readInputToMap(INPUTFILE)
    val solutions = findSolutions(wordsMap)
    saveSolutions(OUTPUTFILE, solutions)

    val t2 = timeSource.markNow()

    println("Found ${solutions.size} solutions, took ${t2-t1}.")

}
