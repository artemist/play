#!/usr/bin/env nix-shell
#!nix-shell -i python3 -p python3
import json
import zlib
import sys
import pathlib
import time
import urllib.request
import shutil

# PNG, APNG, Lottie, GIF
STICKER_EXTENSIONS = ["", "png", "png", "json", "gif"]


def download_file(url, path):
    if path.exists():
        print("Already downloaded, skipping...")
        return
    out_file = open(path, "wb")
    req = urllib.request.urlopen(
        urllib.request.Request(url, headers={"User-Agent": "curl/8.6.0"})
    )
    shutil.copyfileobj(req, out_file)
    time.sleep(0.1)


def get_data(filename):
    bindata = open(filename, "rb").read()
    decompress = zlib.decompressobj()
    jsondata = decompress.decompress(bindata)
    while len(jsondata) > 0:
        try:
            obj = json.loads(jsondata)
            if obj.get("t", "") == "READY":
                return obj
        except json.JSONDecodeError as jde:
            obj = json.loads(jsondata[0 : jde.pos])
            if obj.get("t", "") == "READY":
                return obj
            jsondata = jsondata[jde.pos :]


def main(*args):
    data = get_data(args[1])

    base_dir = pathlib.Path("guilds")

    for guild in data["d"]["guilds"]:
        name = guild["properties"]["name"]
        print(f"Processing guild '{name}'")

        guild_dir = base_dir / name.replace("/", " ")
        sticker_dir = guild_dir / "stickers"
        sticker_dir.mkdir(parents=True, exist_ok=True)
        emoji_dir = guild_dir / "emoji"
        emoji_dir.mkdir(exist_ok=True)

        for sticker in guild["stickers"]:
            break
            print(f"Found sticker '{sticker['name']}'")
            extension = STICKER_EXTENSIONS[sticker["format_type"]]
            url = f"https://media.discordapp.net/stickers/{sticker['id']}.{extension}?size=4096"
            # Set passthrough=false for APNGs if you don't want animated
            filename = sticker_dir / f"{sticker['name']}.{extension}"
            print(f"Downloading {url} to {filename}")
            download_file(url, filename)

        for emoji in guild["emojis"]:
            print(f"Found emoji '{emoji['name']}'")
            extension = "gif" if emoji["animated"] else "png"
            url = f"https://cdn.discordapp.com/emojis/{emoji['id']}.{extension}"
            filename = emoji_dir / f"{emoji['name']}.{extension}"
            print(f"Downloading {url} to {filename}")
            download_file(url, filename)


if __name__ == "__main__":
    main(*sys.argv)
