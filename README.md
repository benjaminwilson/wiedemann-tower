# Calculator and arithmetic library for the Wiedemann tower of fields

Simple calculator for the Wiedemann tower of "binary" fields (see W86):
+ elements are represented as bitstrings with length a power of two;
+ addition is bitwise XOR;
+ the multiplication algorithm (implemented here without Karatsuba) is the evident one;
+ the inversion algorithm comes from FP97.

Done as an exercise and not intended to be efficient.  Parser courtesy of ChatGPT.

## Example usage
```
$ ./wiedemann
Bitstrings represent elements of the Wiedemann tower and must be length a power of 2.
Examples:
T1: 00 = 0, 10 = 1, 01 = X0, 11 = 1 + X0
T2: 0000 = 0, 1000 = 1, .., 1010 = 1 + X1, .., 1001 = 1 + X0X1
Enter expressions using 0/1, '*', '/', '+', '()', and '_' for the previous result.
Type 'exit' or press Ctrl+D to quit.

0010 * 1001
=0101
1  / _
=1001
(_ + 1000) * 1010
=0110
```

## References
+ **W86** Wiedemann, "An Iterated Quadratic Extension of GF(2)" (1986).
+ **FP97** Fan & Paar, "On Efficient Inversion in Tower Fields of Characteristic Two" (1997).

