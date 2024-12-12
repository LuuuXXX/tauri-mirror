#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

chmod +x ci/scripts/run-with-docker.sh
ci/scripts/run-with-docker.sh "aarch64-unknown-linux-gnu"