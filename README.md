# Sprockets - Damage Control Toolkit for SoC Engineers

Sprockets intends to be a python project written with simplicity in mind. 
It is to be used as a damage control unit for webhooks which were processed
but not dropped into Redis due to Redis failures or helping Redis 
reconstruct itself entirely without the availability of an `append-only` 
LOG file by querying the PostgreSQL database. 

## Requirements : Redis Reconstruction
If we were to lose Redis tomorrow, we would have to reconstruct the entire 
in-memory database by checking the damage and repairing it by comparing 
against the data available in PostgreSQL (Neon). In order to do that we 
would need the following things :

1. Get access to all user profiles and their bounty amounts for 
reconstruction of the `sorted-set` that holds the entire leaderboard
2. Get access to all solutions accepted and recreate the `sorted-sets`
which hold individual data for highest number of solutions for 
individual langauge badges.

Unfortunately, if there are failed webhooks then we would require some 
storage mechanism where all the failed webhook events are being dropped
for retrying later on. This is necessary because we might get notified at
a fairly delayed time of things gone broken. (Despite having Prometheus,
Grafana setup on the servers for observability and notifications)

SQLite seems like a good backup mechanism for tasks like these as we can 
have separate tables from different jobs which would mimic the schema of 
different Redis streams. As it is a single `.sqlite` file, it becomes a 
good local-first mechanism for storing stuff. Shipping SQLite as 
a storage-during-failure unit with the webhook manager (Alfred) seems to be a 
viable option because of its low-memory footprint and is essentially writing
to a file via a single-threaded file-descriptor.

If that mecahnism fails too then the only way to handle it is by retrying
failed webhooks from GitHub itself.

Hence, 
3. Read the `.sqlite` file, rebuild all the redis-streams with proper access 
control and then drop in all the data into them.
