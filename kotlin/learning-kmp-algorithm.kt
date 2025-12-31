fun main() {
    print(findMatchLocations("AAAAABAAAABA", "AAAA"))
}

fun findMatchLocations(text: String, pattern: String): List<Int> {
    if (pattern.length == 0) {
        return (0 until text.length).toList()
    }
    if (pattern.length > text.length) {
        return emptyList()
    }

    val lps = calculateLps(pattern)

    val matchLocations = mutableListOf<Int>()
    var textIndex = 0
    var patternIndex = 0

    while (textIndex < text.length) {
        if (patternIndex >= pattern.length || text[textIndex] != pattern[patternIndex]) {
            if (patternIndex == 0) {
                textIndex++
                continue
            }
            patternIndex = lps[patternIndex - 1]
        } else if (text[textIndex] == pattern[patternIndex]) {
            textIndex++
            patternIndex++
        }

        // kept here because textIndex might have overflowed
        if (patternIndex >= pattern.length) {
            matchLocations.add(textIndex - patternIndex)
        }
    }

    return matchLocations
}

fun calculateLps(text: String): Array<Int> {
    val size = text.length
    if (size == 0) return emptyArray()
    val lps = Array(size) { 0 }
    lps[0] = 0
    var len = 0
    var x = 1
    while (x < size) {
        if (text[x] == text[len]) {
            len++
            lps[x] = len
            x++
        } else {
            if (len > 0) {
                len = lps[len - 1]
            } else {
                lps[x] = 0
            }
        }
    }

    return lps
}
