# Curves 

A toy elliptic curve implementation in Rust enough to experiment with key exchange

Idea from: https://curves.xargs.org/

## Run the tests

Please check the unit test named: `test_key_exchange` in [curve61.rs](src/curve61.rs):

```commandline
cargo test
```

## TODO

* FiniteField61: auto generate mul_inverse & square_root automatically
* Improve FiniteField trait
* Curve61 point add: handle point addition with INF
* Add a way to plot the Curve61 in real numbers & in FiniteField61
* Add Curve61 [Montgomery ladder](https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Montgomery_ladder)
  * + benchmark
* Improve EllipticCurve trait
  * add trait based implementation of eval_at / point_add / point_mul
    * add Curve23 and add test_key_exchange too
* Add [Curve25519 impl](https://x25519.xargs.org/)
  * test it against openssl?
* Add more doc & comments

