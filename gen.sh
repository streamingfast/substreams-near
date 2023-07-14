#!/bin/bash

NEAR_SPKG="https://github.com/streamingfast/firehose-near/releases/download/v1.1.0/substreams-near-v1.1.0.spkg"
substreams protogen "$NEAR_SPKG" --exclude-paths="sf/substreams/rpc,sf/substreams/v1,google/" --output-path="./core/src/pb"
