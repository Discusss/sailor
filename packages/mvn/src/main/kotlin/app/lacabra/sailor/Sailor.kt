package app.lacabra.sailor

import app.lacabra.sailor.exceptions.BadRequestError
import app.lacabra.sailor.exceptions.BlacklistedError
import app.lacabra.sailor.exceptions.ConflictError
import app.lacabra.sailor.exceptions.InternalServerError
import app.lacabra.sailor.responses.DomainGetAdvancedResponse
import app.lacabra.sailor.responses.DomainGetResponse
import app.lacabra.sailor.responses.DomainPostResponse
import app.lacabra.sailor.responses.StatsGetResponse
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.core.FuelError
import com.github.kittinunf.fuel.core.extensions.authentication
import com.github.kittinunf.fuel.core.extensions.jsonBody
import com.github.kittinunf.fuel.coroutines.awaitObjectResponse
import com.github.kittinunf.fuel.serialization.kotlinxDeserializerOf
import java.net.URL

object Sailor {

    private const val API_URL = "https://phishing.lacabra.app"

    /**
     * Get information about the specified domain
     * @param domain The domain to get information about
     * @param urlOverride The URL to use instead of the default one, when null the default one will be used
     * @return [DomainGetResponse] if the domain was found, null otherwise
     * @throws InternalServerError If the API returned an internal server error
     * @throws BlacklistedError If the client IP is blacklisted
     * @throws BadRequestError If the request was malformed or the domain is invalid
     */
    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class)
    suspend fun getDomain(domain: String, urlOverride: String = API_URL): DomainGetResponse? {

        val url = URL(urlOverride)
        return try {
            val response = Fuel.get("$url/api/domain", listOf("domain" to domain))
                .awaitObjectResponse<DomainGetResponse>(kotlinxDeserializerOf())

            response.third
        } catch (e: FuelError) {
            when (e.response.statusCode) {
                400 -> throw BadRequestError()
                403 -> throw BlacklistedError()
                404 -> null
                500 -> throw InternalServerError()
                else -> throw e
            }
        }
    }

    /**
     * Get extra information about the specified domain using a key
     * @param domain The domain to get information about
     * @param key The key to use
     * @param urlOverride The URL to use instead of the default one, when null the default one will be used
     * @return [DomainGetResponse] if the domain was found, null otherwise
     * @throws InternalServerError If the API returned an internal server error
     * @throws BlacklistedError If the client IP is blacklisted
     * @throws BadRequestError If the request was malformed or the domain is invalid
     */
    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class)
    suspend fun getDomain(domain: String, key: String, urlOverride: String = API_URL): DomainGetAdvancedResponse? {

        val url = URL(urlOverride)
        return try {
            val response = Fuel.get("$url/api/domain", listOf("domain" to domain))
                .authentication()
                .bearer(key)
                .awaitObjectResponse<DomainGetAdvancedResponse>(kotlinxDeserializerOf())

            response.third
        } catch (e: FuelError) {
            when (e.response.statusCode) {
                400 -> throw BadRequestError()
                403 -> throw BlacklistedError()
                404 -> null
                500 -> throw InternalServerError()
                else -> throw e
            }
        }
    }

    /**
     * Get information about the specified domain
     * @param url The domain to get information about
     * @return [DomainGetResponse] if the domain was found, null otherwise
     * @throws InternalServerError If the API returned an internal server error
     * @throws BlacklistedError If the client IP is blacklisted
     * @throws BadRequestError If the request was malformed or the domain is invalid
     */
    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class)
    suspend fun getDomain(url: URL, urlOverride: String = API_URL): DomainGetResponse? =
        getDomain(url.host, urlOverride)

    /**
     * Get extra information about the specified domain using a key
     * @param url The domain to get information about
     * @param key The key to use
     * @return [DomainGetAdvancedResponse] if the domain was found, null otherwise
     * @throws InternalServerError If the API returned an internal server error
     * @throws BlacklistedError If the client IP is blacklisted
     * @throws BadRequestError If the request was malformed or the domain is invalid
     */
    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class)
    suspend fun getDomain(url: URL, key: String, urlOverride: String = API_URL): DomainGetAdvancedResponse? =
        getDomain(url.host, key, urlOverride)

    /**
     * Submit a domain to the API that will be reviewed by the staff and approved or rejected
     * @param domain The domain to submit
     * @param category [DomainCategory] of the domain, defaults to [DomainCategory.Other]
     * @param severity The severity of the domain, defaults to 0
     * @param notes Notes to add to the domain, defaults to ""
     * @param submittedBy The user that submitted the domain
     * @param reason The reason for submitting the domain, for example "I found this domain in a phishing email"
     * @param urlOverride The URL to use instead of the default one, when null the default one will be used
     * @return [DomainPostResponse] if the domain was submitted successfully, null otherwise
     * @throws InternalServerError If the API returned an internal server error
     * @throws BlacklistedError If the client IP is blacklisted
     * @throws BadRequestError If the request was malformed or the domain is invalid
     * @throws ConflictError If the domain already exists in the database
     */
    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class, ConflictError::class)
    suspend fun submitDomain(
        domain: String,
        category: DomainCategory = DomainCategory.Other,
        severity: Int = 0,
        notes: String = "",
        submittedBy: String,
        reason: String,
        urlOverride: String = API_URL
    ): DomainPostResponse {

        val url = URL(urlOverride)
        return try {
            val response = Fuel.post("$url/api/domain")
                .jsonBody(
                    HashMap<String, Any>().apply {
                        put("domain", domain)
                        put("category", category.ordinal)
                        put("severity", severity)
                        put("notes", notes)
                        put("submitted_by", submittedBy)
                        put("reason", reason)
                    }.toString()
                )
                .awaitObjectResponse<DomainPostResponse>(kotlinxDeserializerOf())

            response.third
        } catch (e: FuelError) {
            when (e.response.statusCode) {
                400 -> throw BadRequestError()
                403 -> throw BlacklistedError()
                409 -> throw ConflictError()
                500 -> throw InternalServerError()
                else -> throw e
            }
        }
    }

    /**
     * Get the statistics of the API
     * @return [StatsGetResponse] if the request was successful, null otherwise
     * @throws InternalServerError If the API returned an internal server error
     */
    @Throws(InternalServerError::class)
    suspend fun getStats(urlOverride: String = API_URL): StatsGetResponse {

        val url = URL(urlOverride)
        return try {
            val response = Fuel.get("$url/api/stats")
                .awaitObjectResponse<StatsGetResponse>(kotlinxDeserializerOf())

            response.third
        } catch (e: FuelError) {
            when (e.response.statusCode) {
                500 -> throw InternalServerError()
                else -> throw e
            }
        }
    }
}