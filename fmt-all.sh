#!/bin/sh

set -xe

for i in day-* commons; do (
	cd $i;
	cargo fmt
); done
