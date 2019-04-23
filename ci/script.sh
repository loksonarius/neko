# This script takes care of testing your crate

set -ex

main() {
  # There are no tests in the repo currently

  # cross build --target $TARGET
  # cross build --target $TARGET --release
  #
  # if [ ! -z $DISABLE_TESTS ]; then
  #     return
  # fi
  #
  # cross test --target $TARGET
  # cross test --target $TARGET --release
  #
  # cross run --target $TARGET
  # cross run --target $TARGET --release
  echo 'Skipping tests for now...'
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
