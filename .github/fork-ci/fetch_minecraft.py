"""Download 26.3's version JSON, client and server jars, libraries and assets into DIR, checking each SHA-1."""
import concurrent.futures
import hashlib
import json
import pathlib
import sys
import urllib.request

MANIFEST = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
VERSION = "26.3"


def get(url):
    with urllib.request.urlopen(url, timeout=120) as response:
        return response.read()


def fetch(url, sha1, path):
    if path.exists() and hashlib.sha1(path.read_bytes()).hexdigest() == sha1:
        return
    data = get(url)
    if hashlib.sha1(data).hexdigest() != sha1:
        raise SystemExit(f"SHA-1 mismatch for {url}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)


def main(root):
    root = pathlib.Path(root)
    entry = next(v for v in json.loads(get(MANIFEST))["versions"] if v["id"] == VERSION)
    raw = get(entry["url"])
    if hashlib.sha1(raw).hexdigest() != entry["sha1"]:
        raise SystemExit("SHA-1 mismatch for the version JSON")
    version = json.loads(raw)
    folder = root / "versions" / VERSION
    folder.mkdir(parents=True, exist_ok=True)
    (folder / f"{VERSION}.json").write_bytes(raw)
    downloads = version["downloads"]
    jobs = [
        (downloads["client"]["url"], downloads["client"]["sha1"], folder / f"{VERSION}.jar"),
        (downloads["server"]["url"], downloads["server"]["sha1"], root / "server.jar"),
    ]
    for library in version["libraries"]:
        artifact = library.get("downloads", {}).get("artifact")
        if artifact:
            jobs.append((artifact["url"], artifact["sha1"], root / "libraries" / artifact["path"]))
    index_info = version["assetIndex"]
    index_raw = get(index_info["url"])
    if hashlib.sha1(index_raw).hexdigest() != index_info["sha1"]:
        raise SystemExit("SHA-1 mismatch for the asset index")
    index_path = root / "assets" / "indexes" / f"{index_info['id']}.json"
    index_path.parent.mkdir(parents=True, exist_ok=True)
    index_path.write_bytes(index_raw)
    for obj in json.loads(index_raw)["objects"].values():
        h = obj["hash"]
        jobs.append((f"https://resources.download.minecraft.net/{h[:2]}/{h}", h,
                     root / "assets" / "objects" / h[:2] / h))
    with concurrent.futures.ThreadPoolExecutor(16) as pool:
        for future in concurrent.futures.as_completed([pool.submit(fetch, *job) for job in jobs]):
            future.result()
    print(f"{len(jobs)} files in {root}")


if __name__ == "__main__":
    main(sys.argv[1])
