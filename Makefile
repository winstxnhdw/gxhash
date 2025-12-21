all:
	rm -rf dist
	uv run maturin sdist --out dist

publish: all
	uv publish

sync:
	uv sync --reinstall

inspect: all
	rm -rf gxhash-*/
	tar -xvf dist/*.tar.gz

clean:
	rm -rf dist target .venv
