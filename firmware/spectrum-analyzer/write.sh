#!/usr/bin/env bash

avrdude -c diecimila -p t84 -U flash:w:$1:e -b 9600