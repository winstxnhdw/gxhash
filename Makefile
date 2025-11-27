all:
	rm -rf dist
	uv run maturin sdist --out dist

publish:
	uv publish

sync:
	uv sync --reinstall

clean:
	rm -rf dist target .venv
