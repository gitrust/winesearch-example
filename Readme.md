# Goal

 Create a simple webpage to search for wines using Rust as backend and simple JS/HTML for the frontend.

 ## Backend

 - Rust
 - Rocket.rs as web framework for the REST API

 ## Frontend

 - simple JS and HTML to show a search form and table of wines


 ## Project Structure

 ```
 winesearch/
│
├── backend/                 # Backend Rust code
│   ├── Cargo.toml           # Rust dependencies file
│   ├── src/                 # Source code directory
│   │   ├── main.rs          # Main entry point for the backend
│   │   ├── api/             # API route handlers (TODO)
│   │   ├── models/          # Data models (TODO)
│   │   └── utils/           # Utility functions (TODO)
│
│── frontend/                # Frontend code
|   ├── index.html           # 
|   ├── pyserver.py          # a simple webserver in Python
|
├── data
    ├── wines.json           # Wine list
    ├── wine-generator.py    # A random winelist generator

```

## Requirements

- Install Rust-lang
- Install Python (3.x) for the webserver
- I use a Makefile for convenient short commands in the console

## Build 

- `make build` to build the rust backend

## Run

- start backend by `make run-backend`
- start frontend webserver by `make run-app`
- open `http://localhost:8080/` in Webbrowser
- REST API is available on `http://127.0.0.1:8000`


# Reference

- https://www.rust-lang.org/learn
- https://doc.rust-lang.org/reference/index.html
- https://rocket.rs/guide/v0.5/
- https://crates.io/ to search for crates