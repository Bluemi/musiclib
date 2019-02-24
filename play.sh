#!/bin/bash

audacious -H -q "test.mid" 2>/dev/null 1>/dev/null
rm core.*
