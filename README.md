# The REST API used for the Oláh Barbershop [app](https://github.com/Olah-Barbershop/app) and [website](https://github.com/Olah-Barbershop/website)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) &nbsp;
[![GitHub Release](https://img.shields.io/github/v/release/Olah-Barbershop/API)](https://github.com/Olah-Barbershop/API/releases/latest) &nbsp;

## Documentation
Check out the API documentation [here](DOCUMENTATION.md)

## Instructions
Clone the repository and install the dependencies
```sh
git clone https://github.com/Olah-Barbershop/API.git
cd API
cargo build
```
Add your MongoDB url and database name to a `.env` file
```env
DATABASE_URL=[your MongoDB url]
DATABASE_NAME=[the name of your database]
```

(Optional) Change the port in `Rocket.toml` if needed

You'll need an SSL certificate. Add your certificate to `certs.pem` and you private key to `key.pem`.

Start the server
```sh
cargo run
```
