package app.lacabra.sailor.responses

import kotlinx.serialization.Serializable

@Serializable
data class GenericDomainGetResponse(
    val status: Int,
    val data: DomainGetResponse,
)

@Serializable
data class GenericDomainGetAdvancedResponse(
    val status: Int,
    val data: DomainGetAdvancedResponse,
)

@Serializable
data class GenericDomainGetMasterResponse(
    val status: Int,
    val data: DomainGetMasterResponse,
)

@Serializable
data class GenericDomainPostResponse(
    val status: Int,
    val data: DomainPostResponse,
)

@Serializable
data class GenericStatsGetResponse(
    val status: Int,
    val data: StatsGetResponse,
)

