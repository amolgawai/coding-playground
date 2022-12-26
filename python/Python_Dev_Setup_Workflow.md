Python Development Tools and Workflow
===============

Introduction
------------
This is my development setup for python. The intended use is for generic Python development and more specifically AI-ML development

Environment
-----------
OS - MacOS (Catalina - 10.15.3 )
Editor - Emacs with [elpy](https://github.com/jorgenschaefer/elpy)
Shell - [zsh](https://www.zsh.org/)

Concept
-------
  * **DO NOT** use the system python - The system python tends to be old as of this writing
  * **DO NOT** manage python with homebrew
  * Use a python manager - to separate cutting edge vs proven versions
  * Use virtual environments - to separate the package needs for different kind of applications
  * Use a python dependency and packaging manager - to manage installation, maintainance and reproduciability of the app eco
  * Use a code formatter - [black](https://github.com/psf/black) is the choice for now

Workflow and tools
------------------
### Install Python manager (pyenv)
[pyenv](https://github.com/pyenv/pyenv)

``` shell
$ brew install pyenv
```
Add `pyenv init `to your shell to enable shims and autocompletion. Please make sure eval "$(pyenv init -)" is placed toward the end of the shell configuration file since it manipulates PATH during the initialization.

``` shell
$ echo -e 'if command -v pyenv 1>/dev/null 2>&1; then\n  eval "$(pyenv init -)"\nfi' >> ~/.zshrc
```
Restart the shell and pyenv is ready.

Basic commands to manage python versions with pyenv

``` shell
# list available python versions
$ pyenv install --list

# install python 3.8.2
$ pyenv install 3.8.2

# list installed python versions, * means the global default
$ pyenv versions
* system
3.8.1

# set the global default to 3.8.2
$ pyenv global 3.8.1
```

### Install virtual environment manager for pyenv
[pyenv-virtualenv](https://github.com/pyenv/pyenv-virtualenv)

``` shell
$ brew install pyenv-virtualenv

# add initialize script in your bash configuration file
$ echo ‘eval “$(pyenv virtualenv-init -)”’ >> ~/.zshrc
```
Restart the shell so that virtual environments can be created

``` shell
# Create a virtualenv for a project
$ pyenv virtualenv 3.8.2 myVirtEnv

# To set and automatically switch to a virtual environment for a project
$ cd path-to-your-direcotry
$ pyenv local myVirtEnv
```

### Install alternate Python and virtual environment manager (conda - miniforge)
The pyenv started to have problems with building python on macOS  and hence this is an alternate to pyenv and pyenv-virtualenv.

[Miniforge](https://github.com/conda-forge/miniforge/#download) is an effort to provide Miniconda-like installers, with the added feature that [conda-forge](https://conda-forge.org/#about) is the default channel. Unlike Miniconda, these support ARMv8 64-bit (formally known as `aarch64`)

Download and run the installer shell script for appropriate platform.
To install with homebrew,

``` shell
brew install miniforge
```

Use conda to create environments based on specific python version and use package manager (poetry as described below) to install/maintain packages.

``` shell
conda create --name py35 python=3.5
# or with an environment file
conda env create -f environment.yaml

```

Minimal environment.yaml file with specific python version (to be committed at the root of project)

``` yaml
name: envname

channels:
  - default
  - conda-forge

dependencies:
  - python=3.8
```

### Global CLI tools using pipx
[pipx](https://pipxproject.github.io/pipx/)
pipx allows to install and run python applications in isolated environment. This is quite useful for italling development tools that are common across poject and do not depend on the virtual environment of the project

Install and configure pipx, install apps and tools
```shell
$ brew install pipx
$ pipx ensurepath

# install cookiecutter
$ pipx install cookiecutter # now cookiecutter is available everywhere

# install with extras
$ pipx install 'coverage[toml]'

# inject apps that have dependencies
$ pipx inject coverage coverage-badge[toml] --include-apps
```

### Install dependency and packaging manager
[poetry](https://python-poetry.org/docs/)

``` shell
# either through pipx
$ pipx install poetry

# or using install script
$ curl -sSL https://install.python-poetry.org | python3 -

# For tab completion in your shell, see the documentation
poetry help completions

# Configure poetry to create virtual environments inside the project's root directory
poetry config virtualenvs.in-project true
```

### Install packages for an environment
With the above setup, now we can install packages for a particular project with poetry

``` shell
# cd to the project directory, this will activate the virtual environment
$ cd path-to-your-direcotry

$ poetry add pandas numpy scipy tensorflow=2.1.0rc2 tensorflow-text matplotlib scikit-learn jupyter ipykernel

# Specify some dev libraries
$ poetry add --dev black flake8 pylint python-language-server importmagic epc ipython pytest
```

### Setup project
#### Manual
Start with initializing git repo and poetry.
Create following folder structure or use the Sample Project git repo as a template ([Sample Project](https://github.com/pypa/sampleproject))

```
project-repo/
├── LICENSE
├── pyproject.toml
├── README.md
├── docs/
├── src/
│   └── project_package/
│       ├── __init__.py
│       └── example.py
└── tests/
```

#### Using [hypermodern python cookiecutter](https://github.com/cjolowicz/cookiecutter-hypermodern-python)
`$ cookiecutter gh:cjolowicz/cookiecutter-hypermodern-python --checkout=2022.6.3`

### Dev Toolchain
  *  [psf/black: The uncompromising Python code formatter](https://github.com/psf/black)
  * [Pylint - code analysis for Python | www.pylint.org](https://www.pylint.org/)
  * [palantir/python-language-server: An implementation of the Language Server Protocol for Python](https://github.com/palantir/python-language-server)
  * [pytest - testing framework](https://docs.pytest.org/en/latest/contents.html)
  * [pycoverage - python code coverage](https://github.com/nedbat/coveragepy)
  * ~~[flake8 - style guide enforcement](https://flake8.pycqa.org/en/latest/)~~
  * [flake9 - modern flake8](https://gitlab.com/retnikt/flake9)
  * [ipython](https://ipython.org)
  * [radon - code metrics](https://github.com/rubik/radon)
  * [pre-commit](https://pre-commit.com "git pre-commit hooks") - Not python specific but ca be used to manage multi-language commit hooks
  * [mypy - static typing checker](https://github.com/python/mypy)
  * [cookiecutter - cli tool to create project from templates](https://github.com/cookiecutter/cookiecutter)
  * [hypermodern python cookiecutter](https://github.com/cjolowicz/cookiecutter-hypermodern-python)
  * settings using pyproject.toml -> [information about pyproject.toml](https://snarky.ca/what-the-heck-is-pyproject-toml/)
  * [tox - (test) automation CI](https://tox.readthedocs.io/en/latest/)
  * [nox - possible successor of tox](https://nox.thea.codes/en/stable/)
  * [invoke - python task execution (replaces make?)](http://docs.pyinvoke.org/en/stable/getting-started.html)
  * [Hypothesis - library to write better unit tests](https://hypothesis.readthedocs.io/en/latest/index.html)
  * [faker - generate fake data (for testing)](https://github.com/joke2k/faker)
  * [Mimesis - Fake Data Generator (for testing)](https://github.com/lk-geimfari/mimesis)

#### pytest plugins
  * [pytest-pythonpath -> add paths of modules while invoking pytest](https://github.com/bigsassy/pytest-pythonpath)
  * [pytest-sugar - change the default look of pytest output](https://github.com/Frozenball/pytest-sugar)
  * [pytest-cov -> add test coverage to pytest](https://github.com/pytest-dev/pytest-cov)
  * [pytest-testmon -> run only affected tests](https://github.com/tarpas/pytest-testmon/)
  * [pytest-watch -> automatically rerun tests](https://github.com/joeyespo/pytest-watch)
  * [pytest-picked -> run tests for modified code only](https://github.com/anapaulagomes/pytest-picked)
  * [pytest-benchmark -> pytest fixture for benchmarking code](https://github.com/ionelmc/pytest-benchmark)
  * [pytest-profiling -> profile code while running tests](https://github.com/man-group/pytest-plugins/tree/master/pytest-profiling)
  * [pytest-instafail -> show failed tests immediately](https://github.com/pytest-dev/pytest-instafail)
  * [pytest-tldr -> Short output of pytest run](https://github.com/freakboy3742/pytest-tldr)
  * [pytest-xdist -> run tests in parallel](https://github.com/pytest-dev/pytest-xdist)
  * [pytest-mock -> thin wrapper arounf unittest.mock](https://github.com/pytest-dev/pytest-mock)
  * [pytest-randomly -> randomly run tests to avoid test interdependency](https://github.com/pytest-dev/pytest-randomly)
  * [pytest-clarity -> more understandable test output](https://github.com/darrenburns/pytest-clarity)
  * [pytest-bdd ->  enables BDD by implementing a subset of the Gherkin language](https://github.com/pytest-dev/pytest-bdd)

### flake8 plugins
  * [awesome flake8 plugins](https://github.com/DmytroLitvinov/awesome-flake8-extensions)
  * [flake8-bugbear](https://github.com/PyCQA/flake8-bugbear)
  * [flake8-pytest-style](https://github.com/m-burst/flake8-pytest-style)
  * [flake8-docstrings](https://gitlab.com/pycqa/flake8-docstrings)
  * [flake8-rst-docstrings](https://github.com/peterjc/flake8-rst-docstrings)

### Emacs workflow
Open a python project in emacs with projectile
Activate virtual environment `pyvenv-workon`
Enjoy the development

### pyenv commands
[Command reference](https://github.com/yyuu/pyenv/blob/master/COMMANDS.md)

#### pyenv install


`pyenv install -l` List available python versions:


`pyenv install 3.5.1` Install Python 3.5.1

`pyenv rehash`

#### pyenv versions

`pyenv versions` List installed versions:

#### pyenv local

`pyenv local 2.7.6` Sets a local application-specific Python version:

`pyenv local --unset` Unset the local version:

#### pyenv-virtualenv

`pyenv virtualenvs` List existing virtualenvs

##### Create virtualenv

`pyenv virtualenv venv35` From current version with name "venv35":


`pyenv virtualenv 2.7.10 venv27` From version 2.7.10 with name "venv27":

##### Activate/deactivate

`pyenv activate <name>`
`pyenv deactivate`

##### Delete existing virtualenv

`pyenv uninstall venv27`

### poetry commands

`poetry new [package-name]` Start a new Python Project.

`poetry init` Create a _pyproject.toml_ file interactively.

`poetry install` Install the packages inside the _pyproject.toml_ file.

`poetry install --without test,docs` Install the packages from current project without documentation and tests

`poetry add [package-name]` Add a package to a Virtual Environment.

`poetry add -D [package-name]` Add a dev package to a Virtual Environment.

`poetry remove [package-name]` Remove a package from a Virtual Environment.

`poetry remove -D [package-name]` Remove a dev package from a Virtual Environment.

`poetry update` Get the latest versions of the dependencies

`poetry shell` Spawns a shell within the virtual environment.

`poetry build` builds the source and wheels archives.

`poetry publish` Publish the package to Pypi.

`poetry publish --build` Build and publish a package.

`poetry self:update` Update poetry to the latest stable version.

### pipx installations
Recommended pipx installations

```shell
$ pipx install black
$ pipx install radon
$ pipx install bandit
$ pipx install unimport
$ pipx install vulture
$ pipx install code2flow
$ pipx install cookiecutter
$ pipx install flake9
$ pipx install tox
$ pipx install nox
$ pipx install invoke
$ pipx inject flake9 flake8-bugbear flake8-bandit flake8-docstrings flake8-pytest-style flake8-rst-docstrings flake8-bandit
```

References
----------
1. [Setting Up Python: pyenv, pyenv-virtualenv, poetry](https://duncanleung.com/set-up-python-pyenv-virtualenv-poetry/)
2. [Get started with pyenv & poetry. Saviours in the python chaos!](https://blog.jayway.com/2019/12/28/pyenv-poetry-saviours-in-the-python-chaos/)
3. [Hypermodern Python](https://cjolowicz.github.io/posts/hypermodern-python-01-setup/)
