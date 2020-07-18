#!/usr/bin/env bash

export PROJDIR=${PROJDIR:-$(git rev-parse --show-toplevel)}
export IN_NIX_SHELL=${IN_NIX_SHELL:-nopes}

export CARGO_HOME="${PROJDIR}/.cargo";
export PATH=${CARGO_HOME}/bin:${PROJDIR}/venv/bin:$PATH

nn() {
    echo "in_nix:" "${IN_NIX_SHELL}"
}

setup() {
    pushd2 /

    test -f venv/bin/python || python -m venv venv
    test -f venv/bin/pre-commit || pip install -r requirements.txt

    test -f .git/hooks/pre-commit || pre-commit install
    mkdir -p .cargo;

    popd2
}

pushd2() {
    PUSHED=$(pwd)
    cd "${PROJDIR}""$1" >> /dev/null || return
}

popd2() {
    cd "${PUSHED:-$PROJDIR}" >> /dev/null || return
    unset PUSHED
}

o() {
    cd "${PROJDIR}" || return
}




alias open=/usr/bin/open
alias pbcopy=/usr/bin/pbcopy
alias gst="git status"
alias gd="git diff"
alias gp="git push"
alias cc="cargo fmt && cargo check"
# alias fix_cargo="find . | grep Cargo.toml | grep -v .cargo | grep -v target-nix | grep -v venv | xargs -n 1 cargo tomlfmt --path"
setup
