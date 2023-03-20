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








HTTPS & TLS
============




Middlewares
===========











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























