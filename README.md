# Basin - Base n encoding

> implements [rfc-4648](https://tools.ietf.org/html/rfc4648)

- base64 and base32 alphabets use padding but not base16
- implementations must reject(ignore) encoded data if it contains characters
  outside the base alphabet.
- the characters in the alphabet depend on the use-case.

## Base64

- 64 characters => 6 bits represented per printable character.
- An extra 65the character (`=`) is used assigned special status.

base64 alphabet

```
 0   25 26  51 52  61 62 63
 a .. z A .. Z 0 .. 9  +  /
```

24-bit groups are formed from 3 8-bit input groups which are then treated as 4
6-bit groups. If fewer than 24 bits are available at the end, then bits with
value zero are added on the right to form an integral number of 6-bit groups.

Since the input is an integral number of octets, 3 cases can arise:

- final quantum of input is an integral multiple of 24 bits; the final unit of
  encoded output will be integral multiple of 4 characters followed by 0 `=`
  padding characters.
- final quantum of input is exactly 8 bits; the final unit of encoded output
  will be 2 characters followed by 2 `=` padding characters.
- final quantum of input is exactly 16 bits; the final unit of encoded output
  will be 3 characters followed by 1 `=` padding character.

```
+--first octet--+-second octet--+--third octet--+
|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
+-----------+---+-------+-------+---+-----------+
|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|
+--1.index--+--2.index--+--3.index--+--4.index--+
```

