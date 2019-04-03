#!/usr/bin/env bash

cd backend && systemfd --no-pid -s http::3000 -- cargo watch -x run