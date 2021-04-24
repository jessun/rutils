DOCKER_IMAGE    = ghcr.io/jessun/rust/rust-compiler
override DOCKER = $(shell which docker)

docker_test: 
	CTN_NAME="test_$$RANDOM" && \
	$(DOCKER) run -d --entrypoint /sbin/init --privileged --name $${CTN_NAME} -v $(shell pwd)/:/app --rm -w /app $(DOCKER_IMAGE) && \
	$(DOCKER) exec $${CTN_NAME} make test ; ret=$$? ; \
	$(DOCKER) exec $${CTN_NAME} make clean ;  \
	$(DOCKER) stop $${CTN_NAME} && (( ! ret ))

run_docker:
	CTN_NAME="test_$$RANDOM" && \
	$(DOCKER) run -d --entrypoint /sbin/init --privileged --name $${CTN_NAME} -v $(shell pwd)/:/app --rm -w /app $(DOCKER_IMAGE) 

# =============================================================================

run:
	RUST_BACKTRACE=full RUST_LOG=debug cargo run

test:
	RUST_BACKTRACE=full RUST_LOG=info cargo test -- --nocapture

clean:
	cargo clean

