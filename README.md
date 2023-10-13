# Lunar

I opted for a NoSQL approach, since I interpreted the constraints on the `/configuration` endpoint as a requirement for a dynamic schema. I've never worked with unstructured data before (except for my desk, flat etc), so this was all new territory. But I relish learning new stuff, so it was very enjoyable, although the majority of debugging consisted of hacking away at my code for ages only to find out that my curl requests used the wrong port and similar witch hunts. On an even more positive note, this means I understood the concepts reasonably well (aka super good enough) and the code actually worked pretty much from the start.

## Actual implementation

- `MongoDB` + `actix-web`
- DB structure: there's a `moonbatteries` database containing a `moon` collection for registered batteries. Can be seen in `mongodb/init-mongo.js`.
- *It worrks on my machine!* Everything should hopefully be self-contained. There's a docker-compose file, I recommend using docker to set up `MongoDB` and you will also need a `.env` file with your target URL.


### What's missing?

- I didn't do a whole lot of error handling/checking
- `actix-web` provides a lot more configuration, I didn't use any of it for this project
- endpoints:
    - `/register`: No validation, you could provide any limerick as a MAC address
    - `/ping`: I didn't overthink edge cases, so there might be room for error, apart from that, it works
    - `/configuration`: same as the others, in addition, I used a very primitive setup DB-wise, so there's no elegant way to update configuration KV pairs. Right now, you can push new KV pairs alright, but duplicates are possible. There are various remedies, I thought about using a seperate `configData` collection and associate each KV with a `moon` object
- proper authentication: I'd use probably use JWT




