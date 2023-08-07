piencode
========

**a toy project only** (I will probably never update this.)

What would a file be like if we encoded it as pairs of (offset, length) into 4 billion bits of pi?

In order to get about twice as much value out of our random digits, as well as searching through the
bits of pi, we also search through the inverted bits. (So if we have bits 010101 in pi, we could match
010101 normally or 101010 inverted.)

0. Get up to 2^32 bits of pi.

I downloaded 1 billion hex digits of pi from https://pi2e.ch/blog/2017/03/10/pi-digits-download/
and then converted it as follows:

```
# cut off '3.'
cut -c 3- pi_hex_1b.txt > pi_hex_1b_only_frac.txt
# convert hex digits to raw bytes
xxd -r -p  pi_hex_1b_only_frac.txt > pibin
```

The file name has to be `pibin` and it has to be in the cwd.

You can also use the 1m digits file; the program will run a lot faster but the average match length will
drop.

1. Run the program

```
# create a small file to encode because encoding is slooooooooow
zstd Cargo.toml
# encode it: (will automatically multithread to your # of cpus)
cargo run --release Cargo.toml.zst
...
pibits.len(): 4000000000
inbytes = 176, inbits = 1408
range (inverted? true) of len 31: 0..31 at 2726557585..2726557616 (a283f791..a283f7b0)
range (inverted? false) of len 31: 31..62 at 871202026..871202057 (33ed7cea..33ed7d09)
range (inverted? false) of len 33: 62..95 at 1696538197..1696538230 (651f1e55..651f1e76)
range (inverted? true) of len 41: 95..136 at 3695393556..3695393597 (dc433b14..dc433b3d)
range (inverted? true) of len 32: 136..168 at 432009475..432009507 (19bff103..19bff123)
range (inverted? true) of len 35: 168..203 at 3524292409..3524292444 (d2106f39..d2106f5c)
range (inverted? false) of len 34: 203..237 at 2459002365..2459002399 (929165fd..9291661f)
range (inverted? true) of len 35: 237..272 at 1381138025..1381138060 (52527e69..52527e8c)
range (inverted? true) of len 34: 272..306 at 710220944..710220978 (2a551c90..2a551cb2)
range (inverted? true) of len 32: 306..338 at 1055745162..1055745194 (3eed648a..3eed64aa)
range (inverted? false) of len 35: 338..373 at 3115031777..3115031812 (b9ab9ce1..b9ab9d04)
range (inverted? true) of len 32: 373..405 at 2524206292..2524206324 (967454d4..967454f4)
range (inverted? false) of len 33: 405..438 at 1596440225..1596440258 (5f27bea1..5f27bec2)
range (inverted? true) of len 32: 438..470 at 1282613713..1282613745 (4c7321d1..4c7321f1)
range (inverted? true) of len 33: 470..503 at 2939198712..2939198745 (af309cf8..af309d19)
range (inverted? true) of len 31: 503..534 at 183395308..183395339 (0aee63ec..0aee640b)
range (inverted? false) of len 37: 534..571 at 944037699..944037736 (3844df43..3844df68)
range (inverted? true) of len 31: 571..602 at 3105150369..3105150400 (b914d5a1..b914d5c0)
range (inverted? false) of len 37: 602..639 at 91467034..91467071 (0573ad1a..0573ad3f)
range (inverted? false) of len 33: 639..672 at 2394190852..2394190885 (8eb47404..8eb47425)
range (inverted? true) of len 35: 672..707 at 706555098..706555133 (2a1d2cda..2a1d2cfd)
range (inverted? false) of len 32: 707..739 at 481449955..481449987 (1cb257e3..1cb25803)
range (inverted? false) of len 34: 739..773 at 779985359..779985393 (2e7da1cf..2e7da1f1)
range (inverted? false) of len 33: 773..806 at 1542136206..1542136239 (5beb218e..5beb21af)
range (inverted? false) of len 32: 806..838 at 556740127..556740159 (212f2e1f..212f2e3f)
range (inverted? false) of len 36: 838..874 at 377746313..377746349 (1683f389..1683f3ad)
range (inverted? false) of len 33: 874..907 at 3203171904..3203171937 (beec8640..beec8661)
range (inverted? true) of len 33: 907..940 at 205948486..205948519 (0c468646..0c468667)
range (inverted? false) of len 32: 940..972 at 727465222..727465254 (2b5c3d06..2b5c3d26)
range (inverted? true) of len 33: 972..1005 at 2901849222..2901849255 (acf6b486..acf6b4a7)
range (inverted? true) of len 32: 1005..1037 at 1027572847..1027572879 (3d3f846f..3d3f848f)
range (inverted? true) of len 32: 1037..1069 at 1533149766..1533149798 (5b620246..5b620266)
range (inverted? true) of len 34: 1069..1103 at 942228464..942228498 (382943f0..38294412)
range (inverted? true) of len 31: 1103..1134 at 3824992007..3824992038 (e3fcbf07..e3fcbf26)
range (inverted? true) of len 35: 1134..1169 at 115505642..115505677 (06e279ea..06e27a0d)
range (inverted? true) of len 32: 1169..1201 at 2047406656..2047406688 (7a08f240..7a08f260)
range (inverted? false) of len 32: 1201..1233 at 653936285..653936317 (26fa469d..26fa46bd)
range (inverted? true) of len 32: 1233..1265 at 599606249..599606281 (23bd43e9..23bd4409)
range (inverted? true) of len 34: 1265..1299 at 2266173605..2266173639 (871310a5..871310c7)
range (inverted? true) of len 34: 1299..1333 at 1272272962..1272272996 (4bd55842..4bd55864)
range (inverted? false) of len 34: 1333..1367 at 511093759..511093793 (1e76abff..1e76ac21)
range (inverted? false) of len 33: 1367..1400 at 818139138..818139171 (30c3d002..30c3d023)
range (inverted? false) of len 8: 1400..1408 at 20..28 (00000014..0000001c)
```

## What/why

 - Encoding and compression is really interesting. 
 - I saw a post talking about encoding your data as an offset into pi, and I wondered if that could actually be made
    kinda-sorta practical.
 - I wanted some more rust practice.

## Other things
There's no decoder. (There's no real output format to decode!)

This is not pratical. It takes multiple minutes to encode 200ish bytes. It could be made faster
in many ways, but probably not to the point of practicality.

## Things that someone else could do
 - try to actually squeeze a compression algorithm out of this. I think you'd need to try ~every
   offset in the input file and try to string together the longest subsequences you could find.
 - write a decoder. It's kind of a cool exercise because there's a formula for hex digits of pi
   without needing to have all the previous digits, so decoding should actually be small and fast.
