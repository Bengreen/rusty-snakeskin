import asyncio
import time


running = True


def start():
    print("i am going to sleep")
    time.sleep(5)
    print("i got here")


async def as_start():
    print("i am going to async sleep")
    await asyncio.sleep(5)
    print("I got an async sleep done")


async def as_start_loop():
    print("i am going to start now")
    while(running):
        await asyncio.sleep(1)
        print(".. still here")
    print("Python has completed")

async def as_stop():
    global running
    print("Stopping loop")
    await asyncio.sleep(5)
    print("Actuating stop")
    running = False
