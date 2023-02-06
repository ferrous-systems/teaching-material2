#!/bin/bash

parent_dir_name=$(basename "$(dirname "$(pwd)")")
cp slides.md "$parent_dir_name".md

