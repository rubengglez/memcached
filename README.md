# The Challenge - Building Your Own Memcached

Memcached supports a client-server architecture with each client aware of all of the servers, but the servers are unaware of each other and do not communicate.

If a client wishes to set or read the value corresponding to a certain key, the client's library first computes a hash of the key to determine which server to use. This provides a simple form of sharding and creates a highly-scalable shared-nothing architecture across the Memcached servers.

When the server receives a request it computes a second hash of the key to determine where to store/read the corresponding value. Values are stored in RAM; if a server runs out of RAM, it discards the oldest values. Memcached is as a transitory cache; client cannot assume that the data in still in cache when they need it.
