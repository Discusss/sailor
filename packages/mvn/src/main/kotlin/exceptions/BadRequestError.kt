package exceptions

class BadRequestError: Exception("Bad Request") {
    override fun toString(): String {
        return "BadRequestError()"
    }
}