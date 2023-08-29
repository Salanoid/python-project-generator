use anyhow::Result;

use crate::file_manager::create_file_with_content;

pub fn create_ci_testing_linux_only_file(
    project_slug: &str,
    source_dir: &str,
    min_python_version: &str,
    github_action_python_test_versions: &[String],
) -> Result<()> {
    let file_path = format!("{project_slug}/.github/workflows/testing.yml");
    let python_versions = github_action_python_test_versions
        .iter()
        .map(|x| format!(r#""{x}""#))
        .collect::<Vec<String>>()
        .join(", ");
    let content = format!(
        r#"name: Testing

on:
  push:
    branches:
    - main
  pull_request:
jobs:
  linting:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Set up Python
      uses: actions/setup-python@v4
      with:
        python-version: "{min_python_version}"
    - name: Get full Python version
      id: full-python-version
      run: echo version=$(python -c "import sys; print('-'.join(str(v) for v in sys.version_info))") >> $GITHUB_OUTPUT
    - name: Install Poetry
      run: |
        pip install pipx
        pipx install poetry
    - name: Configure poetry
      run: |
        poetry config virtualenvs.create true
        poetry config virtualenvs.in-project true
    - name: Cache poetry venv
      uses: actions/cache@v3
      id: poetry-cache
      with:
        path: .venv
        key: venv-${{{{ runner.os }}}}-${{{{ steps.full-python-version.outputs.version }}}}-${{{{ hashFiles('**/poetry.lock') }}}}
    - name: Ensure cache is healthy
      if: steps.poetry-cache.outputs.cache-hit == 'true'
      shell: bash
      run: timeout 10s poetry run pip --version || rm -rf .venv
    - name: Install Dependencies
      run: poetry install
    - name: Black check
      run: |
        poetry run black {source_dir} tests --check
    - name: Lint with ruff
      run: |
        poetry run ruff check .
    - name: mypy check
      run: |
        poetry run mypy .

  testing:
    strategy:
      fail-fast: false
      matrix:
        python-version: [{python_versions}]
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Set up Python {{{{ matrix.python-version }}}}
      uses: actions/setup-python@v4
      with:
        python-version: {{{{ matrix.python-version }}}}
    - name: Get full Python version
      id: full-python-version
      run: echo version=$(python -c "import sys; print('-'.join(str(v) for v in sys.version_info))") >> $GITHUB_OUTPUT
    - name: Install Poetry
      run: |
        pip install pipx
        pipx install poetry
    - name: Configure poetry
      run: |
        poetry config virtualenvs.create true
        poetry config virtualenvs.in-project true
    - name: Cache poetry venv
      uses: actions/cache@v3
      id: poetry-cache
      with:
        path: .venv
        key: venv-${{{{ runner.os }}}}-${{{{ steps.full-python-version.outputs.version }}}}-${{{{ hashFiles('**/poetry.lock') }}}}
    - name: Ensure cache is healthy
      if: steps.poetry-cache.outputs.cache-hit == 'true'
      shell: bash
      run: timeout 10s poetry run pip --version || rm -rf .venv
    - name: Install Dependencies
      run: poetry install
    - name: Test with pytest
      run: |
        poetry run pytest

"#
    );

    create_file_with_content(&file_path, &content)?;

    Ok(())
}

pub fn create_ci_testing_multi_os_file(
    project_slug: &str,
    source_dir: &str,
    min_python_version: &str,
    github_action_python_test_versions: &[String],
) -> Result<()> {
    let file_path = format!("{project_slug}/.github/workflows/testing.yml");
    let python_versions = github_action_python_test_versions
        .iter()
        .map(|x| format!(r#""{x}""#))
        .collect::<Vec<String>>()
        .join(", ");
    let content = format!(
        r#"name: Testing

on:
  push:
    branches:
    - main
  pull_request:
jobs:
  linting:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Set up Python
      uses: actions/setup-python@v4
      with:
        python-version: "{min_python_version}"
    - name: Get full Python version
      id: full-python-version
      run: echo version=$(python -c "import sys; print('-'.join(str(v) for v in sys.version_info))") >> $GITHUB_OUTPUT
    - name: Install Poetry
      run: |
        pip install pipx
        pipx install poetry
    - name: Configure poetry
      run: |
        poetry config virtualenvs.create true
        poetry config virtualenvs.in-project true
    - name: Cache poetry venv
      uses: actions/cache@v3
      id: poetry-cache
      with:
        path: .venv
        key: venv-${{{{ runner.os }}}}-${{{{ steps.full-python-version.outputs.version }}}}-${{{{ hashFiles('**/poetry.lock') }}}}
    - name: Ensure cache is healthy
      if: steps.poetry-cache.outputs.cache-hit == 'true'
      shell: bash
      run: timeout 10s poetry run pip --version || rm -rf .venv
    - name: Install Dependencies
      run: poetry install
    - name: Black check
      run: |
        poetry run black {source_dir} tests --check
    - name: Lint with ruff
      run: |
        poetry run ruff check .
    - name: mypy check
      run: |
        poetry run mypy .

  testing:
    strategy:
      fail-fast: false
      matrix:
        python-version: [{python_versions}]
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: {{{{matrix.os}}}}
    steps:
    - uses: actions/checkout@v3
    - name: Set up Python {{{{ matrix.python-version }}}}
      uses: actions/setup-python@v4
      with:
        python-version: {{{{ matrix.python-version }}}}
    - name: Get full Python version
      id: full-python-version
      run: echo version=$(python -c "import sys; print('-'.join(str(v) for v in sys.version_info))") >> $GITHUB_OUTPUT
    - name: Install Poetry
      run: |
        pip install pipx
        pipx install poetry
    - name: Configure poetry
      run: |
        poetry config virtualenvs.create true
        poetry config virtualenvs.in-project true
    - name: Cache poetry venv
      uses: actions/cache@v3
      id: poetry-cache
      with:
        path: .venv
        key: venv-${{{{ runner.os }}}}-${{{{ steps.full-python-version.outputs.version }}}}-${{{{ hashFiles('**/poetry.lock') }}}}
    - name: Ensure cache is healthy
      if: steps.poetry-cache.outputs.cache-hit == 'true'
      shell: bash
      run: timeout 10s poetry run pip --version || rm -rf .venv
    - name: Install Dependencies
      run: poetry install
    - name: Test with pytest
      run: |
        poetry run pytest

"#
    );

    create_file_with_content(&file_path, &content)?;

    Ok(())
}

pub fn create_dependabot_file(project_slug: &str) -> Result<()> {
    let file_path = format!("{project_slug}/.github/dependabot.yml");
    let content = r#"version: 2
updates:
  - package-ecosystem: "pip"
    directory: "/"
    schedule:
      interval: "daily"
    labels:
    - skip-changelog
    - dependencies
  - package-ecosystem: github-actions
    directory: '/'
    schedule:
      interval: daily
    labels:
    - skip-changelog
    - dependencies
"#;

    create_file_with_content(&file_path, content)?;

    Ok(())
}

pub fn create_pypi_publish_file(project_slug: &str) -> Result<()> {
    let file_path = format!("{project_slug}/.github/workflows/pypi_publish.yml");
    let content = r#"name: PyPi Publish
on:
  release:
    types:
    - published
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Set up Python
      uses: actions/setup-python@v4
      with:
        python-version: "{{ cookiecutter.python_version }}"
    - name: Install Poetry
      run: |
        pip install pipx
        pipx install poetry
    - name: Install Dependencies
      run: |
        poetry install
    - name: Add pypi token to Poetry
      run: |
        poetry config pypi-token.pypi {{ "${{ secrets.PYPI_API_KEY }}" }}
    - name: Publish package
      run: poetry publish --build
"#;

    create_file_with_content(&file_path, content)?;

    Ok(())
}

pub fn create_release_drafter_file(project_slug: &str) -> Result<()> {
    let template_file_path = format!("{project_slug}/.github/release_drafter_template.yml");
    let template_content = r#"name-template: 'v$RESOLVED_VERSION'
tag-template: 'v$RESOLVED_VERSION'
exclude-labels:
  - 'dependencies'
  - 'skip-changelog'
version-resolver:
  minor:
    labels:
      - 'breaking-change'
      - 'enhancement'
  default: patch
categories:
  - title: 'Features'
    labels:
      - 'enhancement'
  - title: 'Bug Fixes'
    labels:
      - 'bug'
  - title: '⚠ Breaking changes'
    label: 'breaking-change'
change-template: '- $TITLE @$AUTHOR (#$NUMBER)'
template: |
  ## Changes

  $CHANGES
"#;

    create_file_with_content(&template_file_path, template_content)?;

    let file_path = format!("{project_slug}/.github/workflows/release_drafter.yml");
    let content = r#"name: Release Drafter

on:
  push:
    branches:
      - main

jobs:
  update_release_draft:
    runs-on: ubuntu-latest
    steps:
      - uses: release-drafter/release-drafter@v5
        with:
          config-name: release_drafter_template.yml
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
"#;

    create_file_with_content(&file_path, content)?;

    Ok(())
}