package exceptions

class InternalServerError: Exception("Internal Server Error") {
    override fun toString(): String {
        return "InternalServerError()"
    }
}