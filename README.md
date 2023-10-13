# WoWs LiveBattle
Read and parse live packets from World of Warships. This is based on [wows-replays](https://github.com/lkolbly/wows-replays) (Rust), [WoWS-ReplaysUnpack](https://github.com/Nodsoft/WoWS-ReplaysUnpack) (C#) and [replays_unpack](https://github.com/Monstrofil/replays_unpack/) (Python). I have also done my own research [here](https://github.com/wowsinfo/WoWs-LiveBattle/tree/master/archive) by analysing raw data. The [parser](https://github.com/wowsinfo/WoWs-LiveBattle/tree/master/parser) project is modified based on [wows-replays](https://github.com/lkolbly/wows-replays).

## Usage
You need to provide the latest `scripts` folder from the game. It can also be downloaded [here](https://github.com/wowsinfo/data/tags). Place the scripts under `version/{game version}/scripts` (like 12.9.0). This can be done automatically later when the Unpacker is ready.

Start the server and write your own client, see [clients](https://github.com/wowsinfo/WoWs-LiveBattle/tree/master/livebattle/client) for a simple Python demo. The server will send latest packets in JSON format to all connected clients. They will be sent in bundles to increase the speed. The client should be receive an array of packets. Simply, loop through all packets and do whatever you like.

## Notice
This project is still in its early development. All packets you can see are sent from the game server. That means everything you get is legit. You cannot get what you cannot see. With this project, you can know more about the current battle. This is similar to a MOD, but without the need of installing anything, and it works alongside the game.
