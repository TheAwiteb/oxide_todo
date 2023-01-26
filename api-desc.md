## What it's?
<!-- What it is the RESTful todo api -->
Simple TODO API to CRUDS (Create, Read, Update, Searsh) the users todos, each user have unique token.

## Authentication
<!-- How to authenticate -->
In this API we use JWT (JSON Web Token) to authenticate the users, the token is generated when the user is created and is returned in the response. The token is used to authenticate the user in the other endpoints.<br>
The token is sent in the header of the request in the format `Authorization: Bearer <token>`.<br>
The token doesn't expire, but can be revoked, if the token is revoked the user will not be able to use it anymore, you can revoke the token in the `/api/auth/revoke` endpoint.

### Note (for the endpoints that need authentication)

- Set `Authorization` header in the request with the token, else will return `400 Bad Request`.
- The token should start with `Bearer ` and then the token, else will return `400 Bad Request`.
- The token should be valid, else will return `401 Unauthorized`.
- The token should not be revoked, else will return `403 Forbidden`.

## Rate Limit
<!-- How the ratelimit work in the API -->
The API has a rate limit of 30 burst requests per minute, if you exceed the limit will return `429 Too Many Requests`. 

### Headers
<!-- The ratelimit headers -->

- `x-ratelimit-limit`: Your burst size, maximum number of requests you can make in a burst
- `x-ratelimit-remaining`: The requests remaining in the current burst, when it reaches 0 the next request will return `429 Too Many Requests`.
- `x-ratelimit-reset`: The time in seconds when the current burst will be reset.
