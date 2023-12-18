#!/usr/bin/env bash
set -e 
git tag $1
git push origin $1