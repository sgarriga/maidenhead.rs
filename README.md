# maidenhead.rs
Rust version of my maidenhead program that converts latitude and longitude to Maidenhead grid locators.
C version https://github.com/sgarriga/maidenhead

I adapted the C version from Ossi Väänänen's code https://ham.stackexchange.com/questions/221/how-can-one-convert-from-lat-long-to-grid-square to something more useful for me.
Test were cases lifted from https://gist.github.com/laemmy/maidenhead.py

To build the executable (in ./target/debug)
  - cargo build maidenhead

To test
  - ./grid_test.sh
