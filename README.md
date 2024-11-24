# Genpasswd - Just Another CLI Password Generator
No dependencies, uses a [Mersenne Twister](https://en.wikipedia.org/wiki/Mersenne_Twister) implementation to generate pseudo-random numbers, seeded with time. Also ensures characters from each enabled character set is distributed evenly, by first randomly selecting a set, then selecting a character from the set, rather than enabling a set by appending its characters to an "enabled" string, and picking randomly from there; there is a lower chance of landing on a digit simply because there are fewer digits, one must first select the set, then the character.

Does what it says on the tin, nothing remarkable here. Mainly an excuse to implement a Mersenne Twister in Rust tbh.

```
Usage: passgen [charclasses (default adx) | length (default 32)]
Where charclasses can be any combination of:
    u = uppercase
    l = lowercase
    a = alphabetic (both l + u)
    n | d = numeric aka digits
    s | x = special aka extra

Example:
    genpasswd 10 ul         10 Upper and lowercase.
    genpasswd 10 ns         10 Numerical and special characters.

Order is irrelevant:
    genpasswd ld 10         10 Lowercase and digits.

Default (no args):
    genpasswd               32 Alphabetic numeric and special (default)
```
