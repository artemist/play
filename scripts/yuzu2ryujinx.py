#!/usr/bin/env nix-shell
#!nix-shell -i python3 -p python3

'''
Convert Yuzu saves to Ryujinx by putting them in profile 0 (RyuPlayer)
Note that this is rather destructive and won't deal well with multiple profiles,
so back saves up before trying it

(if anyone wants to modify it, it's CC0)
'''

from pathlib import Path
import os
import shutil
import struct
import typing

RYUJINX_DIR = Path.home() / ".config" / "Ryujinx"
YUZU_DIR = Path.home() / ".local" / "share" / "yuzu"

def get_yuzu_save_dir():
    profile_path = YUZU_DIR / "nand" / "system" / "save" / "8000000000000010" / "su" / "avators" / "profiles.dat"
    with open(profile_path, "rb") as f:
        # Just go with profile 0
        uid_low, uid_high = struct.unpack("<QQ", f.read(0x20)[0x10:])
    return YUZU_DIR / "nand" / "user" / "save" / "0000000000000000" / f"{uid_high:016X}{uid_low:016X}"

RYUJINX_SAVE_DIR = RYUJINX_DIR / "bis" / "user" / "save"
YUZU_SAVE_DIR = get_yuzu_save_dir()


titles: typing.Dict[str, Path] = dict()

# The first 8 bytes of ExtraData0 are the program ID, next 16 are user ID.
# Ignore ExtraData1, that's just for journaling
# https://github.com/Thealexbarney/LibHac/blob/master/src/LibHac/Fs/Common/SaveDataTypes.cs#L97
# https://github.com/Thealexbarney/LibHac/blob/master/src/LibHac/Fs/Common/SaveDataTypes.cs#L131
for child in RYUJINX_SAVE_DIR.iterdir():
    with open(child / "ExtraData0", "rb") as extra:
        data = extra.read(24)
        title_id, *uid = struct.unpack("<QQQ", data)
    if uid not in ([0, 0], [1, 0]):
        # Always use default profile
        print("Found and ignored save for user {:016X}{:016X} ({:016X})".format(*uid, title_id))
        continue
    titles[f"{title_id:016X}"] = child / "0"

for title_id, ryujinx_dir in titles.items():
    yuzu_dir = YUZU_SAVE_DIR / title_id
    if not yuzu_dir.exists():
        continue
    
    print(yuzu_dir, ryujinx_dir)

    shutil.rmtree(ryujinx_dir)
    shutil.copytree(yuzu_dir, ryujinx_dir)
