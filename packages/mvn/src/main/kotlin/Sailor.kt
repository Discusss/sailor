import exceptions.*
import responses.*
import java.net.URL

object Sailor {

    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class)
    suspend fun getDomain(url: URL): DomainGetResponse? {

    }

    @Throws(InternalServerError::class, BlacklistedError::class, BadRequestError::class)
    suspend fun getDomain(url: URL, key: String): DomainGetAdvancedResponse? {

    }

}