package responses

import DomainCategory

/**
 * The response from the domain get request
 * if the domain was not found, the response will be null
 */
data class DomainGetResponse(
    /**
     * The domain that was checked
     */
    val domain: String,
    /**
     * The category of the domain
     */
    val category: DomainCategory,
    /**
     * The severity of the domain. 0 is the lowest and 10 is the highest depending on how dangerous the domain is.
     * Example: How easy it is for a user to be tricked into giving away their credentials.
     */
    val severity: Int,
    /**
     * The public notes of the domain left by the moderators
     */
    val notes: String,
    /**
     * The user that submitted the domain
     */
    val submittedBy: String,
    /**
     * The date the domain was submitted
     */
    val submittedAt: String,
)

/**
 * The response from the domain get request with a key
 * if the domain was not found, the response will be null
 */
data class DomainGetAdvancedResponse(
    /**
     * The internal ID of the domain
     */
    val id: String,
    /**
     * The domain that was checked
     */
    val domain: String,
    /**
     * The category of the domain
     */
    val category: DomainCategory,
    /**
     * The severity of the domain. 0 is the lowest and 10 is the highest depending on how dangerous the domain is.
     * Example: How easy it is for a user to be tricked into giving away their credentials.
     */
    val severity: Int,
    /**
     * The public notes of the domain left by the moderators
     */
    val publicNotes: String,
    /**
     * The user that submitted the domain
     */
    val submittedBy: String,
    /**
     * The date the domain was submitted
     */
    val submittedAt: String,
    /**
     * The reason the user gave when submitting the domain
     */
    val submittedReason: String,
    /**
     * The moderator that approved the domain
     */
    val approvedBy: String,
    /**
     * The date the domain was approved
     */
    val approvedAt: String,
    /**
     * The note the moderator left when approving the domain
     */
    val notes: String,
    /**
     * How many times the domain has been consulted
     */
    val timesConsulted: Int,
)