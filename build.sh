#!/bin/bash

cargo build --release

cp ./target/release/binaural ./binaural

chmod +x ./binaural
