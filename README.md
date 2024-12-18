# Traefik Dynamic Configuration Manager

A Rust library for managing Traefik dynamic configuration through etcd.

## Installation

Head to [https://auser.github.io/traefikctl/](https://auser.github.io/traefikctl/) for installation instructions.

## Configuration

The configuration is done in the `config/config.yml` file. You can also pass in a partial etcd config via the cli to override the default config.

```
traefikctl get -f ./config/config-devcontainer.yml --etcd-config='{"endpoints": ["https://0.0.0.0:2379"], "tls": {"cert": "./config/tls/etcd-peer.pem", "key": "./config/tls/etcd-peer-key.pem", "ca": "./config/tls/ca.pem", "domain": "etcd"}}'
```

### Hosts

Each host has a domain, a list of paths, and a list of deployments.

#### Paths

Each path has a path, a list of deployments, a list of middlewares, and a boolean to strip the prefix. The deployments are keyed by the deployment name, which is used to determine which router to use.

#### Deployments

Each deployment has an ip, a port, a weight, and a boolean to determine if the cookie should be passed through.

It can also have a list of weights for each deployment.

**The root of the project are deployments.** Every deployment will create a router in Traefik as well as a service. You can configure the deployment to handle [Traefik](https://doc.traefik.io/traefik) routes as well as `Kubernetes` routes. 

## Features

- Strongly typed configuration using Rust structs that are automatically exported to TypeScript
- Support for blue/green deployments with weighted load balancing
- Middleware configuration for headers, TLS, and more
- Host and path-based routing
- Integration with etcd key-value store

## Configuration Example

The configuration is defined in YAML format. Here's an example:

```yaml
etcd:
  endpoints: ["https://0.0.0.0:2379"]
  timeout: 2000
  keep_alive: 300
  tls:
    cert: "./config/tls/etcd-peer.pem"
    key: "./config/tls/etcd-peer-key.pem"
    ca: "./config/tls/ca.pem"
    domain: herringbank.com

middlewares:
  enable-headers:
    headers:
      custom_request_headers:
        X-Forwarded-Proto: "https"
        X-Forwarded-Port: "443"
        Location: ""
      custom_response_headers:
        Location: ""
      access_control_allow_methods:
        - "GET"
      access_control_allow_headers:
        - "Content-Type"
      access_control_expose_headers:
        - Location
      add_vary_header: true

hosts:
  - domain: "example.com"
    www_redirect: true
    paths:
      - path: "/test"
        deployments:
          blue:
            ip: 10.0.0.1
            port: 8080
            weight: 50
          green:
            ip: 10.0.0.2
            port: 8080
            weight: 50
        middlewares:  
          - enable-headers
          - forward-server

    # Root path (catch-all)
    deployments:
      blue:
        ip: 10.0.0.1
        port: 8080
        weight: 100
```

### Connecting to etcd

You can connect to etcd using a TLS certificate, or over an ssh tunnel. The `endpoints` field in the config file should be a list of all the etcd endpoints you want to connect to. If you are connecting over tls, you will need to provide the cert, key, and ca files. as the `tls` field.

### Middleware Configuration

Middlewares are configured in the `middlewares` section. Each middleware has a name, and a set of options that are specific to the middleware. The middleware name is the name of the middleware in Traefik. The middleware name is used to apply the middleware to a path.

### Host Configuration

Hosts are configured in the `hosts` section. Each host has a domain, a list of paths, and a list of deployments. The domain is used to determine which router to use in Traefik. The paths are used to determine which deployments to use for the path.

Without `paths`, you can configure the host to catch all paths. with a root `deployments` section. If you want to configure a specific path, you can do so with the `paths` section.

### Keys in deployments

- `ip` - The ip address of the deployment
- `port` - The port of the deployment
- `weight` - The weight of the deployment
- `protocol` - The protocol to use to connect to the deployment. Defaults to `http` but you can set it to `tls`.

## Running over an ssh tunnel

```
ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -L 2379:0.0.0.0:2379 alerner@proxy
```

## Frontend

The frontend is a simple web app that is used to manage the configuration. It is built with [Svelte](https://svelte.dev/) and [Skeleton](http://getskeleton.com/). 

It is not built with any frameworks in mind, so it could be hosted on any static file server.

## Dev notes

Check the etcd container for keys:

```
# Find the etcd container ID
docker ps --format '{{.ID}} {{.Image}} {{.Names}}' | awk '($2 ~ /docker.io\/bitnami\/etcd/ || $3 ~ /etcd$/) {print $1}'

# Or as a one-liner:
ETCD_ID=$(docker ps --format '{{.ID}} {{.Image}} {{.Names}}' | awk '($3 ~ /etcd/) {print $1}')

# Then use it like:
docker exec -it $ETCD_ID etcdctl get /traefik/config --prefix
# Or as a one-liner:
docker exec -it $(docker ps --format '{{.ID}} {{.Image}} {{.Names}}' | awk '($3 ~ /etcd/) {print $1}') bash

export ecd="/opt/bitnami/etcd/bin/etcdctl --endpoints=https://localhost:2379 --cacert=/etc/etcd/tls/ca.pem --cert=/etc/etcd/tls/server.pem --key=/etc/etcd/tls/server-key.pem"
```
