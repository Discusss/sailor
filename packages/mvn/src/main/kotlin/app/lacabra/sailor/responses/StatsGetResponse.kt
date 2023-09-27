package app.lacabra.sailor.responses

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * The response from the server for the stats get request
 */
@Serializable
data class StatsGetResponse(
    /**
     * The total number of domains in the database
     */
    @SerialName("total_domains")
    val totalDomains: Int,
    /**
     * The total number of blacklisted IPs that can't use the API
     */
    @SerialName("blacklisted_count")
    val blacklistedCount: Int,
    /**
     * The top 5 domains that were submitted
     */
    @SerialName("top_5_domains")
    val top5Domains: List<Top5Domain>,
)

/**
 * Basic data of a domain in the database
 */
@Serializable
data class Top5Domain(
    /**
     * The domain that was submitted
     */
    val domain: String,
    /**
     * The number of times the domain was consulted
     */
    @SerialName("times_consulted")
    val timesConsulted: Int,
)