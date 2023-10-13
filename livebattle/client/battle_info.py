from dataclasses import dataclass

@dataclass
class BattleInfo:
    health: float = 0
    max_health: float = 0
    damage_received: float = 0
    damage_dealt: float = 0
    frags: int = 0
    win: bool = False
