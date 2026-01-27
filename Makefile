all:
	rm -rf dist
	uv build --sdist

publish: all
	uv publish

sync:
	uv sync --reinstall

inspect: all
	rm -rf gxhash-*/
	tar -xvf dist/*.tar.gz

clean:
	rm -rf dist target .venv

benchmark:
	cd bench && uv run --reinstall --no-dev --locked bench && cargo run --locked

perf:
	cargo bench --locked

pre-commit:
	uv run prek install

test:
	uv run prek run

smoke-test: all
	uv run --reinstall --isolated --no-project --with dist/gxhash-*.tar.gz tests/smoke_test.py
