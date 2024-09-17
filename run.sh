#!/bin/bash

function launch_api() {
    RUST_MIN_STACK=104857600 cargo run
}

launch_api