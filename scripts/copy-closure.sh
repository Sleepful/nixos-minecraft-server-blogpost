STORE_PATH=$(cat /store.path) \
  && nix-copy-closure --to --use-substitutes ${SERVER} \
  $(ls $STORE_PATH | xargs -i -n1 echo ${STORE_PATH}/{})
