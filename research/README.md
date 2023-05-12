# Research
This was done on 08/05/2023, around 7:39 PM. The battle starts from 7:40 PM and ended around 7:55 PM.

This is the data from the final replay, and we also have the tempArenaInfo.json and the python.log
```json
{"matchGroup": "pvp", "gameMode": 7, "clientVersionFromExe": "12,3,1,6965290", "scenarioUiCategoryId": 0, "eventType": "", "mapDisplayName": "01_solomon_islands", "mapId": 1, "clientVersionFromXml": "12,3,1,6965290", "weatherParams": {"0": ["PCOW003_Cloudy"], "1": ["PCOW005_Evening"], "2": ["PCOW005_Evening", "PCOW002_Storm"]}, "disabledShipClasses": [], "playersPerTeam": 12, "duration": 1200, "name": "12x12", "scenario": "Domination_3point", "playerID": 0, "vehicles": [{"shipId": 4290689008, "relation": 2, "id": 671300040, "name": "crazy_wanna"}, {"shipId": 4290689008, "relation": 1, "id": 671203957, "name": "Samuel_Lau_SL"}, {"shipId": 4186879312, "relation": 0, "id": 671163302, "name": "HenryQuan"}, {"shipId": 4186879440, "relation": 2, "id": 537194997, "name": "aaakkkiii"}, {"shipId": 4186847184, "relation": 1, "id": 268378400, "name": ":Tirpitz:"}, {"shipId": 4186879312, "relation": 1, "id": 268378401, "name": ":Radford:"}, {"shipId": 4186912208, "relation": 1, "id": 268378402, "name": ":Tributs:"}, {"shipId": 4082055120, "relation": 1, "id": 268378403, "name": ":Jellicoe:"}, {"shipId": 4186846416, "relation": 1, "id": 268378404, "name": ":Cunningham:"}, {"shipId": 4293867216, "relation": 1, "id": 268378405, "name": ":Spee:"}, {"shipId": 4279154384, "relation": 1, "id": 268378406, "name": ":Popov:"}, {"shipId": 4186879760, "relation": 1, "id": 268378407, "name": ":Beatty:"}, {"shipId": 4186846672, "relation": 1, "id": 268378408, "name": ":Zavoyko:"}, {"shipId": 4266538992, "relation": 2, "id": 268378409, "name": ":Fletcher:"}, {"shipId": 4186847056, "relation": 2, "id": 268378410, "name": ":Tegetthoff:"}, {"shipId": 4186879760, "relation": 2, "id": 268378411, "name": ":Souchon:"}, {"shipId": 4082054960, "relation": 2, "id": 268378412, "name": ":Buckmaster:"}, {"shipId": 4082054960, "relation": 2, "id": 268378413, "name": ":Warrender:"}, {"shipId": 4186846512, "relation": 2, "id": 268378414, "name": ":Goltz:"}, {"shipId": 4186912208, "relation": 2, "id": 268378415, "name": ":Spruance:"}, {"shipId": 4186879728, "relation": 2, "id": 268378416, "name": ":Schofield:"}, {"shipId": 4186879728, "relation": 2, "id": 268378417, "name": ":Pakenham:"}, {"shipId": 4269684432, "relation": 2, "id": 268378418, "name": ":Lee:"}, {"shipId": 4266538992, "relation": 1, "id": 268378399, "name": ":Ghormley:"}], "gameType": "RandomBattle", "dateTime": "08.05.2023 19:39:32", "mapName": "spaces/01_solomon_islands", "playerName": "HenryQuan", "scenarioConfigId": 14, "teamsCount": 2, "playerVehicle": "PVSC103-Vicente-Guerrero", "battleDuration": 1200, "mapBorder": null}
```
My ID is `671163302 (280123A6)` the this battle, ship ID is `575442 (08C7D2)`, death type is 18 (12). My ship's max health is `19600 (4C90)`, total damage is `87977 (0157A9)`, potential damage is `1758803 (1AD653)`. I was sunk by `575438 (08C7CE) 671300040 (280339C8)`

## Observations
- 575442 appeared 17993 times if I will guess this is bound to the frame rate
    - 17993 / 60 = 299, if we divide 20 on top of that, we get 14 which is close to the battle time
    - My monitor is 100 Hz so it is updating every 5 frames
- The offset between my team and enemy team is consistent which is 26 (1A)
    - Team Score 406 - 913
        - 0x55F9ED, we have 9601 which is my team score
        - 0x55FA07, we have 9103 which is enemy team score
        - 0x55F901 & 0x55FB85, my ship ID
        - 0x55F9AB & 0x55F972, ship who sunk me ID
        - 0x55F993, 12 which is death type
    - Team Score 453 - 872
        - 0x559D96, we have C501 which is my team score
        - 0x559DB0, we have 6803 which is enemy team score
- Damage received 479, 877
    - 0x50D6CC, 6D03 which is 877
    - 0x507881, DF01 which is 479
        - 479 can be dealt by me
        - 0x5078A2, my ship id
- My ship ID may be related to number of guns I have, the ship has 6 guns so the ID is from 575437 to 575442
    - However, just my guess

# Events
- 31 00 00 00 08 00 00 00, the beginning of all events
- Kill event 4B 00 00 00 0C 00 00 00
    - 4 bytes killer ID
    - 4 bytes killed ID
    - 4 bytes death type
- Chat event 80 00 00 00 XX 00 00 00
    - unknown 4 bytes
    - 1 byte chat type
        - 0B battle team
        - 0D battle common
    - battle_team / battle_common (this is just a string)
    - 1 byte message length
    - message
    - 1 byte null terminator
- Team score event 0E 00 00 00 22 00 00 00
    - unknown 7 bytes
    - 08 00 00 05 00 00 00 A4
    - 2 bytes (F6 90 [Our Team] or F6 D0 [Enemy Team])
    - 2 bytes score
- Damage event (this one can be very complicated)
    - 6B 00 00 00 21 00 00 00, the damage and receiver
        - 4 bytes, weapon ID?
        - 8 unknown bytes
        - 4 bytes, damage receiver ID
        - 4 bytes, damage amount
        - 13 unknown bytes
    - 20 00 00 00 2B 00 00 00, the damage dealer if ID is valid?
        - 4 unknown bytes
        - 4 bytes, damage dealer ID
        - 28 unknown bytes
        - This event has shorter length than expected
        - NOTE: 20 can also be position, since the enemy never moved
    - 0E 00 00 00 07 00 00 00, the damage dealer if ID is valid?
        - 4 unknown bytes
        - 4 bytes, damage dealer ID
