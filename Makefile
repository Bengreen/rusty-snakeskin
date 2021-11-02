


PYTHON:=venv/bin/python3
PIP:=venv/bin/pip
PYTEST:=venv/bin/pytest
CARGO:=cargo

venv:
	# /opt/homebrew/opt/python@3.10/bin/python3.10 -m venv venv
	/opt/homebrew/bin/python3 -m venv venv
	${PIP} install wheel

list: venv
	${PIP} list

clean:
	@rm -rf venv
	@rm -rf *.egg-info
	@rm -rf .pytest_cache
	find . -iname "*.pyc" -delete
	find . -iname "__pycache__" -delete
	${CARGO} clean

build:
	cargo build

venv/bin/activate.path: venv
	@echo source venv/bin/activate.path to set PYTHONPATH
	@${PYTHON} -c "import sys; print('export PYTHONPATH=', end=''); print(':'.join(sys.path[1:]))" > venv/bin/activate.path

install: venv setup.py build venv/bin/activate.path
	${PIP} install mypy

mypy: venv mypy/setup.py
	${PIP} install -e mypy[dev]

${PYTEST}: venv setup.py
	${PIP} install -e mypy[dev]

dev-install: mypy

status:
	@${PYTHON} --version
	@${CARGO} --version

test: dev-install
	@${PYTEST} tests

repl:
	@$(PYTHON) -m asyncio
