Data is in 4kb chunk, let's see how this goes

## Investigation
- Erie is 439921, Dalao is 439917, death is 6
    - 0x17C637 to 0x17C643 is extremely suspicious
    - B6 06 00 4B 00 00 00 0C 00 00 00 means death
        - 0C can be the size of this block?
- Satsuma is 439920, my ID is 439919
    - 0x1CE6B5 and 0x1CE738
- 00 A4 F6 90, this one is our team score update
    - F6 90 xx xx this is our team score
    - F6 D0 xx xx this is enemy team score
- The full message is 43 6B B6 06 00 00 05 00 00 00 A4 xx xx xx xx
= 439915 is battle logic, 6B B6 06
- 2A 91 B1 43, fire damage? NO
