#!/bin/bash

rsync -rCv --exclude target --exclude .git * prater@172.20.0.21:~/src/battlesnake