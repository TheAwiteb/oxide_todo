# RESTful Todo
RESTful Todo API with [Actix](https://actix.rs) and [SeaORM](https://www.sea-ql.org/). Documented by [swagger-ui](https://swagger.io/). Authenticated by [JWT](https://jwt.io/). Rate limited by [actix_extensible_rate_limit](https://crates.io/crates/actix_extensible_rate_limit). Tested by [rstest](https://crates.io/crates/rstest). Have fun! 😁
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)

### Usage
Clone the repository and run the following commands:
```bash
cargo run
```
Just like that, you have a RESTful API running on your machine.
If you want to see the logs, you can run the following command:
```bash
RUST_LOG=debug cargo run
```

### Documentation
- The API documentation is available at `{HOST}:{PORT}/docs/swagger/` (default: [http://localhost:8080/docs/swagger](http://localhost:8080/docs/swagger/))
- The OpenAPI specification is available at `{HOST}:{PORT}/docs/openapi.json` (default: [http://localhost:8080/docs/openapi.json](http://localhost:8080/docs/openapi.json)) 

### Environment variables
Rename the `.env.example` file to `.env` and change the values to your needs. Empty default means that the variable is required.
<!-- Table of enviroment variables -->
| Name | Description | Default |
| --- | --- | --- |
| `DATABASE_URL` | The database url | `sqlite://db.sqlite3` |
| `SECRET_KEY` | The secret key for JWT | ` ` |
| `HOST` | The host to bind | `localhost` |
| `PORT` | The port to run the server | `8080` |
| `RATE_LIMIT_BURST_SIZE` | The burst size for rate limiter | `30` |
| `RATE_LIMIT_PER_SECOND` | The time to reset the burst | `60` |
| `API_CONTACT_NAME` | The name of the API contact | ` ` |
| `API_CONTACT_URL` | The url of the API contact | ` ` |
| `API_CONTACT_EMAIL` | The email of the API contact | ` ` |
| `API_TITLE` | The title of the API | `RESTful Todo API documentation` |

### Testing
#### Prerequisites
- [dotenv cli](https://pypi.org/project/python-dotenv/)
- [just](https://github.com/casey/just)
```bash
just tests
```
### Development
For development you need to install [just](https://github.com/casey/just) and [dotenv cli](https://pypi.org/project/python-dotenv/).
With `just` you can run all needed commands with one command, type `just` folloing by the command you want to run.<br>
Available commands:
- `just build` to build the RESTful API
- `just ci` to run the CI
- `just fmt` to format everything
- `just fmt-check` to check the format of everything
- `just linter` to run Rust linter (clippy)
- `just tests` to run the tests

## Soon
- [X] Swagger UI for API documentation
- [X] Rate limiting
- [ ] CI with Github Actions
- [X] Unit tests
- [ ] Dockerize the server
- [X] JustFile for easy setup, useing [just](https://github.com/casey/just)

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
