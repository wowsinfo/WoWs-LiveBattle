import asyncio
import websockets
import json

from player_info import PlayerInfo
from battle_info import BattleInfo


async def hello():
    players_dict = {}
    battle_info = {}

    async with websockets.connect("ws://127.0.0.1:8615") as websocket:
        await websocket.send("Hello world!")
        print("Connected to server!")
        while True:
            message = await websocket.recv()
            # decode from json
            message = json.loads(message)
            print("Received message: " + str(message))
            if "payload" in message:
                payload = message["payload"]
                if "OnArenaStateReceived" in payload:
                    # reset previous data
                    print(battle_info)
                    battle_info.clear()
                    players_dict.clear()
                    print("New battle started!")
                    print("Payload: " + str(payload))
                    players = payload["OnArenaStateReceived"]["players"]
                    print("Players: " + str(players))
                    for player in players:
                        player_info = PlayerInfo.from_dict(player)
                        # ship_id is the main one
                        key = player_info.ship_id
                        print("Added player: " + player_info.name)

                        info = BattleInfo()
                        info.max_health = player_info.max_health
                        battle_info[key] = info
                        players_dict[key] = player_info
                elif "DamageReceived" in payload:
                    damage = payload["DamageReceived"]
                    victim_id = damage["victim"]
                    aggressors = damage["aggressors"]
                    for aggressor in aggressors:
                        aggressor_id = aggressor["aggressor"]
                        damage_dealt = aggressor["damage"]
                        battle_info[aggressor_id].damage_dealt += damage_dealt
                        battle_info[victim_id].damage_received += damage_dealt
                        print(
                            "{} dealt {} damage to {}".format(
                                players_dict[aggressor_id].name,
                                damage_dealt,
                                players_dict[victim_id].name,
                            )
                        )
                elif "EntityProperty" in payload:
                    entity_property = payload["EntityProperty"]
                    entity_id = entity_property["entity_id"]
                    property = entity_property["property"]
                    value = entity_property["value"]
                    if property == "battleResult":
                        # get the winning teamid
                        winning_teamid = value["winnerTeamId"]
                        print("Winning teamid: " + str(winning_teamid))
                        for player in battle_info:
                            if players_dict[player].teamid == winning_teamid:
                                battle_info[player].win = True
                    if property == "health":
                        # update the health to a ship
                        battle_info[entity_id].health = value


if __name__ == "__main__":
    print("Starting client...")
    asyncio.run(hello())
