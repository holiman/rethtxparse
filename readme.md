### Tx parser (`rethtxparse`)

This is a reth-based implementaton of [`txparse`](https://github.com/holiman/txparse).

A very simple utility, which reads line by line from standard input.

1. Read a line of input
2. If the line starts with `#`, it is ignored, and no output is emitted. Go to 1.
3. Remove any `non-alnum` characters from the input line (`[^0-9A-Za-z]`)
4. Try to interpret it as hexadecimal data
   - If error: print `err: <error>` and goto 1.
5. Parse a transaction (`Prague`-enabled, chainid `1`) from the data.
    - If error: print `err: <error>` and goto 1.
6. Recover the transaction `sender`.
    - If error: print `err: <error>` and goto 1.
    - Otherwise: print address of sender. 
7. Go to 1

If all goes well, it outputs a line containing the `address` of the sender.
Otherwise, it outputs `err: ` and a suitable error message.

Example:

```
$ cat sample.input | cargo r 
err: hex decoding error
err: hex decoding error
err: hex decoding error
err: unexpected tx type
err: input too short
err: overflow
0xd02d72e067e77158444ef2020ff2d325f929b363
0xd02d72e067e77158444ef2020ff2d325f929b363
0xd02d72e067e77158444ef2020ff2d325f929b363
err: hex decoding error
```
