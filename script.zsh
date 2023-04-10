function treecko {
  TREECKO_BIN="~/.cargo/bin/treecko"


  if [[ "$1" == "goto" ]]; then
    RES=$(eval "$TREECKO_BIN $@")
    echo "Changing directory to\"$RES\""

    cd $RES
  else
    eval "$TREECKO_BIN $@"
  fi
}
