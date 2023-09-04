# hash finder

fast finder of numbers with a specified amount of trailing zeros in their SHA-256 hash

## usage

```
Usage: hash-finder-bin [OPTIONS] -N <TRAILING_ZEROS> -F <NUM_NUMBERS>

Options:
  -N <TRAILING_ZEROS>
  -F <NUM_NUMBERS>
      --num-threads <NUM_THREADS>
  -h, --help                       Print help
```

## example run

```sh
cargo run --release -- -N 6 -F 10
```

output:

```
25996400, "99ba2a2d268761b7ea51d9c70b258a989357297b0fc8b77581a80e0c97000000"
33304984, "2936799a7543f193ac7e8aa78bdb506a8b9f98c9830e8b6dd05e1a9b4e000000"
35577035, "d1be60d19b5566dc565513c5d94106c53e6d0a2eae2fb24d14c374ee7b000000"
45111918, "43c285daec3b3b4df6eb569fc5b259c6254fce98bcc7ab193fc0b01874000000"
68860700, "092f1b9e58d288cbfa901e41968689942c6697b34d8a26589186e2963c000000"
85189534, "45c0e27835b91ad5ccc8f6d66de89c986f83383c9028f41e711a6a944f000000"
84927136, "493ec207cbced93f0b36af6ceeedf300ca2a30d6e5bd9bfe3f6177b702000000"
92438489, "f2b95b8b4ee40f9e13d125af60d6b96da8c4b3899fea8600685980e1c9000000"
99614398, "e554d871774fe9f3b8136aaae7ad39f3316ff153c9b624ec93c71f9643000000"
121368255, "14bc90013c89c6a5a3f45fe243650393e4d936d5e854159771edae2ab7000000"
```

## testing

```sh
cargo nextest run --workspace
```
