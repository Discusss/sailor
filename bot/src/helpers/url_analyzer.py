import socket
import ssl

import whois

# TODO: Make both functions work when no protocol is present
def check_ssl_certificate(domain):
    try:
        context = ssl.create_default_context()
        with socket.create_connection((domain, 443)) as sock:
            with context.wrap_socket(sock, server_hostname=domain) as ssock:
                cert = ssock.getpeercert()

                # Check if the SSL certificate is valid
                if ssl.match_hostname(cert, domain):
                    return True
    except Exception:
        return False
    return False


def get_domain_registration_info(domain):
    try:
        domain_info = whois.whois(domain)

        return {
            "is_registered": domain_info.status is not None,
            "registrar": domain_info.registrar,
            "creation_date": domain_info.creation_date,
            "updated_date": domain_info.updated_date,
            "expiration_date": domain_info.expiration_date
        }
    except whois.exceptions.FailedParsingWhoisOutput:
        return {
            "is_registered": False,
            "registrar": None,
            "creation_date": None,
            "updated_date": None,
            "expiration_date": None
        }

