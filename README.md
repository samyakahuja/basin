# Basin - Base n encoding

> implements [rfc-4648](https://tools.ietf.org/html/rfc4648) for Base64

## Base64

- 64 characters => 6 bits represented per printable character.
- An extra 65the character (`=`) is used assigned special status.

```
+--first octet--+-second octet--+--third octet--+
|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
+-----------+---+-------+-------+---+-----------+
|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|
+--1.index--+--2.index--+--3.index--+--4.index--+
```

Since the input is an integral number of octets, 3 cases can arise:

- final quantum of input is an integral multiple of 24 bits; the final unit of
  encoded output will be integral multiple of 4 characters followed by 0 `=`
  padding characters.
- final quantum of input is exactly 8 bits; the final unit of encoded output
  will be 2 characters followed by 2 `=` padding characters.
- final quantum of input is exactly 16 bits; the final unit of encoded output
  will be 3 characters followed by 1 `=` padding character.
