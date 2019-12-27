#!/usr/bin/env python3

# This script could be used for actix-web multipart example test
# just start server and run client.py

import json
import asyncio
import aiohttp

async def req():
    resp = await aiohttp.ClientSession().request(
        "post", 'http://192.168.0.63:8088/',
        data=json.dumps({"log": "Test user"}),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


asyncio.get_event_loop().run_until_complete(req())
