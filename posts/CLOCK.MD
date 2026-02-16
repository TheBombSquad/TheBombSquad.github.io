---
title: Reverse-engineering an Apparat RTC card for Y2K compliance
description: A super secret template post.
date: 2026-02-12
tags:
header_image: /images/post.png
---

The Compaq Portable that I've restored comes with a couple of
after-market additions, one of which is an RTC card from Apparat - the rather boringly named "Clock Calendar Card".
Unfortunately, information about this card is quite difficult to come across 
online, and all I have to work with is the original software, sourced from the original image of the Compaq's hard drive.
Oh, and the software isn't Y2K-compliant! The horror! Well, I guess it's time to patch it before the next millennium arrives...

---

### **Some context**
First, some context - what even is an 'RTC card'? The original IBM PC did not come with any
solution for storing the date and time after shutting off, requiring users
to enter the date and time on every boot. Therefore, aftermaket calendar cards, with a battery backup,
became popular - just copy the included software to your disc and add a line to `AUTOEXEC.BAT`, and the date and
time would automatically get set each boot.

```ibm
Current date is Tue  1-01-1980
Enter new date (mm-dd-yy): 
Current time is  0:00:10.98
Enter new time: _
```

*A common sight for MS-DOS users without RTC cards.*


(TODO: when did this stop happening? add something about clones doing the same thing)

This Compaq Portable contains an Apparat Clock/Calendar card, one with
frustratingly little information available, besides an old archived
magazine article (PC Mag 1982-09 page 9) indicating that it was for sale for $129. (~$428.00 in 2026)
(TODO: behavior with non-Y2K compliant software here)

So, how do we fix this? Well, we're first going to need to find where
in the code the date gets set... thankfully, MS-DOS makes this pretty
easy for us, we just need to search through the code until we find the
x86 instruction `int 21h` - and look for instances in which register
AH has been loaded with value 0x2B. This is the 'SET DATE' function,
with the following parameters;

DH register - month, 1 through 12
DL register - day of month, 1 through 31
CX year - year
AL - day of the week, sunday through saturday (0-6)

We do end up finding a call to this at 0x2C9 in the code!
At 0x2C3 in the code, we set an immediate value of 0x76C (1900) -
bingo! And following this, we add the value of [si+05] - AKA the value
at ds:0173. This is where our value got read in from the clock card.

From this, we can see the format in which the year is stored on the
card - an offset from the year 1900.

TODO: Need to look at ds:0173 and ds:0156 and get specific behavior

However, one oddity here is that the *origin* of the date is *two
bytes* - we see 0204 instead of 24. What's going on here? In fact, all
of the elements of the date are being stored as multiple bytes, which
seems quite inefficient. This happens at cs:028C.

We call LODSW to load these two bytes, and then call `AAD`. Wait, what
instruction is that?

This is actually an instruction called 'ASCII ADjust AX Before
Division'. Basically, what this instruction does is takes a
BCD-encoded number in AX, and coverts it to a single byte in AL. So,
if we have something like 0x0402, we will get 0x002A in AX instead.
What's the purpose of this? Likely, to make the date card as cheap as
possible, BCD-compatible 7400 series logic was used.

TODO: How does storing data into the card work?

The usage of BCD in this case tells us a few things - namely that the
*easiest* solution to Y2K compliance is just changing that constant
`1900` to a constant `2000`, seeing as the card simply stores an
offset from the start of the millenium. This definitely passes the
buck onto some sucker in a hundred years, so it's not exactly the most
ideal.

But a solution that works for the next 75 years is a good start, so
let's try and achieve that! We just need to look for another `int 21h`
call, this time with argument 0x2Ah. This is where the program sets
the date. Before that, let's just fix up the first constant involved
when it comes to reading the date from the card, which we find at
cs:02C3. Now, setting the date isn't properly working, so let's track
own the other constant.

In regards to that `int 21h` call, we find it at cs:02EEE, which is
where the software reads in the value. It does actually return the
correct value, as DOS is Y2K-complaint - 0x7E8 (2024). However, we're
once again subtracting a constant 1900, at cs:02F3. We just fix that
up, and boom! It works.

Okay, maybe some more useful docs on how to write an improved
program.. perhaps... here's some info on the calls to cs:037C,
the subroutine that handles I/O with the OUT instruction.

First call: AL 10h, DX 02A2 (DX is always the same)
S1: AL 50h
S2: AL 51h
M1: AL 52h
M2: AL 53h
H1: AL 54h
H2: AL 55h
D1: AL 57h
D2: AL 58h
M1: AL 59h
M2: AL 5Ah
Y1: AL 5Bh
Y2: AL 5Ch
Last call: AL 00h

All of these calls to OUT are followed by 380 cycles of waiting.
We then do an 'IN' call w/ AL remaining the same to read the output.
