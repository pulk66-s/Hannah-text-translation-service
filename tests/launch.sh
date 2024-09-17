#!/bin/bash

function launch_functional() {
    cd tests/api/functionnal
    python3 main.py
    cd -
}

echo "Launching functional tests..."
launch_functional