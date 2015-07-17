build:
	cargo build --release
	go build main.go

clean:
	cargo clean
	rm -f "./main"
