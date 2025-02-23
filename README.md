# <img src="./logo.svg" alt="logo" width="32px" height="32px"> FriendVote

### Running the application server

#### prerequisites

- check out the repo, and install the rust toolchain (see https://rustup.rs)
- Add WASM as a compilation target to your Rust toolchain with `rustup target add wasm32-unknown-unknown`
- Install cargo-leptos with `cargo install cargo-leptos`
- set up a PostgreSQL database, and create a database called 'friendvote' in it, and give a user full access to that database.

#### Debug

This method is intended for debugging only, and results in a larger WASM file, making it slower than a release.
Use
```
DATABASE_URL=postgresql://<username>:<password>@localhost/friendvote cargo leptos watch
```
(where `<username>` and `<password>` are the credentials for the postgres user that has read and write access to the `friendvote` database)

#### Release

Either download a pre-compiled release build from github, or build one yourself with `cargo leptos build --release`

After that, the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
friendvote
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="friendvote"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
DATABASE_URL="postgresql://<username>:<password>@localhost/friendvote"
```
Finally, run the server binary.

## Licensing

Copyright © 2025 Simon De Ridder

FriendVote is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

FriendVote is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with FriendVote. If not, see <https://www.gnu.org/licenses/>.
