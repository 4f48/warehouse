Warehouse is a lightweight object storage database, with a built-in web panel for quick uploads. Warehouse exposes a simple Rust API; for reference, see [panel.html](https://github.com/4f48/warehouse/blob/main/static/panel.html) and [index.js](https://github.com/4f48/warehouse/blob/main/static/index.js). The Warehouse web panel is made with 100% dependency-free HTML, JavaScript, and CSS.

## Deploying
Hosting Warehouse for yourself is recommended. It is very easy to get started. Either download a pre-built binary from GitHub Releases, or build it yourself by installing the Rust toolchain using [rustup](https://rustup.rs/), cloning the repository, and running `cargo build -r` for an optimized build. No external dependencies are needed. It is recommended that you run Warehouse behind a reverse proxy, like Caddy. For a secure connection, use HTTPS.

## Live demo
You can try Warehouse on [warehouse.4f48.dev](https://warehouse.4f48.dev). This is not an official instance, just a demo, thus it will be taken down once Hack Club High Seas ends. Do not upload personal or sensitive documents, since Warehouse's storage is not encrypted. Use the following secret key on the web panel:
```
pShiFCIGClV0/VScvsWivcRJ5mInV7QhPHDBsON5clD8YY0UDi+qqf2WiVgAwH43GAkA8gyAewcAaUII+Dt6DQ
```

## License
Warehouse is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, version 3 of the License only.

Warehouse is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with Warehouse. If not, see <https://www.gnu.org/licenses/>.
