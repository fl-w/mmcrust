#!/usr/bin/env sh

# for tst in tests/*; do
#   cargo -q run "--" < $tst 2>/dev/null
# done

function quit() {
  rm tests/.err 2>/dev/null
}

trap quit EXIT

watchexec -c "cat tests/main.cmm | cargo run -- --eval --tac || echo 😔 failed.." 2>tests/.err
