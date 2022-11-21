## What it's?
<!-- What it is the RESTful todo api -->
Simple TODO API to CRUDS (Create, Read, Update, Searsh) the users todos, each user have unique token.

## Authentication
<!-- How to authenticate -->
In this API we use JWT (JSON Web Token) to authenticate the users, the token is generated when the user is created and is returned in the response. The token is used to authenticate the user in the other endpoints.<br>
The token is sent in the header of the request in the format `Authorization: Bearer <token>`.<br>
The token doesn't expire, but you can revoke it with the endpoint `/api/auth/revoke`.

### Note (for the endpoints that need authentication)

- Set `Authorization` header in the request with the token, else will return `400 Bad Request`.
- The token should start with `Bearer ` and then the token, else will return `400 Bad Request`.
- The token should be valid, else will return `401 Unauthorized`.
- The token should not be revoked, else will return `403 Forbidden`.

## Rate Limit
<!-- How the ratelimit work in the API -->
The API has a rate limit of 60 burst requests, and 1 request per 5 seconds, if you exceed the limit you will receive a 429 status code.

### Headers
<!-- The ratelimit headers -->

- `X-RateLimit-Limit`: Your burst limit.
- `X-RateLimit-Remaining`: The number of requests remaining in the current burst. Will return `Too Many Requests` if you exceed the limit.
- `X-RateLimit-Reset`: The number of seconds left to add a new request to the burst.
