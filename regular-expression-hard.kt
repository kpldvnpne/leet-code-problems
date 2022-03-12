class Solution {
    fun isMatch(text: String, pattern: String): Boolean {
        if (pattern.isEmpty()) {
            if (text.isEmpty()) {
                return true
            } else {
                return false
            }
        } else {
            if (text.isEmpty()) {
                if (pattern.length > 1 && pattern[1] == '*') {
                    return true
                } else {
                    return false
                }
            } else {
                val restOfText = text.substring(1)
                if (pattern.length > 1 && pattern[1] == '*') {
                    val restOfPattern = pattern.substring(2)
                    if (pattern[0] == '.') {
                        return isMatch(restOfText, pattern) || isMatch(text, restOfPattern)
                    } else {
                        if (text[0] == pattern[0]) {
                            return isMatch(restOfText, pattern) || isMatch(text, restOfPattern)
                        } else {
                            return isMatch(text, restOfPattern)
                        }
                    }
                } else {
                    val restOfPattern = pattern.substring(1)
                    if (pattern[0] == '.') {
                        return isMatch(restOfText, restOfPattern)
                    } else {
                        if (pattern[0] == text[0]) {
                            return isMatch(restOfText, restOfPattern)
                        } else {
                            return false
                        }
                    }
                }
            }
        }
    }
}