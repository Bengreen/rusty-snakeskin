import asyncio
import time

def start():
    print("i am going to sleep")
    time.sleep(5)
    print("i got here")


async def as_start():
    print("i am going to async sleep")
    await asyncio.sleep(5)
    print("I got an async sleep done")
