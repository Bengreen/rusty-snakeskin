


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
	@rm -rf clibrary/*.so clibrary/libclibrary_test.so*
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
	@echo "export DYLD_LIBRARY_PATH=$(PWD)/clibrary" >> venv/bin/activate.path

venv: venv/bin/activate venv/bin/activate.path

list: venv
	${PIP} list


build: clibrary
	cargo build


install: venv mypy/setup.py sharedmodule/setup.py build
	${PIP} install ./mypy
	${PIP} install ./sharedmodule

dev-install: venv mypy/setup.py sharedmodule/setup.py build
	${PIP} install -e mypy[dev]
	${PIP} install -e sharedmodule[dev]

clibrary: clibrary/libclibrary_test.so

clibrary/libclibrary_test.so:
	(cd clibrary; gcc -g -shared clibrary_test.c -o libclibrary_test.so)


${PYTEST}: venv setup.py
	${PIP} install -e mypy[dev]


test: dev-install
	@${PYTEST} tests

repl:
	@$(PYTHON) -m asyncio
