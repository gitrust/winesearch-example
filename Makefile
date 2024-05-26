
run-app:
	cd frontend && python pyserver.py

run-backend:
	cd backend && cargo run

build-backend:
	cd backend && cargo build