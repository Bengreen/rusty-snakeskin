


PYTHON:=venv/bin/python3
PIP:=venv/bin/pip
PYTEST:=venv/bin/pytest
CARGO:=cargo

status:
	@${PYTHON} --version
	@${CARGO} --version

clean:
	@rm -rf venv
	@rm -rf *.egg-info
	@rm -rf .pytest_cache
	find . -iname "*.pyc" -delete
	find . -iname "__pycache__" -delete
	${CARGO} clean

venv/bin/activate:
	# /opt/homebrew/opt/python@3.10/bin/python3.10 -m venv venv
	/opt/homebrew/bin/python3 -m venv venv
	${PIP} install wheel

venv/bin/activate.path: venv
	@echo source venv/bin/activate.path to set PYTHONPATH
	@${PYTHON} -c "import sys; print('export PYTHONPATH=', end=''); print(':'.join(sys.path[1:]))" > venv/bin/activate.path

venv: venv/bin/activate venv/bin/activate.path

list: venv
	${PIP} list


build:
	cargo build


install: venv mypy/setup.py sharedmodule/setup.py build
	${PIP} install ./mypy
	${PIP} install ./sharedmodule

dev-install: venv mypy/setup.py sharedmodule/setup.py build
	${PIP} install -e mypy[dev]
	${PIP} install -e sharedmodule[dev]


${PYTEST}: venv setup.py
	${PIP} install -e mypy[dev]


test: dev-install
	@${PYTEST} tests

repl:
	@$(PYTHON) -m asyncio
