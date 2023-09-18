import socket
from urllib.parse import urlparse

import whois
from OpenSSL import SSL
from cryptography import x509
from cryptography.hazmat._oid import NameOID
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization


def get_netloc(url):
    parsed = urlparse(url)
    if parsed.netloc == "" and parsed.path != "":
        # Handle cases where netloc is empty but path is present (e.g., google.com)
        return parsed.path
    else:
        return parsed.netloc


def check_ssl_certificate(domain):
    try:
        with socket.create_connection((domain, 443)) as sock:
            ctx = SSL.Context(SSL.SSLv23_METHOD)
            ctx.check_hostname = True
            ctx.verify_mode = SSL.VERIFY_PEER

            sock = SSL.Connection(ctx, sock)
            sock.set_tlsext_host_name(domain.encode())
            sock.set_connect_state()
            sock.do_handshake()

            cert_pem = sock.get_peer_certificate().to_cryptography()
            x509_cert = x509.load_der_x509_certificate(
                cert_pem.public_bytes(serialization.Encoding.DER), default_backend()
            )
            return (
                True,
                x509_cert.issuer.get_attributes_for_oid(NameOID.ORGANIZATION_NAME)[
                    0
                ].value,
            )
    except SSL.Error:
        return False, ""
    except socket.gaierror:
        return False, ""


def get_domain_registration_info(domain):
    try:
        domain_info = whois.whois("https://" + domain)

        return {
            "is_registered": domain_info.status is not None,
            "registrar": domain_info.registrar,
            "creation_date": domain_info.creation_date,
            "updated_date": domain_info.updated_date,
            "expiration_date": domain_info.expiration_date,
        }
    except Exception:
        try:
            domain_info = whois.whois("http://" + domain)

            return {
                "is_registered": domain_info.status is not None,
                "registrar": domain_info.registrar,
                "creation_date": domain_info.creation_date,
                "updated_date": domain_info.updated_date,
                "expiration_date": domain_info.expiration_date,
            }
        except Exception:
            return {
                "is_registered": False,
                "registrar": None,
                "creation_date": None,
                "updated_date": None,
                "expiration_date": None,
            }
