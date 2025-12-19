#!/bin/bash

cargo build --release
sudo cp target/release/nero /usr/bin
