#!/usr/bin/env bash

systemfd --no-pid -s http::3002 -- cargo watch -x run 