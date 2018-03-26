#!/bin/bash

cargo doc

if [ -d "./target/doc/qcgpu" ]; then
    # If cargo doc(generates ./target/doc directory) has been run
    cp ./doc/external_docs.css ./target/doc/

    rustdoc "./doc/getting_started.md" --markdown-css "../rustdoc.css" --markdown-css "../main.css" --markdown-css "../normalize.css" --markdown-css "../external_docs.css" -o "./target/doc/qcgpu/"
fi
