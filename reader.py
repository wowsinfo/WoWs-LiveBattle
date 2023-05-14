"""
Read temp.wowsreplay, parse it, and print the result while streaming.
"""

import sys
import os
import time
import struct

def parse_events(data: memoryview):
    """
    Events are in format like XX (type) 00 00 00 XX (size) 00 00 00 Data Bytes,
    Sometimes, the actual data size is smaller than the size in the header.
    """

    total_damage = 0
    while len(data) > 4:
        # combine 4 bytes into an integer
        event_type = struct.unpack("I", data[0:4])[0]
        if event_type == 0 or event_type > 255:
            data = data[1:]
            continue

        size = struct.unpack("I", data[4:8])[0]
        if size > 255 or size == 0:
            # some error here, skip this event
            data = data[1:]
            continue
        
        data = data[8:]
        payload = data[0:size]
        if event_type == 0x4B and size == 0x0C:
            # kill event
            killer_id = struct.unpack("I", payload[0:4])[0]
            victim_id = struct.unpack("I", payload[4:8])[0]
            death_type = struct.unpack("I", payload[8:12])[0]
            print('Killer: {}, Victim: {}, Death Type: {}'.format(killer_id, victim_id, death_type))
        elif event_type == 0x0E and size == 0x22:
            battle_logic_id = struct.unpack("I", payload[4:8])[0]
            # make sure this is 00 05 00 00 00 A4 F6
            if  payload[8:15] == b'\x00\x05\x00\x00\x00\xA4\xF6':
                # team byte
                team_type = payload[15]
                team = 'My Team' if team_type == 0x90 else 'Enemy Team'
                # team score 2 bytes
                team_score = struct.unpack("H", payload[16:18])[0]
                print('BattleLogic: {}\t{}:\t{}'.format(battle_logic_id, team, team_score))
        elif event_type == 0x80:
            # check if we have "battle" in the payload
            raw_chat = bytes(payload).decode('utf-8', errors='ignore')
            if "battle_" in raw_chat:
                is_private = "battle_team" in raw_chat
                offset = 5 + (11 if is_private else 13)
                message_length = struct.unpack("B", payload[offset:offset+1])[0]
                message = bytes(payload[offset+1:offset+1+message_length]).decode('utf-8', errors='ignore')
                if is_private:
                    print('Private: {}'.format(message))
                else:
                    print('Public: {}'.format(message))
        elif event_type == 0x6B:
            weapon_id = struct.unpack("I", payload[0:4])[0]
            receiver_id = struct.unpack("I", payload[12:16])[0]
            damage = struct.unpack("I", payload[16:20])[0]
            total_damage += damage
            if damage > 0:
                print('Receiver: {}, Damage: {} \t Weapon?: {}'.format(receiver_id, damage, weapon_id))

        # the actual data size can be smaller than the size in the header, let's seek only half of the size
        data = data[int(size/2):]
    print('=== End, total damage: {} ==='.format(total_damage))

def parse_header():
    pass

def stream_wowsreplay(path: str):
    """
    Read temp.wowsreplay in binary mode, parse it, and all parsable data.
    """
    with open(path, "rb") as replay:
        content = replay.read()
    content = memoryview(content)
    
    # TODO: for now let's skip until we find 31 00 00 00 08 00 00 00
    for i in range(len(content)):
        if content[i:i+8] == b'\x31\x00\x00\x00\x08\x00\x00\x00':
            content = content[i:]
            print('Found first event at offset {}'.format(i))
            break
    
    parse_events(content)
    

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: reader.py <path to temp.wowsreplay>')
        sys.exit(1)

    path = sys.argv[1]
    if not os.path.isfile(path):
        print('File not found: {}'.format(path))
        sys.exit(1)
    stream_wowsreplay(path)

    # stream
    # with open(path, 'rb') as f:
    #     # Get the initial file size and modification time
    #     stat_info = os.stat(path)
    #     last_size = stat_info.st_size
    #     last_modified = stat_info.st_mtime

    #     while True:
    #         # Wait for a short time
    #         time.sleep(0.1)

    #         # Get the current file size and modification time
    #         stat_info = os.stat(path)
    #         current_size = stat_info.st_size
    #         current_modified = stat_info.st_mtime

    #         # Check if the file has been modified
    #         if current_size != last_size or current_modified != last_modified:
    #             print('File has been modified!')
    #             # Do something with the file here
    #             f.seek(last_size)
    #             new_data = f.read(current_size - last_size)
    #             print(new_data)

    #         # Update the last size and modification time
    #         last_size = current_size
    #         last_modified = current_modified
