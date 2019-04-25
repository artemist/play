#!/usr/bin/env python3
import os
import os.path
import hashlib
import html
orig_path = '../'
new_path = '.'

def format_name(name):
    name = html.unescape(name)
    name = name.replace('The - ', 'The ')
    return name

processed = [name.strip() for name in open(os.path.join(orig_path, "processed.txt")).readlines()]

games = dict()

for dfile in 'gc.dat', 'wii.dat':
    lines = open(os.path.join(orig_path, dfile)).readlines()
    lines = [line.strip() for line in lines if line.strip().startswith('<rom name=')]
    for line in lines:
        ns = line.find("name=\"") + len("name=\"")
        ne = line.find('"', ns)
        hs = line.find("sha1=\"") + len("sha1=\"")
        he = line.find('"', hs)
        name = line[ns:ne]
        hsh = line[hs:he]
        games[hsh] = format_name(name)


isos = [fn for fn in os.listdir(orig_path) if fn.endswith('.iso')]
for iso_name in isos:
    if iso_name in processed:
        print(f'Already processed {iso_name}, skipping')
        continue
    iso = os.path.join(orig_path, iso_name)
    h = hashlib.sha1()
    with open(iso, 'rb') as isof:
        buf = isof.read(2**16)
        while len(buf) > 0:
            h.update(buf)
            buf = isof.read(2**16)
    hsh = h.hexdigest()
    print(hsh)
    if hsh not in games:
        print(f"Warning! File {iso_name} with hash {hsh} was not found in game database")
        print("Not counting as processed, will be processed next time")
        isos.remove(iso_name)
        continue
    name = games[hsh]
    print(name)
    path = os.path.join(new_path, name)
    if os.path.exists(path):
        print(f"'{path}' already exists, skipping")
    else:
        os.link(iso, os.path.join(new_path, name))

f = open(os.path.join(orig_path, "processed.txt"), "w")
f.write("\n".join(isos))
f.close()
