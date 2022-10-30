# tcpprepend
Simple TCP forwarder that prepends some fixed bytes to responses and ignores some fixed header on requests

Install it by building (`cargo install --path .`) or download pre-built executables from [Github Releases](https://github.com/vi/tcpprepend/releases/).

# Example

```
A$ echo -n 'ABC' | base64
A: QUJD

A$ echo -n 'XYZ' | base64
A: WFla

A$ tcpprepend 127.0.0.1:1234 QUJD 127.0.0.1:1235 WFla&
A: [1] 30101

B$ nc -lvp 1235
B: Listening on 0.0.0.0 1235

C$ nc 127.0.0.1 1234
A: Incoming connection from 127.0.0.1:40834
C> 12345
C> asdfg
C> 67ABC890
A:  found matching request bytes
A:   connected to upstream
A:   wrote prepender bytes
B: Connection received on localhost 44242
B: 890
A: XYZ
C> tttyyy
B: tttyyy
B> 555666
C: 555666
```

# Usage line

```
ARGS:
    <listen>

    <request_needle_base64>

    <connect>

    <response_prepend_base64>
 ```

    
