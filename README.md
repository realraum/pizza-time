# pizza-time

Pizza good :3

## At a glance

A simple client-server web app to coordinate ordering several pizzas in a group setting, where either most if not all people have access to a web client, or just one person manages everything.

### Goals

- Keep track of who wants what
- Keep track of who laid out cash
- Keep track of who spent cash
- Keep track of who got their order
- Be easily usable, clean, and pretty on mobile & desktop
- Best effort: Work with noscript clients
- Free as in Freedom: Licensed under the AGPL
- Future: Separate, but concurrent orders
- Future: Easy to deploy & maintain: single server binary

### Non-goals

- History/statistics
- Persistent data
- Authentication & logins
- Place food orders over APIs
- Handle transactions
- Be written in Node.js or Go
- Support every possible web browsing setup (e.g. retro stuff)

## Development

To easily support both a client-side rendered, API-first approach _and_ noscript-compatibility at the same time,
we opted for the full-stack web framework [Leptos](https://leptos.dev/),
meaning the app is written in [Rust](https://www.rust-lang.org/).
Everything is written in Rust with inline [Tailwind](https://tailwindcss.com/) styling.
Data is kept in-memory, without a database, as it's deliberately not persisted.

Any IDE will work, tho I'm (currently) using VS Code, but I may switch to NeoVim in the future.

Assuming you have [installed Rust](https://rustup.rs/) already, get `cargo leptos` [installed](https://github.com/leptos-rs/cargo-leptos#getting-started), clone this repo down to your machine, cd into it, and run `cargo leptos serve`.
The default address is <http://127.0.0.1:3000>.

I'd highly recommend setting up a Rust LSP and a Tailwind plugin for your editor or IDE.

## Copyright & License

Copyright 2025 Tanja, some rights reserved.

![AGPLv3 license logo](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)

This entire project, consisting of code, styling, and documentation is licensed under the GNU Affero General Public License (either version 3, or, at your option, any later version), as seen in [LICENSE.md](./LICENSE.md), or at <https://www.gnu.org/licenses/agpl-3.0.en.html>.
