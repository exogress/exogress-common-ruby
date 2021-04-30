#!/bin/bash

cargo rustc --release -- -C link-args=-Wl,-undefined,dynamic_lookup
