#!/bin/sh

cargo build --release
diesel migration run --database-url=db.sqlite
mkdir -p package/
cp target/release/spoonfull package/spoonfull
cp db.sqlite package/
cp -r res/ package/res/
cp -r templates/ package/templates/
cp Rocket.toml package/

