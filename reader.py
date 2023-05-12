"""
Read temp.wowsreplay, parse it, and print the result while streaming.
"""

import sys
import os
import struct

def parse_events(data):
    """
    Events are in format like XX (type) 00 00 00 XX (size) 00 00 00 Data Bytes,
    Sometimes, the actual data size is smaller than the size in the header.
    """
    data = memoryview(data)
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
            # team score, 2 bytes
            team_type = struct.unpack("H", payload[14:16])[0]
            team = 'My Team' if team_type == 0x90F6 else 'Enemy Team'
            team_score = struct.unpack("H", payload[16:18])[0]
            print('{}: {}'.format(team, team_score))
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

    print('=== End')

def parse_header():
    pass

def stream_wowsreplay(path: str):
    """
    Read temp.wowsreplay in binary mode, parse it, and all parsable data.
    """
    with open(path, "rb") as replay:
        content = replay.read()
    
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