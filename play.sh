#!/bin/bash

x=$(mktemp -d)
ffmpeg -i "test.mid" "${x}/test.wav" 1>/dev/null 2>/dev/null
paplay "${x}/test.wav"
rm -r "${x}"
