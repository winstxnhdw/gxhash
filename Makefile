all:
	rm -rf dist
	uv build --sdist

publish: all
	uv publish

inspect: all
	rm -rf gxhash-*/
	tar -xvf dist/*.tar.gz

smoke-test: all
	uv run --reinstall --no-cache --isolated --no-project --with dist/gxhash-*.tar.gz tests/smoke_test.py

sync:
	uv sync --reinstall

clean:
	rm -rf dist target .venv
	rm -rf bench/target bench/.venv

benchmark:
	cd bench && sudo nice -n -20 ionice -c 1 -n 0 su -c "uv run --reinstall --refresh --no-dev --locked bench" $$(whoami) && cargo run --locked

performance:
	cargo bench --locked --manifest-path perf/Cargo.toml

pre-commit:
	uv run prek install -t pre-commit -t commit-msg

test:
	uv run prek run
