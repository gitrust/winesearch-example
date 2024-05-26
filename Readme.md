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
│   │   ├── api/             # API route handlers
│   │   ├── models/          # Data models
│   │   └── utils/           # Utility functions
│
└── frontend/                # Frontend code
    ├── index.html           # 
    ├── pyserver.py          # a simple webserver in Python

```