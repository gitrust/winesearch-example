
run-app:
	cd frontend && python pyserver.py

run-backend:
	cd backend && cargo run

build:
	cd backend && cargo build