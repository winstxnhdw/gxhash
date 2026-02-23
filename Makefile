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

benchmark:
	cd bench && sudo nice -n -20 uv run --reinstall --no-cache --no-dev --locked bench && cargo run --locked

performance:
	cargo bench --locked

pre-commit:
	uv run prek install

test:
	uv run prek run
