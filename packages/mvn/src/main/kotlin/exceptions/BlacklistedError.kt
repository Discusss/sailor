package exceptions

class BlacklistedError: Exception("Your IP has been blacklisted.") {
    override fun toString(): String {
        return "BlacklistedError()"
    }
}