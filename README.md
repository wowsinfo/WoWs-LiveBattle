# WoWs LiveBattle
Read and parse live packets from World of Warships. This is based on [wows-replays](https://github.com/lkolbly/wows-replays) (Rust), [WoWS-ReplaysUnpack](https://github.com/Nodsoft/WoWS-ReplaysUnpack) (C#) and [replays_unpack](https://github.com/Monstrofil/replays_unpack/) (Python). I have also done my own research [here](https://github.com/wowsinfo/WoWs-LiveBattle/tree/master/archive) by analysing raw data. The [parser](https://github.com/wowsinfo/WoWs-LiveBattle/tree/master/parser) project is modified based on [wows-replays](https://github.com/lkolbly/wows-replays).

## Usage
You need to provide the latest `scripts` folder from the game. It can be downloaded from [here](https://github.com/wowsinfo/data/tags). Place the scripts like this `version/12.9.0/scripts` (12.9.0 is the game version), the version folder needs to be in the same folder as the executable. This can be done automatically when the Unpacker is ready.

### Writing a client
Currently, the websocket sends packets in a specific format. It always sends a JSON string which contains an array of objects. The length can change in the future, but I will ensure it will be always an array. Loop through the array and parse all packets. Please see [clients](https://github.com/wowsinfo/WoWs-LiveBattle/tree/master/livebattle/client) for a simple Python demo. There will be some delays, but it shouldn't be that far apart. However, it is also not instant.

The reason why packets are sending in a bundle was to improve the performance. While packets can be sent one by one, it introduces a lot of overheads. By sending multiple packets in a bundle, the delay is greatly reduced, and it doesn't use up lots of resources at the same time.

## Notice
This project is still in its early development. All packets you can see are sent from the game server. That means everything you get is legit. You cannot get what you cannot see. With this project, you can know more about the current battle. This is similar to a MOD, but without the need of installing anything, and it works alongside the game.
