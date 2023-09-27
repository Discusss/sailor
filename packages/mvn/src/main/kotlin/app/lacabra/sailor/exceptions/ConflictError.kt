package app.lacabra.sailor.exceptions

class ConflictError: Exception("Conflict") {
    override fun toString(): String {
        return "ConflictError()"
    }
}