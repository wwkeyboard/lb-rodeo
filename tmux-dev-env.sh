#!/bin/sh

set +x
set +e

tmux new-session -s lb-dev -n editor -d
tmux send-keys -t lb-dev 'nvim .' C-m

