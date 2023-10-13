from dataclasses import dataclass
from typing import Any


def from_str(x: Any) -> str:
    assert isinstance(x, str)
    return x


def from_int(x: Any) -> int:
    assert isinstance(x, int) and not isinstance(x, bool)
    return x

@dataclass
class PlayerInfo:
    name: str
    clan_tag: str
    avatar_id: int
    ship_id: int
    ship_params_id: int
    teamid: int
    max_health: int

    @staticmethod
    def from_dict(obj: Any) -> 'PlayerInfo':
        assert isinstance(obj, dict)
        name = from_str(obj.get("name"))
        clan_tag = from_str(obj.get("clan_tag"))
        avatar_id = from_int(obj.get("avatar_id"))
        ship_id = from_int(obj.get("ship_id"))
        ship_params_id = from_int(obj.get("ship_params_id"))
        teamid = from_int(obj.get("teamid"))
        max_health = from_int(obj.get("max_health"))
        return PlayerInfo(name, clan_tag, avatar_id, ship_id, ship_params_id, teamid, max_health)
