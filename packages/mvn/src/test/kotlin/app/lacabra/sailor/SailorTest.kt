package app.lacabra.sailor

import kotlinx.coroutines.runBlocking
import java.net.URL

internal class SailorTest {

    private val url = "http://localhost:8000"
    private val testDomain = "https://google.com"
    private val key = System.getenv("SAILOR_KEY")
    private val masterKey = System.getenv("SAILOR_MASTER_KEY")

    @org.junit.jupiter.api.Test
    fun getDomain() {

        runBlocking {
            val domain = Sailor.getDomain(testDomain, urlOverride = url)
            println(domain)
        }
    }

    @org.junit.jupiter.api.Test
    fun getDomainWithKey() {

        runBlocking {
            assert(key != null && key.isNotEmpty())
            val domain = Sailor.getDomain(testDomain, key, urlOverride = url)
            println(domain)
        }

    }

    @org.junit.jupiter.api.Test
    fun getDomainWithMasterKey() {

        runBlocking {
            assert(key != null && key.isNotEmpty())
            val domain = Sailor.getDomainWithMasterKey(testDomain, masterKey, urlOverride = url)
            println(domain)
        }

    }

    @org.junit.jupiter.api.Test
    fun getDomainFromUrl() {

        runBlocking {

            val uri = URL(testDomain)
            val domain = Sailor.getDomain(uri, urlOverride = url)
            println(domain)
        }
    }

    @org.junit.jupiter.api.Test
    fun getDomainFromUrlWithKey() {

        runBlocking {
            assert(key != null && key.isNotEmpty())
            val uri = URL(testDomain)
            val domain = Sailor.getDomain(uri, key, urlOverride = url)
            println(domain)
        }
    }

    @org.junit.jupiter.api.Test
    fun submitDomain() {

        runBlocking {
            val domain = Sailor.submitDomain(
                rawDomain = "https://${(1..10).map { ('a'..'z').random() }.joinToString("")}.com",
                category = DomainCategory.entries.toTypedArray().random(),
                severity = (0..10).random(),
                notes = "This is a test",
                submittedBy = "ender",
                reason = "This is a test",
                urlOverride = url
            )
            println(domain)
        }
    }

    @org.junit.jupiter.api.Test
    fun getStats() {

        runBlocking {
            val stats = Sailor.getStats(urlOverride = url)
            println(stats)
        }
    }
}