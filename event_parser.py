"""
Read temp.wowsreplay in binary mode, extract all data with the following format:
XX 00 00 00 XX 00 00 00, ignore when XX is 00
"""

import os
import sys
import time
import struct
import json
import zlib

def parse_events(content: memoryview, event_map: dict):
    """
    Parse events from the replay file.
    """
    while len(content) > 8:
        event_type = content[0]
        if content[1:4] != b'\x00\x00\x00':
            content = content[1:]
            continue
        
        event_size = content[4]
        if content[5:8] == b'\x00\x00\x00':
            content = content[1:]
            continue

        if event_type == event_size == 0:
            content = content[1:]
            continue

        # convert to XX 00 00 00 XX 00 00 00 format
        event_name = '{:02X} 00 00 00 {:02X} 00 00 00'.format(event_type, event_size)
        content = content[1:]
        if not event_name in event_map:
            event_map[event_name] = 1
        else:
            event_map[event_name] += 1

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: event_parser.py <path to temp.wowsreplay>')
        sys.exit(1)

    path = sys.argv[1]
    if not os.path.isfile(path):
        print('File not found: {}'.format(path))
        sys.exit(1)

    with open(path, "rb") as replay:
        content = replay.read()
    content = memoryview(content)
    event_map = {}
    parse_events(content, event_map)
    # filter out rare events
    event_map = {k: v for k, v in event_map.items() if v > 1}

    event_type_map = {}
    for k, v in event_map.items():
        event_type = k.split(' ')[0]
        if not event_type in event_type_map:
            event_type_map[event_type] = v
        else:
            event_type_map[event_type] += v

    # sort by value
    event_map = dict(sorted(event_map.items(), key=lambda x: x[1], reverse=True))
    event_type_map = dict(sorted(event_type_map.items(), key=lambda x: x[1], reverse=True))

    with open('event_type_map.json', 'w') as f:
        json.dump(event_type_map, f, indent=4)

    with open('event_map.json', 'w') as f:
        json.dump(event_map, f, indent=4)

