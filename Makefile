
run-app:
	cd frontend && python pyserver.py

run-backend:
	cd backend && cargo run ..\data\wines.json

build:
	cd backend && cargo build