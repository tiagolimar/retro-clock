use crate::config::Theme;
use crate::config::ShadowLevel;

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub background: u32,
    pub panel: u32,
    pub shadow: u32,
    pub muted: u32,
    pub accent: u32,
    pub highlight: u32,
}

pub fn segment_shadow(palette: Palette, level: ShadowLevel) -> Option<u32> {
    match level {
        ShadowLevel::Full => Some(palette.shadow),
        ShadowLevel::Half => Some(blend_half(palette.background, palette.shadow)),
        ShadowLevel::None => None,
    }
}

fn blend_half(first: u32, second: u32) -> u32 {
    let red = (((first >> 16) & 0xff) + ((second >> 16) & 0xff)) / 2;
    let green = (((first >> 8) & 0xff) + ((second >> 8) & 0xff)) / 2;
    let blue = ((first & 0xff) + (second & 0xff)) / 2;

    (red << 16) | (green << 8) | blue
}

pub fn for_theme(theme: Theme) -> Palette {
    match theme {
        Theme::Green => Palette {
            background: 0x000000,
            panel: 0x0014261a,
            shadow: 0x00152b15,
            muted: 0x002c7a3c,
            accent: 0x0000ff66,
            highlight: 0x0066ff99,
        },
        Theme::Amber => Palette {
            background: 0x00080400,
            panel: 0x00201608,
            shadow: 0x003a250b,
            muted: 0x008f5e18,
            accent: 0x00ffad32,
            highlight: 0x00ffd27a,
        },
        Theme::Blue => Palette {
            background: 0x00000610,
            panel: 0x00101c32,
            shadow: 0x00132e49,
            muted: 0x002d668d,
            accent: 0x0047c7ff,
            highlight: 0x0098e6ff,
        },
        Theme::Red => Palette {
            background: 0x00120307,
            panel: 0x002a1015,
            shadow: 0x004f1420,
            muted: 0x00b74354,
            accent: 0x00ff4d62,
            highlight: 0x00ffa1aa,
        },
        Theme::Violet => Palette {
            background: 0x000b0415,
            panel: 0x00211237,
            shadow: 0x00462568,
            muted: 0x009254c7,
            accent: 0x00c878ff,
            highlight: 0x00efc1ff,
        },
        Theme::Cyan => Palette {
            background: 0x00001215,
            panel: 0x00082a30,
            shadow: 0x000e5156,
            muted: 0x001ca6a6,
            accent: 0x0020f0e4,
            highlight: 0x00a5fff7,
        },
        Theme::White => Palette {
            background: 0x00080b10,
            panel: 0x001b2029,
            shadow: 0x002e3845,
            muted: 0x008291a3,
            accent: 0x00dbe7f3,
            highlight: 0x00ffffff,
        },
    }
}
