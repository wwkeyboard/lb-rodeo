#!/bin/sh

set +x
set +e

tmux has-session -t lb-dev
if [ $? != 0 ]
then
  tmux new-session -s lb-dev -n editor -d
  tmux send-keys -t lb-dev 'nvim .' C-m

  tmux split-window -v -t lb-dev
  tmux select-layout -t lb-dev main-horizontal

  tmux split-window -h -t lb-dev:1.2
fi
tmux attach -t lb-dev

