# LB Rodeo

This is a client and server meant to sit on both sides of a Load Balancer.
lbrodeo's main purpose is not formal verification of the LB's correctness or
performance, but to give some insight into the inner workings of the LB.

Right now there are two main components, a client and a server. The client could talk directly to one instance of a server, but what's the fun in that? Instead you should put a load balancer between the two.

