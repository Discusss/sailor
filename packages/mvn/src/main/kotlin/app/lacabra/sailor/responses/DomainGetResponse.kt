package app.lacabra.sailor.responses

import app.lacabra.sailor.DomainCategory
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * The response from the domain get request
 * if the domain was not found, the response will be null
 */
@Serializable
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
    @SerialName("submitted_by")
    val submittedBy: String,
    /**
     * The date the domain was submitted
     */
    @SerialName("submitted_at")
    val submittedAt: String,
)

/**
 * The response from the domain get request with a key
 * if the domain was not found, the response will be null
 */
@Serializable
data class DomainGetAdvancedResponse(
    /**
     * The internal ID of the domain
     */
    val id: Int,

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
    @SerialName("public_notes")
    val publicNotes: String,

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

    /**
     * The reason the user gave when submitting the domain
     */
    @SerialName("submitted_reason")
    val submittedReason: String,

    /**
     * The moderator that approved the domain
     */
    @SerialName("approved_by")
    val approvedBy: String,

    /**
     * The date the domain was approved
     */
    @SerialName("approved_at")
    val approvedAt: String,

    /**
     * The note the moderator left when approving the domain
     */
    val notes: String,

    /**
     * How many times the domain has been consulted
     */
    @SerialName("times_consulted")
    val timesConsulted: Int,
)

/**
 * The response from the domain get request with a key
 * if the domain was not found, the response will be null
 */
@Serializable
data class DomainGetMasterResponse(
    /**
     * The internal ID of the domain
     */
    val id: Int,

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
    @SerialName("public_notes")
    val publicNotes: String,

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

    /**
     * The IP address of the user that submitted the domain, only if the key is a master key
     */
    @SerialName("submitted_ip")
    val submittedIp: String,

    /**
     * The user agent of the user that submitted the domain, only if the key is a master key
     */
    @SerialName("submitted_user_agent")
    val submittedUserAgent: String,

    /**
     * The reason the user gave when submitting the domain
     */
    @SerialName("submitted_reason")
    val submittedReason: String,

    /**
     * The moderator that approved the domain
     */
    @SerialName("approved_by")
    val approvedBy: String,

    /**
     * The date the domain was approved
     */
    @SerialName("approved_at")
    val approvedAt: String,

    /**
     * The key that was used to approve the domain, only if the key is a master key
     */
    @SerialName("approved_key")
    val approvedKey: String,

    /**
     * The note the moderator left when approving the domain
     */
    val notes: String,

    /**
     * How many times the domain has been consulted
     */
    @SerialName("times_consulted")
    val timesConsulted: Int,
)