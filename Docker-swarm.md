Docker Swarm
============

A swarm consists of multiple Docker hosts ("Nodes") which run in swarm mode and act as:

* managers: to manage membership and delegation; and/or
* workers: which run swarm services.

While swarm services are running on a host, you can still run standalone containers.

When you create a service, you define its optimal state: number of replicas, network/storage resources, ports, etc.
Docker works to maintain that desired state.
For instance, if a worker node becomes unavailable, Docker schedules that node’s tasks on other nodes.
A *task* is a running container which is part of a swarm service and is managed by a swarm manager, as opposed to a standalone container.

To deploy an application, you submit a service definition to a *manager node*. It will dispatch tasks to worker nodes.
Manager nodes elect a single leader and perform the orchestration.

*Worker nodes* receive an execute tasks.
By default, management nodes also run as worker nodes, but you can choose to have manager-only nodes.

Services can be:

* replicated: the swarm manager distributes a specific number of replica tasks among nodes;
* global services: `--mode=global`: the swarm runs one task for the service on every available node of the cluster.
  Good candidates for global services are monitoring agents, anti-virus scanners or other types of
  containers that you want to run on every node in the swarm.

The swarm manager uses *ingress load balancing* to expose the services.
It routes ingress connections to a running task instance.

Swarm mode has an internal DNS component that automatically assigns each service a DNS entry.

Recommendations:

* Use odd number of manager nodes: 3, 5, .... Raft consensus works best.
* A 3-manager swarm tolerates a max loss of 1 manager. A 5-manager swarm can survive 2 managers being down.
  The rule is: an N-manager cluster tolerates at most `(N-1)/2` lost managers.
* Important Note: Adding more managers does NOT mean increased scalability or higher performance. In general, the opposite is true.
* Adding worker nodes increase capacity.
* Adding manager nodes increase fault-tolerance. They perform the orchestration.
*

## Tutorial

You need three machines: 1 manager + 2 workers.
Because worker needs to connect to the manager, it needs a fixed IP.

The following ports must be available: `2377` TCP, `7946` TCP/UDP, `4789` UDP (VXLAN overlay network).

WARNING: Make sure that no untrusted traffic can reach `4789`: VXLAN does not provide authentication!
Only open it to a trusted network. If the network is not fully trusted, create an encrypted overlay network!

Create a swarm: SSH into the manager and:

```console
$ ip -brief addr
ens3             UP             176.124.199.2/32

$ docker swarm init --advertise-addr 176.124.199.
Swarm initialized: current node (xy9cbc6137r30eksoc7oib112) is now a manager.

To add a worker to this swarm, run the following command:

    docker swarm join --token SWMTKN-1-3b3tqbh9wdk0xmgmkds4yyzsy0l69d5v51fwwn061zkvrjayfi-5345nxgm6477qgmd3rfvtliq7 176.124.199.2:2377

To add a manager to this swarm, run 'docker swarm join-token manager' and follow the instructions.

$ docker info
Swarm: active
  NodeID: dxn1zf6l61qsb1josjja83ngz
  Is Manager: true
  Managers: 1
  Nodes: 1
  ...

$ docker node ls                                                                                                                                                                                                                                                                                                                                                                ID                            HOSTNAME              STATUS    AVAILABILITY   MANAGER STATUS   ENGINE VERSION
gbzq4kd8q8pkicgkhcbklqet4 *   Rocket.aeza.network   Ready     Active         Leader           20.10.21
```

This node is now the leader of the swarm and its manager.
It also generated a self-signed root CA for the swarm's autnetication.

Execute the command on worker nodes (or get the token again using `$ docker swarm join-token worker`).

Deploy a service:

```console
$ docker service create --replicas 1 --name whoami traefik/whoami

$ docker service ls
ID             NAME      MODE         REPLICAS   IMAGE                   PORTS
l30f1r7q1b47   whoami    replicated   1/1        traefik/whoami:latest

$ docker service inspect --pretty whoami
ID:             l30f1r7q1b47jh8m05g3ykswi
Name:           whoami
Service Mode:   Replicated
 Replicas:      1

$ docker service ps whoami
ID             NAME       IMAGE                   NODE                  DESIRED STATE   CURRENT STATE            ERROR     PORTS
rvpyt6sczgsv   whoami.1   traefik/whoami:latest   Rocket.aeza.network   Running         Running 44 seconds ago
```

Scale it:

```console
$ docker service scale whoami=3

$ docker service ps whoami
ID             NAME       IMAGE                   NODE                  DESIRED STATE   CURRENT STATE            ERROR     PORTS
rvpyt6sczgsv   whoami.1   traefik/whoami:latest   Rocket.aeza.network   Running         Running 2 minutes ago
r6ki5l2rop4i   whoami.2   traefik/whoami:latest   Rocket.aeza.network   Running         Running 10 seconds ago
iht94ml67t14   whoami.3   traefik/whoami:latest   Rocket.aeza.network   Running         Running 10 seconds ago
```

Delete the service:

```console
$ docker service rm whoami
```

Rolling updates: use `--update-delay 30s` to configure the swarm to take down services one by one with 30s delay when updating.
The scheduler will stop the first task, update, start it. If it is `RUNNING`, it will wait, and proceed.

Use `$ docker service ps <id>` to watch the rolling update:

If any container is `FAILED`, the update is paused. Use `$ docker service inspect` to investigate, `$ docker service update` to proceed.

```console
$ docker service create --replicas 3 --name redis --update-delay 10s redis:3.0.6
$ docker service update --image redis:3.0.7 redis
$ watch docker service ps redis
```

## Drain a Node

Drain a node: set it to `DRAIN` availability that prevents it from receiving new tasks.
Also, all tasks will be rescheduled to other nodes.

```console
$ docker node update --availability drain worker1
$ docker node inspect --pretty worker1
ID:             38ciaotwjuritcdtn9npbnkuz
Hostname:       worker1
Status:
 State:	        Ready
 Availability:  Drain
...snip...
```

now it can be taken down for maintenance.
And brought back:

```console
$ docker node update --availability active worker1
```

## Routing Mesh

All nodes participate in an ingress *routing mesh*. It enables *each* node in the swarm to accept connections on published ports
for *any service*: even if there's no task running on the node.

To use the ingress network, you need the following ports open between the swarm nodes:
`7946` TCP/UDP for container discovery, and `4789` UDP for the container ingress network.
Also, the published port must be open and available.

To publish a port:

```console
$ docker service create --replicas 3 --name whoami --publish published=8000,target=80 traefik/whoami
// OR
$ docker service update --publish-add published=8000,target=80 whoami
```

now, open `:8000` on any node, and it's routed to an active container.
The internal port, 80, does not have to be published: the routing mesh knows how to route traffic to it.

Find out the port number:

```console
$ docker service inspect --format="{{json .Endpoint.Spec.Ports}}" whoami                                                                                                                                                                                                                                                                                                        [{"Protocol":"tcp","TargetPort":80,"PublishedPort":8000,"PublishMode":"ingress"}]
```

You can **bypass the routing mesh** by accessing the bound port on a given node:
this way you always access the instance of the service running on that node.

NOTE: if you run multiple service tasks on one node (e.g. when 10 replicas are run on 5 nodes),
you cannot specify a static target port. Either allow Docker to assign a random port,
or use a "global service", or use placement constraints.

Another way to bypass the routing mesh and publish a port:

```console
$ docker service create --name dns-cache \
  --publish published=53,target=53,protocol=udp,mode=host \
  --mode global \
  dns-cache
```

## External Load Balancer

You can configure an external load balancer to keep the rest of the network private.
For instance, use a HAProxy to balance requests to internal swarm load balancers.

```
// haproxy.cfg
global
        log /dev/log    local0
        log /dev/log    local1 notice
...snip...

# Configure HAProxy to listen on port 80
frontend http_front
   bind *:80
   stats uri /haproxy?stats
   default_backend http_back

# Configure HAProxy to route requests to swarm nodes on port 8080
backend http_back
   balance roundrobin
   server node1 192.168.99.100:8080 check
   server node2 192.168.99.101:8080 check
   server node3 192.168.99.102:8080 check
```


## Labels

Add metadata to nodes: label name, or `key=value`:

```console
$ docker node update --label-add foo --label-add bar=baz node-1
```

Node labels can be used to limit critical tasks to nodes that meet certain requirements.



## Notes

* If you want to prevent a service from being deployed, use `scale=0`
* You can reserve a specific amount of memory for a service. It will remain "pending" until there's a node with enough memory.
* You can impose placement constraints on the service



# Deploy Services

Swarm uses a *declarative model*: you define the desired state of the service, and reply upon Docker to maintain this state.
For instance: image name, number of replicas, exposed ports, etc.

To create a service from a private registry:

```console
$ docker service create --replicas 1 --name whoami traefik/whoami
```

with env variables and command:

```console
$ docker service create --replicas 1 --name whoami --env NAME=value --args "..." traefik/whoami
```

create a service using an image on a private registry:

```console
$ docker login registry.example.com
$ docker service  create --with-registry-auth --name my_service registry.example.com/acme/my_image:latest
```

to deploy a global service: i.e. that runs on every available node, exactly one instance per node:

```console
$ docker service create --mode global --publish mode=host,target=80,published=8080 --name=nginx nginx:latest
```

to modify a service:

```console
$ dockser service update --publish-add 80 whoami
```

## Connect the service to an overlay network

You can use overlay networks to connect services within the swarm.
Create a network and connect a service to the overlay network:

```console
$ docker network create --driver overlay my-network
$ docker service create --network my-network
```

The network is available on every node in the swarm.

Connect new nodes to the network:

```console
$ docker service update --network-add my-network my-web
```

## Reserve memory or CPUs for a service

```console
$ docker service create --reserve-memory=... --reserve-cpu=...
```

## Placement constraints

Use placement constraints to control the nodes a service can be assigned to.

This service runs only on nodes with the label "region" set to "east":

```console
$ docker service create --constraint node.labels.region==east ...
```

Operators: `==` and `!=`.

If you specify multiple constraints, the service is only deployed noto nodes where they are all met.

## Placement preferences

While placement constraints limit hte nodes a service can run on,
*placement preferences* try to place task on appropriate nodes, but will use other nodes when necessary.
This way, if you lose some servers, the service is still running on nodes on other racks.

This service is deployed as evenly as possible across the two sets of nodes, per datacenter:

```console
$ docker service create --replicas=9 --placement-pref 'spread=node.labels.datacenter' ...
```

Nodes that do not have a `datacenter` label will also form a group and receive tasks in equal proportion.

You can specify multiple placement preferences, and they are processed in the order they are encountered.

Here,  Tasks are spread first over the various datacenters, and then over racks (as indicated by the respective labels):

```console
$ docker service create --replicas 9 \
  --placement-pref 'spread=node.labels.datacenter' \
  --placement-pref 'spread=node.labels.rack' \
  redis:3.0.6
```

## Service Update Behavior

The `--update-delay` configures the time delay between updates to a service task or sets of tasks.

By default, the scheduler updates 1 task at a time.
Use `--update-parallelism=2` to update multiple service tasks at a time.

```console
$ docker service create \
  --replicas 10 \
  --name my_web \
  --update-delay 10s \
  --update-parallelism 2 \
  --update-failure-action continue \
  alpine
```

The `--update-max-failure-ratio` flag controls what fraction of tasks can fail during an update
before the update as a whole is considered to have failed.
For example, with `--update-max-failure-ratio 0.1 --update-failure-action pause`,
after 10% of the tasks being updated fail, the update is paused.

An individual task update is considered to have failed if the task doesn’t start up, or if it stops
running within `--update-monitor=30s` flag.

## Roll back to the previous state of a service

In case the updated version of the service fails, roll back to the previous version of the service:

```console
$ docker service update --rollback --update-delay 0s
```

You can configure the service to roll back automatically.

* `--rollback-delay=0s`: wait this much time after rolling back a task before rolling back the next one
* `--rollback-failure-action=pause`: when a task fails to roll back, do this: `pause` or `continue`
* `--rollback-max-failure-ratio=0`: the failure rate to tolerate. Example: `.2` means tolerate 2/10 tasks failing
* `--rollback-monitor=5s`: if a task stops before this time period, consider it failed
* `--rollback-parallelism=1`: roll back this many tasks at a time

Example: roll back this "redis" automatically if it fails to deploy: two tasks at a time, monitor them for 20s,
tolerate at most 20% failure ratio:

```console
$ docker service create --name=my_redis \
                        --replicas=5 \
                        --rollback-parallelism=2 \
                        --rollback-monitor=20s \
                        --rollback-max-failure-ratio=.2 \
                        redis:latest
```

## Volumes

Volumes outlive containers. Use for important data.

Volumes can be created before deploying services.
Or, if a volume does not exist on a particular node when the container starts, they are created automatically
according to the volume spec of a service.

To use an existing volume:

```console
$ docker service create \
  --mount src=<VOLUME-NAME>,dst=<CONTAINER-PATH> \
  --name myservice \
  <IMAGE>
```

The default volume driver is `local`.
Also supported: nfs, cloud object storage, S3, NFS, sshfs, brtfs & zfs (with snapshots), ...
Install as Docker plugins.

## Templates

You can use Go placeholders:

* `.Service.ID`	Service ID
* `.Service.Name`	Service name
* `.Service.Labels`	Service labels
* `.Node.ID	Node` ID
* `.Node.Hostname`	Node hostname
* `.Task.Name`	Task name
* `.Task.Slot`	Task slot


in the following flags:

* `--hostname`
* `--mount`
* `--env`








# Deploy a Stack (docker-compose)

