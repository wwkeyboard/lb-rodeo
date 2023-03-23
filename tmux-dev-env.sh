#!/bin/sh

set +x
set +e

tmux has-session -t lb-dev
if [ $? != 0 ]
then
  tmux new-session -s lb-dev -n editor -d
  #tmux send-keys -t lb-dev 'nvim .' C-m

  tmux split-window -v -t lb-dev

  tmux split-window -h -p 66 -t lb-dev:0.1
  tmux split-window -h -p 50 -t lb-dev:0.2

fi
tmux attach -t lb-dev

