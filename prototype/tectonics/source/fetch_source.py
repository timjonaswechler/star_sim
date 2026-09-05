#!/usr/bin/env python3
"""Regeneriert die lokalen Quelldaten fuer den Tectonics-Prototyp.

Laedt den Worldbuilding-Pasta-Artikel (Part Va: Tectonics), extrahiert den
Text ab dem Anker #patternsofplatemotion, laedt die 107 Abschnittsbilder
und baut daraus beschriftete Contact Sheets.

Ergebnis in prototype/tectonics/source/:
  post.md, image_map.txt, section_images.txt  -> eingecheckt (Text)
  post.html, imgs/, contact_*.png             -> lokal only (siehe .gitignore)

Abhaengigkeiten: beautifulsoup4, Pillow. Beispiel:
  python prototype/tectonics/source/fetch_source.py
"""

from __future__ import annotations

import re
import time
import urllib.request
from pathlib import Path

URL = "https://worldbuildingpasta.blogspot.com/2020/01/an-apple-pie-from-scratch-part-va.html"
OUT_DIR = Path(__file__).resolve().parent
IMG_DIR = OUT_DIR / "imgs"

HEADING = {"h1": "#", "h2": "##", "h3": "###", "h4": "####"}
BLOCKS = set(HEADING) | {"p", "li", "div", "blockquote", "figcaption", "caption", "pre", "td"}
GIF_IDX = 7  # 1-based: Pangea-Animation
GIF_FRAMES = 12
THUMB_W = 480
COLS = 4


def fetch_html() -> str:
    req = urllib.request.Request(URL, headers={"User-Agent": "Mozilla/5.0 (research export)"})
    with urllib.request.urlopen(req, timeout=90) as r:
        return r.read().decode("utf-8", errors="replace")


def export_text(html: str) -> tuple[str, list[str], list[str]]:
    from bs4 import BeautifulSoup, Tag

    soup = BeautifulSoup(html, "html.parser")
    body = soup.find("div", class_="post-body") or soup.find("article") or soup.body
    assert body is not None, "no post body found"
    anchor = body.find(id=re.compile(r"patternsofplatemotion", re.I))
    if anchor is None:
        anchor = body.find("a", attrs={"name": re.compile(r"patternsofplatemotion", re.I)})
    assert anchor is not None, "anchor #patternsofplatemotion not found"

    md: list[str] = []
    urls: list[str] = []
    ctx: list[str] = []
    last = ""
    past = False
    for el in body.descendants:
        if el is anchor:
            past = True
            continue
        if not past or not isinstance(el, Tag):
            continue
        if el.name in HEADING:
            text = el.get_text(" ", strip=True)
            if text:
                md.append(f"\n{HEADING[el.name]} {text}\n")
                last = text[:300]
        elif el.name == "img":
            src = el.get("src") or el.get("data-src") or ""
            if not src or "icon" in src or "emoji" in src or src in urls:
                continue
            src = re.sub(r"/s\d+(-[^/]*)?/", "/s1600/", src)
            urls.append(src)
            ctx.append(last)
            alt = (el.get("alt") or "").strip()
            md.append(f"\n[IMAGE {len(urls):03d}: {alt}]({src})\n")
        elif el.name in BLOCKS and not el.find(list(BLOCKS - {"td"})):
            text = el.get_text(" ", strip=True)
            if text:
                md.append(text + "\n")
                last = text[:300]
    raw = re.sub(r"\n{3,}", "\n\n", "\n".join(md)).strip() + "\n"
    return raw, urls, ctx


def download(urls: list[str]) -> None:
    IMG_DIR.mkdir(parents=True, exist_ok=True)
    for i, url in enumerate(urls, start=1):
        tail = url.split("/")[-1].split("?")[0]
        ext = "." + tail.rsplit(".", 1)[-1].lower() if "." in tail else ".jpg"
        if ext not in {".jpg", ".jpeg", ".png", ".gif", ".webp"}:
            ext = ".jpg"
        dest = IMG_DIR / f"{i:03d}{ext}"
        if dest.exists() and dest.stat().st_size > 0:
            print(f"[{i}/{len(urls)}] {dest.name} (cached)")
            continue
        print(f"[{i}/{len(urls)}] {dest.name}", flush=True)
        req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 (research export)"})
        for attempt in range(3):
            try:
                with urllib.request.urlopen(req, timeout=60) as r, open(dest, "wb") as f:
                    f.write(r.read())
                break
            except Exception as e:  # noqa: BLE001
                print(f"  versuch {attempt + 1} fehlgeschlagen: {e}")
                time.sleep(2 * (attempt + 1))
        else:
            print(f"  FEHLER: {url}")
        time.sleep(0.3)


def contact_sheets() -> None:
    from PIL import Image, ImageDraw

    def thumb(im: Image.Image, label: str) -> Image.Image:
        t = im.resize((THUMB_W, max(1, round(im.size[1] * THUMB_W / im.size[0]))), Image.LANCZOS)
        d = ImageDraw.Draw(t)
        d.rectangle([0, 0, THUMB_W, 30], fill=(180, 0, 0))
        d.text((8, 8), label, fill=(255, 255, 255))
        return t

    def sheet(thumbs: list[Image.Image], out: Path) -> None:
        row_h = max(t.size[1] for t in thumbs)
        rows = (len(thumbs) + COLS - 1) // COLS
        canvas = Image.new("RGB", (COLS * THUMB_W, rows * row_h), (20, 20, 20))
        for k, t in enumerate(thumbs):
            canvas.paste(t, ((k % COLS) * THUMB_W, (k // COLS) * row_h))
        canvas.save(out)
        print(f"  {out.name} {canvas.size} ({len(thumbs)} bilder)")

    stills: list[tuple[int, Path]] = []
    for i in range(1, len(list(IMG_DIR.glob("???*"))) + 1):
        if i == GIF_IDX:
            continue
        hit = next((IMG_DIR / f"{i:03d}{e}" for e in (".png", ".jpg", ".jpeg", ".webp") if (IMG_DIR / f"{i:03d}{e}").exists()), None)
        if hit is not None:
            stills.append((i, hit))
    per_sheet = (len(stills) + 6) // 7
    for s in range(7):
        chunk = stills[s * per_sheet:(s + 1) * per_sheet]
        if not chunk:
            break
        thumbs = []
        for idx, p in chunk:
            try:
                im = Image.open(p)
                if getattr(im, "n_frames", 1) > 1:
                    im.seek(0)
                thumbs.append(thumb(im.convert("RGB"), f"{idx:03d}"))
            except Exception as e:  # noqa: BLE001
                print(f"  skip {p.name}: {e}")
        sheet(thumbs, OUT_DIR / f"contact_{s:02d}.png")

    gif_path = next((IMG_DIR / f"007{e}" for e in (".gif", ".GIF") if (IMG_DIR / f"007{e}").exists()), None)
    assert gif_path is not None, "007-gif fehlt"
    gif = Image.open(gif_path)
    n = getattr(gif, "n_frames", 1)
    frames = []
    for j in [round(i * (n - 1) / (GIF_FRAMES - 1)) for i in range(GIF_FRAMES)]:
        gif.seek(j)
        fr = gif.convert("RGB").resize((THUMB_W, round(gif.size[1] * THUMB_W / gif.size[0])), Image.LANCZOS)
        d = ImageDraw.Draw(fr)
        d.rectangle([0, 0, THUMB_W, 30], fill=(180, 0, 0))
        d.text((8, 8), f"007 frame {j}", fill=(255, 255, 255))
        frames.append(fr)
    sheet(frames, OUT_DIR / "contact_gif_007.png")


def main() -> None:
    print(f"hole {URL}")
    html = fetch_html()
    (OUT_DIR / "post.html").write_text(html, encoding="utf-8")
    print("extrahiere text + bildliste")
    raw, urls, ctx = export_text(html)
    (OUT_DIR / "post.md").write_text(raw, encoding="utf-8")
    (OUT_DIR / "section_images.txt").write_text("\n".join(urls) + "\n", encoding="utf-8")
    with (OUT_DIR / "image_map.txt").open("w", encoding="utf-8") as f:
        for i, (url, c) in enumerate(zip(urls, ctx), start=1):
            fname = url.split("/")[-1].split("?")[0][:120]
            f.write(f"{i:03d} | {fname} | {url}\n     nearby: {c}\n")
    print(f"post.md: {len(raw.splitlines())} zeilen, {len(urls)} bilder")
    print("lade bilder")
    download(urls)
    print("baue contact sheets")
    contact_sheets()
    print("fertig.")


if __name__ == "__main__":
    main()
