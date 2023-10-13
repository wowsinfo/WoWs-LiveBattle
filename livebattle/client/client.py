import asyncio
import websockets

async def hello():
    async with websockets.connect("ws://127.0.0.1:8615") as websocket:
        await websocket.send("Hello world!")
        print("Connected to server!")
        while True:
            message = await websocket.recv()
            print(f"Received: {message}")

asyncio.run(hello())
