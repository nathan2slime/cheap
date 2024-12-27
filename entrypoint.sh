#!/bin/sh

set -e

./migrations
RUST_LOG=info ./invee
