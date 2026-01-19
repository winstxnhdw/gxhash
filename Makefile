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
