---
title: Reverse-engineering an Apparat RTC card for Y2K compliance
description: Reverse-engineering an Apparat RTC card for Y2K compliance
date: 2026-02-20
tags:
  - retrocomputing
  - reverse-engineering
header_image: images/chip.png
---

The Compaq Portable that I've restored comes with a couple of
after-market additions, one of which is an RTC card from Apparat - the rather boringly named "Clock/Calendar Card".
Unfortunately, information about this card is quite difficult to come across 
online, and all I have to work with is the original software, sourced from the original image of the Compaq's hard drive.
Oh, and the software isn't Y2K-compliant! The horror! Well, I guess it's time to patch it before the next millennium arrives...

---

### **Some context**
First, some context - what even is an 'RTC card'? The original IBM PC did not come with any
solution for storing the date and time after shutting off, requiring users
to enter the date and time on every boot. Therefore, aftermarket RTC (Real-Time Clock) cards
became popular - just copy the included software to your disk, add a line to `AUTOEXEC.BAT`, and the date and
time would automatically get set on boot, with the date and time being saved to the card, preserved with a battery backup.

This became less of an issue following the release of the IBM AT in 1984, which actually featured an on-board RTC controller with a battery backup.

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
This Compaq Portable that I've been restoring contains a "Clock/Calendar" card from a company called Apparat,
a manufacturer of accessories for many home computers of the 1980s. Unfortunately, there is frustratingly little
information available about this card online. The only information I could find was an old archived
magazine article (PC Mag 1982-09 page 9) indicating that it was for sale for $129 (~$428.00 in 2026).[^apparat-ad]

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
Let's first figure out how the original Apparat software is actually getting the date in the first place.
In MS-DOS, this is actually pretty easy! In assembly language, we can execute system functions,
conveniently provided to us by MS-DOS, just by loading a byte into the `AH` register and calling `int 21h`.
The byte specified in the `AH` register determines which function is called.
We're interested in instances of these calls where `AH` is set to either `0x2A` or `0x2B`.
These represent the functions for getting the system time and setting the system time, respectively.


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
and we do indeed end up finding a single call to `int 21h` with parameter `0x2A` here:
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

Looking at our above reference on this particular `int 21h` call, we do indeed see that the year is stored in register `CX` as `0x07EA` - 2026!

So, everything is working correctly on the MS-DOS side, but a closer look at the assembly code above shows
that we're subtracting a suspicious-looking constant from `CX` - `0x076C` - which happens to be 1900!

Yup, it's just converting the year to a value representing 'number of years from 1900', 
and the code we're looking at is also, from lines `0x02F7` to `0x02FC`, capping this number to 99.

In fact, a quick search for our constant `6c 07` (bytes swapped around due to endianness) using `DEBUG` shows another use:
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


Is this correct? (Y/N) Y

C:\>DATE
Current date is Mon  2-16-2026
Enter new date (mm-dd-yy):
```
Finally, it's correct... for the next 74 years, anyways.

---

### **Digging a little deeper**

![M583](/images/apparat-card-m5832.jpg)

So, we swapped out a constant and patched the software for the next three-quarters of a century.
Is this the best possible solution, or can we actually come up with a better solution
with the hardware we have?

Wait, what specific hardware do we have, anyways? 
If you take a look at the image of the Clock/Calendar card in the previous section,
you'll notice it almost entirely consists of off-the-shelf 7400-series components, with one exception - the OKI Semiconductors M5832. 

As per its data sheet[^m5832], this chip is an RTC IC with a 4-bit address and a 4-bit data bus. 
With this combination, we would normally anticipate a total capacity of 256 bits, but the function table
provided in the data sheet indicates that the internal counters have some fairly strict data limitations.
For example, only three bits are used for the 10s place of the second counter, and only one bit is used for the 10s place of the month counter.

So, unfortunately, the card itself is only capable of keeping track of a period of 100 years.  

This data sheet also presents us with another question - we can only communicate with this card using 4 bits at a time, so how exactly does the x86 processor interact with this card? 

Well, after tracing through the original Apparat software some more, we find that the software uses the `IN` and `OUT` instructions to communicate with the card on IO port `0x2A2`.
The software also specifically uses a routine at `CS:037C` to add a delay between calls to `OUT`, by using the `LOOP` instruction.
The specific number of times the `LOOP` instruction is called is 550 times, which resolves to an approximate delay of 580 microseconds on a 4.77Mhz-clocked 8088.[^x86]

Also, any read or write access to the card also seems to stop the clock on the card, 
so in order to restore operation, the byte `0x00` needs to be sent.

I've created a table of operations based on my observations, with M5832 internal counter names taken from the data sheet:

| **Counter** | **Read** | **Write** |
|-------------|----------|-----------|
| S1          | `0x50`   | `0x10`    |
| S10         | `0x51`   | `0x11`    |
| MI1         | `0x52`   | `0x12`    |
| MI10        | `0x53`   | `0x13`    |
| H1          | `0x54`   | `0x14`    |
| H10         | `0x55`   | `0x15`    |
| W           | `0x56`   | `0x16`    |
| D1          | `0x57`   | `0x17`    |
| D10         | `0x58`   | `0x18`    |
| MO1         | `0x59`   | `0x19`    |
| MO10        | `0x5A`   | `0x1A`    |
| Y1          | `0x5B`   | `0x1B`    |
| Y10         | `0x5C`   | `0x1C`    |
| End         | `0x00`   | `0x00`    |

So, we need to split our date or time values into both their 10's place and their 1's place digits,
as the method of communication with the M5832 uses 4-bit BCD encoding.

This sounds like it would take a handful of operations to carry out, but we actually have an easy way to do this on the x86!
The x86 provides us with some instructions to easily communicate with the M5832, or any device expecting BCD encoding.
We can use the `AAM` instruction to convert a byte in `AL` to two BCD-encoded bytes,
and we can use the `AAD` instruction to convert two bytes in `AX` to a single hexadecimal-encoded value in `AX`.

As an example for how this works, we can consider the number of years since 2000 - 26, or `0x1A` in hex.

Loading `0x1A` into our `AX` register and then running the `AAM` instruction fills `AX` with the value `0x0206`.

Then, we just need to send `0x1B` to the M5832, followed by the value in `AL`, and then `0x1C` for the Y10 counter, and value in `AH`. Finally, sending `0x00` will have set the year properly.

This works in reverse too! If we want to read the year back, we just read the value at `0x5B` and `0x5C`.
However, since this is a 4-bit value, the top 4 bits of the value are junk and need to be masked out by calling `AND` with a value of `0xF`.
Once this is done, moving both of the values back to the `AX` register, and then running the `AAD` instruction will restore our value of `0x1C`.

---

## **Final thoughts**

There's actually an oddity in the above table that you might have noticed - counter 'W' - for day of the week!

This seemingly throws a wrench into everything, since the pattern determining what the days of the week are each month actually does not repeat every century, it repeats every 400 years!

Therefore, the 16th of February in 2026 is a Monday, and the 16th of February in 1926 is a Tuesday.

The day of the week is going to be inaccurate! Everything's ruined now, isn't it?

Actually, no, everything is fine - the original Apparat software completely ignores this counter and never assigns any value to it.
It also doesn't need to read from the value either, because MS-DOS automatically assigns the date of the week for you - 
if you'll notice above in this article, the `int 21h` call for setting the date does not take a day of the week as a parameter.

What this *does* mean is that we can actually use this day-of-the-week counter as a century counter! We have about three bits of precision to work with,
so if we just write our own clock-setting software that takes advantage of this, we can solve this issue for the next 800 years!

...actually, the FAT filesystem will need to be patched first, since the way *that* stores years only leaves us covered until 2107.

I'll leave that as an exercise for the reader!

---

## **The software**

I've uploaded the original `SETCLOCK.COM` and my patched `SETCLK21.COM` variant to Archive.org, for anyone who happens to have the same RTC card!
Find it here: https://archive.org/details/APPARAT-SETCLOCK

Thanks for reading!

---

[^apparat-ad]: Apparat, Inc. *PC Magazine*, vol. 1, no. 5, Sept. 1982, https://archive.org/details/PC-Mag-1982-09
[^m5832]: OKI Semiconductor, *MSM5832 MICROPROCESSOR REAL-TIME CLOCK/CALENDAR*, Dec. 1983, https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/OKI/Oki%20MM5832%20RTC.pdf
[^x86]: Intel Corporation, *iAPX 86/88, 186/188 User's Manual, Programmer's Reference*, May. 1983, https://bitsavers.trailing-edge.com/components/intel/80186/210911-001_iAPX86_88_186_188_Programmers_Reference_1983.pdf