#!/bin/sh

set -xe

for i in day-*; do (
	cd $i;
	cargo run
); done
