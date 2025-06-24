# Self-Signed Certificates with CA

Source: <https://www.cockroachlabs.com/docs/stable/create-security-certificates-openssl>

## Become a CA

Create this `ca.cnf` file.

Then create the CA key and certificate:

```console
$ openssl req -x509 -newkey rsa:4096 -sha256 -nodes -config ca.cnf -days 365 -keyout ca.key -out ca.crt
$ openssl x509 -noout -text -in ca.crt
    Issuer: C=NO, O=Example Company, CN=Example Company CA
    Validity
        Not Before: Jun 20 18:39:34 2025 GMT
        Not After : Jul 20 18:39:34 2025 GMT
    Subject: C=NO, O=Example Company, CN=Example Company CA
    X509v3 extensions:
        X509v3 Key Usage: critical
            Digital Signature, Non Repudiation, Key Encipherment, Certificate Sign
        X509v3 Basic Constraints: critical
```

Reset database and index files:

```console
$ rm -f index.txt serial.txt
$ touch index.txt
$ echo '01' > serial.txt
```

## Server Certificate

Create the `server.cnf` file.

Then create the server's CSR and sign it using the CA:

```console
$ mkdir -p certs/
$ openssl req -newkey rsa:4096 -nodes -config server.cnf -keyout certs/server.key -out certs/server.csr
$ openssl ca -config ca.cnf -keyfile ca.key -cert ca.crt -policy signing_policy -extensions signing_node_req -outdir certs/ -out certs/server.crt -in certs/server.csr -batch
$ openssl x509 -in certs/server.crt -text
        Issuer: C=NO, O=Example Company, CN=Example Company CA
        Validity
            Not Before: Jun 20 18:56:26 2025 GMT
            Not After : Jun 20 18:56:26 2026 GMT
        Subject: O=Example Company
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name: critical
                DNS:example.com, DNS:*.example.com, IP Address:10.0.0.1
```

## Client Certificate

Create the `client.cnf` file and use it to create a CSR:

```console
$ openssl req -newkey rsa:2048 -nodes -config client.cnf -keyout certs/client.1234.key -out certs/client.1234.csr
```

or alternatively, use this one-liner to create a CSR without a file:

```console
$ openssl req -newkey rsa:2048 -nodes -addext "subjectAltName=DNS:mqtt.example.com" -subj '/CN=1234/O=Company' -keyout certs/client.1234.key -out certs/client.1234.csr
```

Note that NATS will try to map multiple fields to a user:

1. All e-mail addresses first (none here; add them with `-addext "subjectAltName=email:user@host"`)
2. All DNS names: `mqtt.example.com`
3. If no user is found, it will try the whole certificate subject line: `CN=1234,O=Company`

Now view it, and sign in with CA:

```console
$ openssl req -in certs/client.1234.csr -text
    Subject: O=Company, CN=1234
    Public-Key: (2048 bit)
    Requested Extensions:
    X509v3 Subject Alternative Name:
        DNS:root
$ openssl ca -config ca.cnf -keyfile ca.key -cert ca.crt -policy signing_policy -extensions signing_client_req -outdir certs/ -out certs/client.1234.crt -in certs/client.1234.csr -batch
$ openssl x509 -in certs/client.1234.crt -text
Certificate:
    Serial Number: 2 (0x2)
    Issuer: C=NO, O=Example Company, CN=Example Company CA
    Validity
        Not Before: Jun 21 10:05:55 2025 GMT
        Not After : Jun 21 10:05:55 2026 GMT
    Subject: O=Company, CN=1234
    X509v3 Key Usage: critical
        Digital Signature, Key Encipherment
    X509v3 Extended Key Usage:
        TLS Web Client Authentication
    X509v3 Subject Alternative Name:
        DNS:root
```


# Test OpenSSL Server

Use OpenSSL client:

```console
$ openssl s_client -servername localhost:8883 -tlsextdebug -connect localhost:8883
Certificate chain
 0 s:O=Example Company
   i:C=NO, O=Example Company, CN=Example Company CA
   a:PKEY: rsaEncryption, 4096 (bit); sigalg: RSA-SHA256
   v:NotBefore: Jun 23 17:44:51 2025 GMT; NotAfter: Jun 23 17:44:51 2026 GMT
Server certificate
subject=O=Example Company
issuer=C=NO, O=Example Company, CN=Example Company CA
Acceptable client certificate CA names
C=NO, O=Example Company, CN=Example Company CA
```

Spin up an OpenSSL server:

```console
$ openssl s_server -port 8883 -CAfile tls/ca.crt -cert tls/certs/server.crt -key tls/certs/server.key -trace -tlsextdebug -verify 0
```

# NATS Config

Run NATS with `--debug --trace` to see how/why things work.
