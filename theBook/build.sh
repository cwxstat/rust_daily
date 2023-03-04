#!/bin/bash
mdbook build
rm -rf ../docs
mv book ../docs
git add ../docs
git add ../theBook
