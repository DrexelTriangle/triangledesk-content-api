# Triangledesk Content API
Superdesk's content API is one of its best features, at least in theory, because it allows any frontend to consume Superdesk articles in a sensible and standard way. In practice, however, it's wonky, kind of outdated, and misses some of the main advantages of NewsML. That can be said for most of Superdesk actually. 
This project, a part of the Triangledesk fork of Superdesk, aims to correct that by being a Subscriber Destination that serves ninjs over both REST and GraphQL with rNews linked data schema metadata.
Currently being prototyped and not yet approved for the project by the Triangle IT Directors.

## Infrastructure
- MongoDB stores ninjs+LD items and media files
- Webserver binary that serves all endpoints
  - GraphQL endpoint that resolves queries to json with formulated `@context` at root of response, adjacent to `data`
  - REST API endpoint that serves complete JSON-LD of news items and full media files
  - Reception endpoint where Superdesk pushes published articles, which are then decorated with rNews types and saved to MongoDB, along with media which is downloaded
