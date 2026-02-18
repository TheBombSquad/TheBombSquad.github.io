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
to enter the date and time on every boot. Therefore, aftermaket RTC (Real-Time Clock) cards
became popular - just copy the included software to your disk, add a line to `AUTOEXEC.BAT`, and the date and
time would automatically get set on boot, with the date and time being saved to the card, preserved with a battery backup.

This became less of an issue following the release of the IBM AT in 1984, which actually featured an on-board RTC controller with a battery backup.

---

```ibm
Current date is Tue  1-01-1980
Enter new date (mm-dd-yy): 
Current time is  0:00:10.98
Enter new time: _
```

*A common sight for MS-DOS users without RTC cards.*

---

### **The Clock/Calendar card**
![Apparat](/images/apparat-card-front.jpg)
This Compaq Portable that I've been restoring contains an "Clock/Calendar" card from a company called Apparat,
a manufacturer of accessories for many home computers of the 1980s. Unfortunately, this particular RTC card has
frustratingly little information available about it online. The only information I could find was an old archived
magazine article (PC Mag 1982-09 page 9) indicating that it was for sale for $129. (~$428.00 in 2026).[^apparat-ad]

MS-DOS allows for a date to be set up to the year 2099, meaning it's already Y2K-compliant. 
What happens when we attempt to initialize our RTC card with a date in the current year, 2026?

```ibm
C:\>DATE
Current date is Mon  2-16-2026
Enter new date (mm-dd-yy):

C:\>SETCLOCK INITIAL

Clock/Calendar Routine V1.14
Copyright 1982 Apparat, Inc.
02/16/99 20:39:34


Is this correct? (Y/N) _
```

...no, that's not correct! What's going on here?

---

### **Debugging!**
Let's first figure out how our program is actually getting the date in the first place.
In MS-DOS, this is actually pretty easy! In assembly language, we can execute system functions,
conveniently provided to us by MS-DOS, just by loading a byte into the `AH` register and calling `int 21h`.
The byte specified in the `AH` register determines which function is called.
We're interested in instances of these calls where `AH` is set to either `2Ah` or `2Bh`.
These represent the functions for getting the system time, and setting the system time, respectively.


>**2Ah - Get date**
>
>Returns in registers:
> - AL - day of the week
> - CX - year, between 1980 and 2099
> - DH - month, between 1 and 12
> - DL - day of the month, between 1 and 31

>**2Bh - Set date**
>
>Parameters supplied to registers:
> - CX - year, between 1980 and 2099
> - DH - month, between 1 and 12
> - DL - day of the month, between 1 and 31

Just as a side note, the year here is actually a 16-bit value, stored in the `CX` register.
...does this mean that a year up to 65535 can be stored? Nope, if you attempt to supply
a date beyond 2099 (0x833 in hex), `int 21h` will fail and return `0xFF` in register `AL`.

We can use the MS-DOS program `DEBUG` to disassemble (in its words - *unassemble*) the machine language,
and we do indeed end up finding a single call to `int 21h` with parameter `2Ah` here:
```ibm
-u 02e9
101A:02E9 BF6E01        MOV     DI,016E
101A:02EC B42A          MOV     AH,2A
101A:02EE CD21          INT     21
101A:02F0 895503        MOV     [DI+03],DX
101A:02F3 81E96C07      SUB     CX,076C
101A:02F7 80F963        CMP     CL,63
101A:02FA 7202          JB      02FE
101A:02FC B163          MOV     CL,63
```

We can then execute the program and break right after the call to `int 21h`, and inspect the registers:
```ibm
-g 02f0
AX=2A01  BX=0000  CX=07EA  DX=0210
SP=FFFE  BP=0000  SI=0066  DI=016E
DS=101A  ES=101A  SS=101A  CS=101A  
IP=02F0   NV UP EI PL ZR NA PE NC
101A:02F0 895503        MOV     [DI+03],DX       DS:0171=4D44
```

Looking at our above reference on this particular `int 21h` call, we do indeed see that the year is stored in register `CX` as `07EAh` - 2026!

So, everything is working correctly on the MS-DOS side, but a closer look at the assembly code above shows
that we're subtracing a suspicious-looking constant from `CX` - `076C` - which happens to be 1900!

Yup, it's just converting the year to a value representing 'number of years from 1900', 
and the code we're looking at is also, from lines `02F7` to `02FC`, capping this number to 99.

In fact, a quick search for our constant `6c 07` using `DEBUG` shows another use:
```ibm
-s cs:0100 0300 6c 07
101A:02C4 
101A:02F5 
-u 02c3
101A:02C3 B96C07        MOV     CX,076C  
101A:02C6 024C05        ADD     CL,[SI+05]  
```
It's used in the portion of the code where the data is being read back from the card. 
So, we can just adjust this constant to `0x7D0` (2000), and all of our issues should be solved!
Now we're storing the year as an offset from the year 2000, rather than the year 1900.

```ibm
C:\>SETCLOCK INITIAL

Clock/Calendar Routine V1.14
Copyright 1982 Apparat, Inc.
02/16/26 21:10:15


Is this correct? (Y/N) _
```
Finally, it's correct... for the next 74 years, anyways.

---

### **Digging a little deeper**

![M583](/images/apparat-card-m5832.jpg)

Is this the best possible solution, or can we actually come up with a better solution
with the hardware we have?

Wait, what specific hardware do we have, anyways? 
If you take a look at the image of the Clock / Calendar card in the previous section,
you'll notice it almost entirely consists of off-the-shelf 7400-series components, with one stand-out - the OKI Semiconductors M5832. 

As per the data sheet[^m5832], this chip is an RTC IC with a 4-bit address and data bus. 
We would normally anticipate a total capacity of 256 bits, but the function table
in the data sheet indicates that the internal counters have some fairly strict data limitations.
Only 3 bits are used for the 10s place of the second counter, and only one bit is used for the 10s place of the month counter.

The x86 actually provides us with some instructions to easily communicate with the M5832, or any device expecting BCD.
We can use the `AAM` instruction to convert a byte in `AL` to two BCD-encoded bytes. 
We can use the `AAD` instruction to convert two bytes in `AX` to a single hexadecimal-encoded value in `AX`.

For example:
If we have `0x1A` (26) in `AL`, the `AAM` instruction will fill register `AX` with `0x0206`.
Running `AAD` on this will reverse the process, restoring the original value of `0x1A`.

Okay, maybe some more useful docs on how to write an improved
program.. perhaps... here's some info on the calls to cs:037C,
the subroutine that handles I/O with the OUT instruction.

The IO port used is 0x2a2.

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

[^apparat-ad]: Apparat, Inc. *PC Magazine*, vol. 1, no. 5, Sept. 1982, https://archive.org/details/PC-Mag-1982-09
[^m5832]: OKI Semiconductor, *MSM5832 MICROPROCESSOR REAL-TIME CLOCK/CALENDAR*, Dec. 1983, https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/OKI/Oki%20MM5832%20RTC.pdf