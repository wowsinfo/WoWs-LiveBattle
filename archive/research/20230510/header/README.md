# Header Analysis
- 0x0 - 0xB unknown bytes
- 0xC is version length
- 0X10 - 0x17 is the version
- 0X31 - 0x36 is unknown
- 0x45 - 0x47 is the magic number
    - This number + 1 is the player's ship ID
    - This is very strange, need further investigation
Starting from 0x13B0H