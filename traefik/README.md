Traefik
=======

An Edge Router: the door to your platform.

Traefik automatically discovers services from Docker and publishes them.
It supports many configuration providers:
[Kubernetes, Docker, Docker Swarm, AWS, ECS, Consul, Etcd, ZooKeeper](https://doc.traefik.io/traefik/providers/overview/),
as well as File, Redis, HTTP.




Quick Start: Docker
====================

Docker-Compose:


```yaml
version: '3'

services:

  # Edge router
  # This is the only container that has published ports.
  traefik:
    image: traefik:v2.9

    ports:
      # Incoming requests
      - "80:80"
      # Web UI (enabled by --api.insecure=true)
      - "8080:8080"

    # Listed to Docker.
    # Enable Web UI
    command: --api.insecure=true --providers.docker

    volumes:
      # Listen to Docker daemon
      - /var/run/docker.sock:/var/run/docker.sock


  # Services

  # Whoami: 
  #   When Host = whoami.localhost
  #   Echo IP address
  whoami:
    # Whoami: shows its IP address
    image: traefik/whoami
    labels:
      # Define: http.router: name="whoami"
      # Rule: Hostname match: "whoami.localhost"
      - "traefik.http.routers.whoami.rule=Host(`whoami.localhost`)"
```


Traefik will discover new services and even load-balance them:

```console
$ docker-compose up -d --scale whoami=2
```

See also: [Quick Start with Kubernetes](https://doc.traefik.io/traefik/getting-started/quick-start-with-kubernetes/).

















Configuration
=============

Two different configurations:

* Startup configuration: *static configuration*
* Dynamic routing configuration: *dynamic configuration*

Three mutually-exclusive ways to provide static configuration:

1. Config file
2. Command-line arguments
3. Environment variables

## Config File

Traefik tries to locate `traefik.yaml` or `traefik.toml` in:

1. `/etc/traefik/`
2. `$XDG_CONFIG_HOME/`
3. `$HOME/.config/`
4. `.` (the working directory)

Override with `--configFile`:

```console
$ traefik --configFile=foo/bar/myconfigfile.yml
```

See all CLI options: [static-configuration/cli](https://doc.traefik.io/traefik/reference/static-configuration/cli/)
or `traefik --help`.

See all ENV options: [static-configuration/env](https://doc.traefik.io/traefik/reference/static-configuration/env/)

## Configuration Discovery

Every config lives in their provider's namespace. 
That is, if a middleware `add-foo-prefix` is defined in a file, this is how you reference it in a Docker label:

```yaml
labels:
- "traefik.http.routers.my-container.middlewares=add-foo-prefix@file"
```

By default, Traefik creates routes for all detected containers.
Disable: `exposedByDefault=false` and use label `traefik.enable`.

















Providers
=========

## Docker

Attach labels to your containers. 
Traefik will pick them up.

Port detection:

* If a container exposes one port, Traefik routes to it
* If the container exposes multiple ports, you must manually specify which one to use

    > traefik.http.services.<service_name>.loadbalancer.server.port

Provider configuration:

* `exposedByDefault`: expose all containers by default? 
  
    If `false`, use `traefik.enable=true` label to mark containers.

* `defaultRule`: the Go template: the default rule to apply. 

    Example:
    
    ```
    "Host(`{{ .Name }}.{{ index .Labels \"customLabel\"}}`)"
    ```

* `allowEmptyServices`: set `true` to allow publishing unhealthy containers




## File Provider

The provider lets you define dynamic configuration in a YAML file.

Static: watch one file:

```yaml
providers:
  file:
    filename: /path/to/config/dynamic_conf.yml
    watch: true
```

Static: watch directory:

```yaml
providers:
  file:
    directory: /path/to/config
    watch: true
```

Example:

```yaml
# NOTE: you can use Go templates here, including {{range ...}} ... {{end}}

http:
  # Add the router
  routers:
    router0:
      entryPoints:
      - web
      middlewares:
      - my-basic-auth
      service: service-foo
      rule: Path(`/foo`)

  # Add the middleware
  middlewares:
    my-basic-auth:
      basicAuth:
        users:
        - test:$apr1$H6uskkkW$IgXLP6ewTrSuBkTrqE8wj/
        - test2:$apr1$d9hr9HBB$4HxwgUir3HP4EsggP/QNo0
        usersFile: etc/traefik/.htpasswd

  # Add the service
  services:
    service-foo:
      loadBalancer:
        servers:
        - url: http://foo/
        - url: http://bar/
        passHostHeader: false
```




## HTTP Provider

Load config from a remote server (static file hosting?):

```yaml
providers:
  http:
    endpoint: "http://127.0.0.1:9000/api"
    pollInterval: "5s"

```























Routing & Load Balancing
=========================

When you start Traefik, you first define *entrypoints*: ~port numbers. Connected to these entrypoints, *routers* analyze the incoming requests to see if they match a set of *rules*. If they do, the router might apply *middleware*, and then will forward the request to your *services*.

So:

* *Providers* discover services
* *Entrypoints* listen for incoming traffic
* *Routers* analyze requests
* *Services* forward requests to your services
* *Middlewares* may update the request (authentication, rate limiting, headers, etc)

## Example HTTP Configuration

Example: listen on `:8081`, use a config file:

CLI:

```
--entryPoints.web.address=:8081
--providers.file.directory=/path/to/dynamic/conf
```

or YAML:

```yaml
entryPoints:
  web:
    address: :8081

providers:
  file:
    directory: /path/to/dynamic/conf
```

Now dynamic configuration:

```yaml
# HTTP routing
http:
  routers:
    # If this rule matches: apply middleware, forward to service
    to-whoami:
      rule: "Host(`example.com`) && PathPrefix(`/whoami/`)"
      middlewares:
      - test-user
      service: whoami

  middlewares:
    # Middleware: require basic authentication
    test-user:
      basicAuth:
        users:
        - test:$apr1$H6uskkkW$IgXLP6ewTrSuBkTrqE8wj/

  services:
    # How to find a service
    whoami:
      loadBalancer:
        servers:
        - url: http://private/whoami-service

# Example: TCP routing
# Priority: TCP routes have priority over HTTP routes (when listening to the same entry points)
tcp:
  routers:
    to-whoami-tcp:
      service: whoami-tcp
      rule: HostSNI(`whoami-tcp.example.com`)
      tls: {}

  services:
    whoami-tcp:
      loadBalancer:
        servers:
        - address: xx.xx.xx.xx:xx
```

## Transport Configuration

Here are a few parameters that configure globally when happens with the connections between Traefik and the backends:

```yaml
# Static configuration
serversTransport:
  # disables SSL certificate verification.
  insecureSkipVerify: true
  # list of CA authorities (when using a self-signed TLS cert)
  rootCAs:
    - foo.crt
    - bar.crt
  # Max idle (keep-alive) connections to keep per host
  maxIdleConnsPerHost: 7
  # Timeouts for forwarded requests
  forwardingTimeouts:
    # Establish connection timeout
    dialTimeout: 30s
    # Time to wait for server headers. 0s = no timeout
    responseHeaderTimeout: 0s
    # Max time to keep an idle connection open
    idleConnTimeout: 90s
```




## Entrypoints

Entrypoints open connections for incoming requests.

Configuration example: port 80, port 443, UDP 1704

```yaml
# Static configuration
entryPoints:
  web:
    address: ":80"

  websecure:
    address: ":443"
  
  streaming:
    address: ":1704/udp"
```

or CLI:

```
--entryPoints.web.address=:80
--entryPoints.websecure.address=:443
--entryPoints.streaming.address=:1704/udp
```

Example: listen on specific IP addresses only:

```yaml
entryPoints:
  specificIPv4:
    address: "192.168.2.7:8888"
  specificIPv6:
    address: "[2001:db8::1]:8888"
```

Configuration is available for certain protocols:

```yaml
entryPoints:
  web:
    address: ":80"
    # HTTP/2 configuration
    http2:
      maxConcurrentStreams: 250
    
    # HTTP/3 configuration
    # HTTP/3 always starts as a TCP connection, and then gets upgraded to UDP.
    http3:
      advertisedPort: 443
    
    # Forwarded headers
    forwardedHeaders:
      # Trust X-Forwarder-* headers from specific IPs
      trustedIPs:
        - "127.0.0.1/32"
        - "192.168.1.7"
      # Alternative: trust all forwarded headers
      insecure: true
    
    # Transport
    transport:
      # Timeouts for incoming requests
      respondingTimeouts:
        readTimeout: 0s
        writeTimeout: 0s
        idleTimeout: 180s
        # How to handle the shutdown phase
        lifeCycle:
          # Keep accepting requests before graceful shutdown. 
          # Gives downstream load-balancers time to take us out of rotation
          requestAcceptGraceTimeout: 0s
          # The time active requests have to finish before Traefik stops
          graceTimeOut: 42
```

HTTP options: HTTP redirection to HTTPS:


```yaml
entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure   # target: entrypoint name , or port (":443")
          scheme: https   # target scheme. Default "https"
          permanent: true # apply a permanent redirection. Default: true
          priority: MaxInt32-1  # Priority of the generated router

  websecure:
    address: :443
```

### Entrypoint Middleware

Middleware: executed before router middleware:

```yaml
entryPoints:
  websecure:
    address: ':443'
    http:
      middlewares:
        - auth@file
        - strip@file
```

### TLS configuration

TLS config applied to all routers associated with this entrypoint.
See TLS for routers: it's the same.

```yaml
entryPoints:
  websecure:
    address: ':443'
    http:
      tls:
        options: foobar
        certResolver: leresolver  # Let's Encrypt
        domains:
          - main: example.com
            sans:
              - foo.example.com
              - bar.example.com
          - main: test.com
            sans:
              - foo.test.com
              - bar.test.com
```











## Routers

By default, routers listen to every entrypoint. That is, a host/url rule would apply to all incoming traffic:

```
--entrypoints.web.address=:80
--entrypoints.websecure.address=:443
--entrypoints.other.address=:9090
```

```yaml
# Dynamic config
http:
  routers:
    Router-1:
      rule: "Host(`example.com`)"
      service: "service-1"
      # By default, routers would listen to every entrypoint
      # Here's how you pick specific entrypoints
      entryPoints: ["websecure", "other"]
```

### Rules

Example rule: host, `or` host `and` url

```go
rule = "Host(`example.com`) || (Host(`example.org`) && Path(`/traefik`))"
```

Available operators: `()`, `&&`, `||`, `!`.

Use Go strings: `""` or <code>``</code>

All available matchers:

```go
// Has header, with value
Headers(`key`, `value`)
HeadersRegexp(`key`, `regexp`)

// Host matches one of.
// NOTE: must be ASCII. Use punycode!
Host(`example.com`, ...)
HostRegexp(`example.com`, `{subdomain:[a-z]+}.example.com`, ...)

// Match HTTP request
// Use Go regexp. Named {name:.*} parameter is just a convenience.
Method(`GET`, ...)
Path(`/path`, `/articles/{cat:[a-z]+}/{id:[0-9]+}`, ...)
PathPrefix(`/api`, `/api/v1`)  // service mounted on a url, handles sub-paths
Query(`foo=bar`, `bar=baz`)

// Match client IP
ClientIP(`10.0.0.0/16`, `::1`)
```

Rules are evaluated "before" any middleware. 

Rules are sorted desc using rule length: priority is equal to the length of the rule. 
Here's how you can customize priorities:

```yaml
http:
  routers:
    Router-1:
      rule: "HostRegexp(`{subdomain:[a-z]+}.traefik.com`)"
      entryPoints: ["web"]
      service: service-1
      priority: 1
    Router-2:
      rule: "Host(`foobar.traefik.com`)"
      entryPoints: ["web"]
      priority: 2
      service: service-2
```


### Middleware

Middlewares take effect only if the rule matches. 

Middlewares are applied in the order of declaration.

Example: 

```yaml
http:
  routers:
    my-router:
      rule: "Path(`/foo`)"
      middlewares:
      - authentication   # declared elsewhere
      service: service-foo
```




### Router & TLS

When a TLS section is specified, the route is dedicated to HTTPS requests only. It will ignore non-TLS requests:

```yaml
http:
  routers:
    Router-1:
      rule: "Host(`foo-domain`) && Path(`/foo-path/`)"
      service: service-id
      tls: {}  # only https here
```

If you want the same route to apply to both http and https, you'll need to define two different routers: 
one with the TLS section, and one without:

```yaml
http:
  routers:
    # HTTPS only
    my-https-router:
      rule: "Host(`foo-domain`) && Path(`/foo-path/`)"
      service: service-id
      tls: {}

    # HTTP only
    my-http-router:
      rule: "Host(`foo-domain`) && Path(`/foo-path/`)"
      service: service-id
```

Options enable TLS configuration.

Note that TLS options do not map to a rule: they actually map to the `Host(...)` domain!
If two rules provide options for a `Host(...)` name, **both are discarded**!!!

```yaml
http:
  routers:
    routerfoo:
      rule: "Host(`snitest.com`) && Path(`/foo`)"
      tls:
        # Refer to a named "tls" section
        # NOTE: only one per host!
        options: foo

        # Generate certificates based on Host() and HostSNI()
        # With multiple Host(a, b, ...) in a rule: generate certificate for the first domain, SAN second domain (alternative domain)
        certResolver: foo
        
        # Manually specify domains and alternative domains (SANs)
        domains:
          - main: "snitest.com"
            sans:
              # With Let's Encrypt, wildcard certificates can only be generated through a DNS-01 challenge
              - "*.snitest.com"
        

# TLS options referenced in `tls`
tls:
  options:
    foo:
      minVersion: VersionTLS12
      cipherSuites:
        - TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
        - TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256
        - TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256
        - TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
        - TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
```

### TCP Routers

TCP routers match before HTTP. If no TCP router matches, then HTTP takes over.

Rules:

```go
// Server Name indication
// Only works with TLS routers, because Server Name comes from the TLS
HostSNI(`domain-1`, ...)
HostSNIRegexp(`example.com`, `{subdomain:[a-z]+}.example.com`, ...)

// Client IP
ClientIP(`10.0.0.0/16`, `::1`)

// Check the conection's ALPN:
// > Application-Layer Protocol Negotiation (ALPN) is a Transport Layer Security (TLS) extension
// NOTE: it must read a few bytes first
ALPN(`mqtt`, `h2c`)
```

Example:

```yaml
tcp:
  routers:
    my-router:
      # One exception: HostSNI(`*`) works with non-TLS rules.
      # Any other host would require a TLS connection.
      rule: "HostSNI(`*`)"
      service: service-foo
      # Apply middleware
      middlewares: ["ipwhitelist"]
```

TLS options:

```yaml
tcp:
  routers:
    Router-1:
      rule: "HostSNI(`foo-domain`)"
      service: service-id
      tls:
        # Pass requests "as is".
        # Otherwise, Traefik will terminate TLS connection.
        passthrough: true

        # Generate certificate
        certResolver: ...
```



### UDP Routers

```yaml
udp:
  routers:
    Router-1:
      entryPoints: ["streaming"]  # optional: listen to just one endpoint
      service: "service-1"
```






















## Services

Service: configures how to reach the actual services that handle the incoming requests.

Example: http service with two servers

```yaml
http:
  services:
    my-service:
      loadBalancer:
        servers:
        # Round-robin balancing
        - url: "http://<private-ip-server-1>:<private-port-server-1>/"
        - url: "http://<private-ip-server-2>:<private-port-server-2>/"
```

Each service has a load balancer, even if there is only one server:

```yaml
http:
  services:
    my-service:
      loadBalancer:
        servers:
          - url: "http://private-ip-server-1/"
```

Sticky sessions: sets a cookie to keep the client on the same server.

```yaml
http:
  services:
    my-service:
      loadBalancer:
        sticky:
         cookie: {}

         # Or set cookie options
         cookie:
          secure: true
          httpOnly: true
      servers:
        ...
```

Health check: removes unhealthy servers from the load balancing rotation

```yaml
http:
  services:
    Service-1:
      loadBalancer:
        healthCheck:
          method: GET  # default: GET
          path: /health  # expects: 2xx and 3xx
          port: 8080
          interval: 30s # default: 30s
          # also: scheme, hostname, interval, timeout, headers (map), followRedirects, method
```

Response forwarding: how backend responses are forwarded to the client:

```yaml
http:
  services:
    Service-1:
      loadBalancer:
        responseForwarding:
          # How often to flush the response body
          # Default: flush every 100ms
          flushInterval: 100ms
```

Set forwarding timeouts:

```yaml
# Dynamic configuration
http:
  serversTransports:
    mytransport:
      forwardingTimeouts:
        dialTimeout: "1s"
        responseHeaderTimeout: "1s"
        idleConnTimeout: "1s"
        readIdleTimeout: "1s"
        pingTimeout: "1s"
```

Weighted round robin: load balance based on weights:

```yaml
http:
  services:
    # Weighted load balancer
    app:
      weighted:
        # Enable health checks down the line
        healthCheck: {}
        # WRR balancing
        services:
        - name: appv1
          weight: 3
        - name: appv2
          weight: 1

    # Server groups
    appv1:
      loadBalancer:
        servers:
        - url: "http://private-ip-server-1/"

    appv2:
      loadBalancer:
        servers:
        - url: "http://private-ip-server-2/"
```

Mirroring: mirror requests sent to a service to other services

```yaml
http:
  services:
    mirrored-api:
      mirroring:
        service: appv1
        healthCheck: {}
        # maxBodySize is the maximum size allowed for the body of the request.
        # If the body is larger, the request is not mirrored.
        # Default value is -1, which means unlimited size.
        maxBodySize: 1024
        mirrors:
        - name: appv2
          percent: 10

    ...
```

Failover: forward all requests to a fallback service when the main service becomes unreachable.

NOTE: this strategy can only be defined with the File provider

```yaml
http:
  services:
    # Failover
    app:
      failover:
        service: main
        fallback: backup
        healthCheck: {}

    # Services: main, backup
    main:
      loadBalancer:
        # NOTE: failover relies on healthchecks!!
        healthCheck:
          path: /status
          interval: 10s
          timeout: 3s
        servers:
        - url: "http://private-ip-server-1/"

    backup:
      loadBalancer:
        servers:
        - url: "http://private-ip-server-2/"
```




HTTPS & TLS
============

## Cert Files

You can use cert files:

```yaml
# Dynamic configuration

tls:
  certificates:
    - certFile: /path/to/domain.cert
      keyFile: /path/to/domain.key
    - certFile: /path/to/other-domain.cert
      keyFile: /path/to/other-domain.key
```

Here's how to use a default certificate:

```yaml
tls:
  stores:
    default:
      defaultCertificate:
        certFile: path/to/cert.crt
        keyFile: path/to/cert.key
```


## TLS Options

```yaml

tls:
  options:
    # Option groups
    default:
      # Min version
      minVersion: VersionTLS12
      # Require clients to specify a server_name extension
      sniStrict: true
      # List of supported ALPN protocols. 
      alpnProtocols: ["http/1.1", "h2"]  # Default="h2, http/1.1, acme-tls/1"

    mintls13:
      minVersion: VersionTLS13
      # Max version
      maxVersion: VersionTLS13
```


## Let's Encrypt

Also, Traefik can automatically generate a Let'sEncrypt (ACME) TLS certificate!

**NOTE**: Let's Encrypt has rate limiting! Use the staging server for tests! And store your certificates!

```yaml
tls:
  stores:
    default:
      defaultGeneratedCert:
        resolver: myresolver
        domain:
          main: example.org
          sans:
            - foo.example.org
            - bar.example.org
```

or Docker:

```yaml
labels:
  - "traefik.tls.stores.default.defaultgeneratedcert.resolver=myresolver"
  - "traefik.tls.stores.default.defaultgeneratedcert.domain.main=example.org"
  - "traefik.tls.stores.default.defaultgeneratedcert.domain.sans=foo.example.org, bar.example.org"
```

Traefik requires you to define "Certificate Resolvers" in the static configuration:

```yaml
certificatesResolvers:
  myresolver:
    acme:
      # Used for registration
      email: "user@example.com"
      
      # Store certificates here.
      # With Docker, mount the JSON file:
      #   $ docker run -v "/my/host/acme.json:/acme.json" traefik
      #   $ docker run -v "/my/host/acme:/etc/traefik/acme" traefik
      storage: "acme.json"

      # Uncomment to use staging server (no rate limit)
      # caServer: "https://acme-staging-v02.api.letsencrypt.org/directory"

      # Certificate duration in hours.
      # By default, Traefik will automatically renew certificates 30 days before they expire
      certificatesDuration: 2160  # Default = 2160 hours = 90 days

      # Which ACME challenge to use
      tlsChallenge:  # recommended
      # httpChallenge: { entrypoint: "web" }
      # dnsChallenge: { provider: "digitalocean" }
```

Example Docker: single domain

```yaml
labels:
  - traefik.http.routers.blog.rule=Host(`example.com`) && Path(`/blog`)
  - traefik.http.routers.blog.tls=true
  - traefik.http.routers.blog.tls.certresolver=myresolver
```

Example: multiple domains

```yaml
labels:
  - traefik.http.routers.blog.rule=(Host(`example.com`) && Path(`/blog`)) || Host(`blog.example.org`)
  - traefik.http.routers.blog.tls=true
  - traefik.http.routers.blog.tls.certresolver=myresolver
```






















Middlewares
===========

Example middleware: YAML:

```yaml
# As YAML Configuration File
http:
  routers:
    router1:
      service: myService
      rule: "Host(`example.com`)"
      # Middlewares
      middlewares:
        - "foo-add-prefix"

  # Middlewares
  middlewares:
    foo-add-prefix:
      addPrefix:
        prefix: "/foo"

  services:
    service1:
      loadBalancer:
        servers:
          - url: "http://127.0.0.1:80"
```

or Docker:

```yaml
whoami:
  #  A container that exposes an API to show its IP address
  image: traefik/whoami
  labels:
    - "traefik.http.middlewares.foo-add-prefix.addprefix.prefix=/foo"
    - "traefik.http.routers.router1.middlewares=foo-add-prefix@docker"
```

## Middlewares

```yaml
labels:
  # Add Prefix: /foo
  - "traefik.http.middlewares.add-foo.addprefix.prefix=/foo"

  # Buffering: set max request body size to 2Mb
  - "traefik.http.middlewares.limit.buffering.maxRequestBodyBytes=2000000"

  # Compress responses with gzip
  - "traefik.http.middlewares.test-compress.compress=true"
  - "traefik.http.middlewares.test-compress.compress.excludedcontenttypes=text/event-stream"

  # Automatically set Content-Type, if not set already
  - "traefik.http.middlewares.autodetect.contenttype.autodetect=false"

  # Retry: reissue the request if the server does not reply. Has exponential backoff.
  - "traefik.http.middlewares.test-retry.retry.attempts=4"
  - "traefik.http.middlewares.test-retry.retry.initialinterval=100ms"
```

Redirect:

```yaml
  # Use HTML error page for status codes
  - "traefik.http.middlewares.test-errors.errors.status=500-599"  # code range
  - "traefik.http.middlewares.test-errors.errors.service=serviceError"  # the service that will serve the page
  - "traefik.http.middlewares.test-errors.errors.query=/{status}.html"  # URL

  # Redirect to a scheme/port
  - "traefik.http.middlewares.test-redirectscheme.redirectscheme.scheme=https"
  - "traefik.http.middlewares.test-redirectscheme.redirectscheme.permanent=true"

  # Redirect to a host/path: regexp match & rewrite
  - "traefik.http.middlewares.test-redirectregex.redirectregex.regex=^http://localhost/(.*)"
  - "traefik.http.middlewares.test-redirectregex.redirectregex.replacement=http://mydomain/$${1}"
```

Rewrite:

```yaml
labels:
  # Replace the path completely before forwarding the request
  - "traefik.http.middlewares.test-replacepath.replacepath.path=/foo"
  
  # Replace the path, regexp
  - "traefik.http.middlewares.test-replacepathregex.replacepathregex.regex=^/foo/(.*)"
  - "traefik.http.middlewares.test-replacepathregex.replacepathregex.replacement=/bar/$$1"

  # Remove prefix
  - "traefik.http.middlewares.test-stripprefix.stripprefix.prefixes=/foobar,/fiibar"
  - "traefik.http.middlewares.example.stripprefix.forceSlash=false"  # recommended

  # Remove prefix, regexp
  - "traefik.http.middlewares.test-stripprefixregex.stripprefixregex.regex=/foo/[a-z0-9]+/[0-9]+/"
```

Limiting requests:

```yaml
labels:
  # CircuitBreaker: protect from cascading failures
  - "traefik.http.middlewares.latency-check.circuitbreaker.expression=LatencyAtQuantileMS(50.0) > 100"

  # Limit the number of simultaneous requests from a common source, reject with 429 Too Many Requests
  # See: `sourceCriterion` to define what a source is
  - "traefik.http.middlewares.test-inflightreq.inflightreq.amount=10"

  # Rate limit: limit the number of requests going to a service from a given source
  # Example: allow 100 rps, with an additional burst of 50 rps being ok
  # See: `sourceCriterion` to define what a source is
  - "traefik.http.middlewares.test-ratelimit.ratelimit.average=100"
  - "traefik.http.middlewares.test-ratelimit.ratelimit.period=1s"
  - "traefik.http.middlewares.test-ratelimit.ratelimit.burst=50"
```

Headers. See <https://doc.traefik.io/traefik/middlewares/http/headers/>

```yaml
# customRequestHeaders: Set custom headers on request/response
labels:
  - "traefik.http.middlewares.testHeader.headers.customrequestheaders.X-Script-Name=test"
  - "traefik.http.middlewares.testHeader.headers.customresponseheaders.X-Custom-Response-Header=value"
  # Remove
  - "traefik.http.middlewares.testheader.headers.customresponseheaders.X-Custom-Response-Header="

# Easy CORS headers
labels:
  - "traefik.http.middlewares.testheader.headers.accesscontrolallowmethods=GET,OPTIONS,PUT"
  - "traefik.http.middlewares.testheader.headers.accesscontrolalloworiginlist=https://foo.bar.org,https://example.org"
  - "traefik.http.middlewares.testheader.headers.accesscontrolmaxage=100"
  - "traefik.http.middlewares.testheader.headers.addvaryheader=true"

# Easy security headers
labels:
  - "traefik.http.middlewares.testHeader.headers.framedeny=true"
  - "traefik.http.middlewares.testHeader.headers.browserxssfilter=true"

# A lot more! See: https://doc.traefik.io/traefik/middlewares/http/headers/
```

Authentication:

```yaml
  # Basic authentication
  # Format: "user:hash,user:hash"
  # Generate with:
  #   $ htpasswd -nB user | sed -e s/\\$/\\$\\$/g
  - "traefik.http.middlewares.test-auth.basicauth.users=test:$$apr1$$.../,test2:$$apr1$$..."
  # Or use users file: same user:hash, but in a file
  - "traefik.http.middlewares.test-auth.basicauth.usersfile=/path/to/my/usersfile"
  # Optional: realm name
  - "traefik.http.middlewares.test-auth.basicauth.realm=MyRealm"
  # Optional: store authenticated users into a header
  - "traefik.http.middlewares.my-auth.basicauth.headerField=X-WebAuth-User"

  # Digest authentication.
  # Use `htdigest` to generate passwords.
  # Format: "name:realm:hash"
  - "traefik.http.middlewares.test-auth.digestauth.users=test:traefik:...,test2:traefik:..."
  # Or use a file:
  - "traefik.http.middlewares.test-auth.digestauth.usersfile=/path/to/my/usersfile"
  # Store user into a header
  - "traefik.http.middlewares.my-auth.digestauth.headerField=X-WebAuth-User"

  # Forward authentication: delegate authentication to an external service.
  # If the service responds with a 2xx, the original requests is performed. 
  # Otherwise, the response from the authentication server is returned.
  # The target gets original request as X-Forwarded-{For,Proto,Host,Method,Uri} headers.
  - "traefik.http.middlewares.test-auth.forwardauth.address=https://example.com/auth"
  # List of headers to copy from the auth server's response
  - "traefik.http.middlewares.test-auth.forwardauth.authResponseHeaders=X-Auth-User, X-Secret"
  - "traefik.http.middlewares.test-auth.forwardauth.authResponseHeadersRegex=^X-"

  # IP whitelist
  - "traefik.http.middlewares.test-ipwhitelist.ipwhitelist.sourcerange=127.0.0.1/32, 192.168.1.7"

```

Chain: define a reusable chain

```yaml
http:
  routers:
    router1:
      service: service1
      # Just one name
      middlewares:
        - secured
      rule: "Host(`mydomain`)"

  middlewares:
    # Define a chain
    secured:
      chain:
        middlewares:
          - https-only
          - known-ips
          - auth-users

    # Chain links
    auth-users:
      basicAuth:
        users:
          - "test:$apr1$H6uskkkW$IgXLP6ewTrSuBkTrqE8wj/"

    https-only:
      redirectScheme:
        scheme: https

    known-ips:
      ipWhiteList:
        sourceRange:
          - "192.168.1.7"
          - "127.0.0.1/32"

  services:
    service1:
      loadBalancer:
        servers:
          - url: "http://127.0.0.1:80"
```

or with Docker:

```yaml
labels:
  - "traefik.http.routers.router1.service=service1"
  - "traefik.http.routers.router1.middlewares=secured"
  - "traefik.http.routers.router1.rule=Host(`mydomain`)"
  - "traefik.http.middlewares.secured.chain.middlewares=https-only,known-ips,auth-users"
  - "traefik.http.middlewares.auth-users.basicauth.users=test:$apr1$H6uskkkW$IgXLP6ewTrSuBkTrqE8wj/"
  - "traefik.http.middlewares.https-only.redirectscheme.scheme=https"
  - "traefik.http.middlewares.known-ips.ipwhitelist.sourceRange=192.168.1.7,127.0.0.1/32"
  - "traefik.http.services.service1.loadbalancer.server.port=80
```

TCP middleware:

```yaml
labels:
  # Limit the numeber of simultaneous connections
  - "traefik.tcp.middlewares.test-inflightconn.inflightconn.amount=10"
  
  # IP whitelist
  - "traefik.tcp.middlewares.test-ipwhitelist.ipwhitelist.sourcerange=127.0.0.1/32, 192.168.1.7"
```




Recipes
=======

CatchAll rule that will give a 503 when no rule matches

```yaml
# traefik.yaml
entrypoints:
  web:
    address: :80

providers:
  file:
    filename: dynamic.yaml
```

Dynamic:

```yaml
# dynamic.yaml

http:
  routers:
    catchall:
      # attached only to web entryPoint
      entryPoints:
        - "web"
      # catchall rule
      rule: "PathPrefix(`/`)"
      service: unavailable
      # lowest possible priority
      # evaluated when no other router is matched
      priority: 1

  services:
    # Service that will always answer a 503 Service Unavailable response
    unavailable:
      loadBalancer:
        servers: {}
```























