# LB Rodeo

This is a client and server meant to sit on both sides of a Load Balancer.
lbrodeo's main purpose is not formal verification of the LB's correctness or
performance, but to give some insight into the inner workings of the LB.

Right now there are two main components, a client and a server. The client
could talk directly to one instance of a server, but what's the fun in that?
Instead you should put a load balancer between the two.

# Server

A simple HTTP server. Future plans include adding pathologies to the server
like random errors, slow responses, or huge responses, things that would cause
a naive balancing algorithm problems.

# Client

Makes repeated requests and checks that those requests are resolved correctly.
I'm still working on the best way to show what the client is thinking, so if
anyone has ideas please let me know.
