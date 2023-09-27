package app.lacabra.sailor.responses

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * The response from the server after submitting a domain
 */
@Serializable
data class DomainPostResponse(
    /**
     * The domain that was submitted
     */
    val domain: String,
    /**
     * The category of the domain
     */
    val category: String,
    /**
     * The severity of the domain. 0 is the lowest and 10 is the highest depending on how dangerous the domain is.
     * Example: How easy it is for a user to be tricked into giving away their credentials.
     */
    val severity: Int,
    /**
     * The notes of the domain left by the submitter
     */
    val notes: String,
    /**
     * The user that submitted the domain
     */
    @SerialName("submitted_by")
    val submittedBy: String,
    /**
     * The date the domain was submitted
     */
    @SerialName("submitted_at")
    val submittedAt: String,
)