class Solution {
    fun shortestPalindrome(s: String): String {
        val length = s.length
        if (length == 0 || length == 1) {
            return s
        }
        
        val isEven = length % 2 == 0
        var (left, right) = if (isEven) {
            Pair(length / 2 - 1, length / 2)
        } else {
            Pair(length / 2, length / 2)
        }
        
        while (left >= 0) {
            var currentLeft = left
            var currentRight = right

            var canBePadded = true
            while (currentLeft >= 0) {
                val charAtLeft = s[currentLeft]
                val charAtRight = s[currentRight]
                if (charAtLeft != charAtRight) {
                    canBePadded = false
                    break
                }
                currentLeft--
                currentRight++
            }
            if (canBePadded) {
                val toBePadded = s.substring(currentRight).reversed()
                return toBePadded + s
            } else {
                if (left == right) {
                    --left
                } else if (left < right) {
                    --right
                } else {
                    throw RuntimeException("Left greater than right")
                }
            }
        }

        throw RuntimeException("Did not finish")
    }
}