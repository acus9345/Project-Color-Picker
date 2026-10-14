#![windows_subsystem = "windows"]

use macroquad::miniquad::window::{clipboard_get, clipboard_set, set_mouse_cursor};
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, CursorIcon, Equation};
use macroquad::prelude::*;

// default window size = the design canvas times this (change it whenever you want)
const WINDOW_SCALE: f32 = 1.0;

// the ui is laid out on a fixed canvas (title bar + content block) and scaled
// uniformly to fit whatever size the window is
const TITLE_H: f32 = 32.0;
const CONTENT_W: f32 = 520.0;
const CONTENT_H: f32 = 298.0;
const WINDOW_W: f32 = CONTENT_W * WINDOW_SCALE;
const WINDOW_H: f32 = (TITLE_H + CONTENT_H) * WINDOW_SCALE;

fn conf() -> Conf {
    let icon = include_bytes!("../assets/icon.png");
    let iconimg = image::load_from_memory(icon).expect("Failed to load icon").to_rgba8();
    let s = image::imageops::resize(&iconimg,16,16,image::imageops::FilterType::Lanczos3).into_raw();
    let m = image::imageops::resize(&iconimg,32,32,image::imageops::FilterType::Lanczos3).into_raw();
    let l = image::imageops::resize(&iconimg,64,64,image::imageops::FilterType::Lanczos3).into_raw();
    Conf {
        window_resizable: true,
        window_height: WINDOW_H as i32,
        window_width: WINDOW_W as i32,
        window_title: "Project Color picker".to_owned(),
        icon: Some(miniquad::conf::Icon{
            small:s.try_into().unwrap(),
            medium:m.try_into().unwrap(),
            big:l.try_into().unwrap(),
        }),
        high_dpi: true,
        ..Default::default()
    }
}
// ----- theme -----
const BG: Color = Color::from_rgba(30, 30, 28, 255);
const SURFACE: Color = Color::from_rgba(44, 44, 41, 255);
const SURFACE_HI: Color = Color::from_rgba(58, 58, 54, 255);
const POPOVER: Color = Color::from_rgba(38, 38, 35, 255);
const STROKE: Color = Color::new(1.0, 1.0, 1.0, 0.08);
const STROKE_HI: Color = Color::new(1.0, 1.0, 1.0, 0.15);
const TEXT: Color = Color::from_rgba(236, 236, 238, 255);
const TEXT_DIM: Color = Color::from_rgba(142, 142, 147, 255);
const DANGER: Color = Color::from_rgba(255, 105, 97, 255);

// ----- custom title bar -----
const TITLE_TEXT: &str = "Project Color Picker";
const TITLE_BTN_W: f32 = 40.0;
/// window border color matching BG (COLORREF = 0x00BBGGRR)
const TITLE_CAPTION_BGR: u32 = 0x001C_1E1E;

const LAB_IMG_X_MIN:f64= 0.0039; // locus minimum x (~504 nm)
const LAB_IMG_X_MAX:f64= 0.7347; // red tip (700 nm)
const LAB_IMG_Y_MIN:f64= 0.0048; // violet end (~400 nm)
const LAB_IMG_Y_MAX:f64= 0.8338; // green apex (~520 nm)zzz did i learn enough cie 1931 chromaticity
const LAB_TEX_W: u16=512;
const LAB_TEX_H:u16=581;


fn lab_img_x(u: f32) -> f64 {
    LAB_IMG_X_MIN +(LAB_IMG_X_MAX - LAB_IMG_X_MIN) * clamp01(u) as f64
}


fn lab_img_y(v:f32)-> f64 {
    LAB_IMG_Y_MIN +(LAB_IMG_Y_MAX - LAB_IMG_Y_MIN) * clamp01(v) as f64
}


fn lab_img_u(x:f64)-> f32 {
    ((x - LAB_IMG_X_MIN) /(LAB_IMG_X_MAX - LAB_IMG_X_MIN)).clamp(0.0, 1.0) as f32
}


fn lab_img_v(y:f64)-> f32 {
    ((y- LAB_IMG_Y_MIN) /(LAB_IMG_Y_MAX - LAB_IMG_Y_MIN)).clamp(0.0, 1.0) as f32
}


fn clamp01(v:f32) -> f32 {
    v.clamp(0.0,1.0) }


fn hue_from_t(t:f32) -> f32 {
    (t*360.0).min(359.9999)
}
//ctrl c ctrl v
const SPECTRAL_LOCUS: [(f64, f64, f64); 65] = [
    (380.0,0.1741, 0.0050),
    (385.0, 0.1740, 0.0050),
    (390.0, 0.1738, 0.0049),
    (395.0, 0.1736, 0.0049),
    (400.0, 0.1733, 0.0048),
    (405.0, 0.1730, 0.0048),
    (410.0, 0.1726, 0.0048),
    (415.0, 0.1721, 0.0048),
    (420.0, 0.1714, 0.0051),
    (425.0, 0.1703, 0.0058),
    (430.0, 0.1689, 0.0069),
    (435.0, 0.1669, 0.0086),
    (440.0, 0.1644, 0.0109),
    (445.0, 0.1611, 0.0138),
    (450.0, 0.1566, 0.0177),
    (455.0, 0.1510, 0.0227),
    (460.0, 0.1440, 0.0297),
    (465.0, 0.1355, 0.0399),
    (470.0, 0.1241, 0.0578),
    (475.0, 0.1096, 0.0868),
    (480.0, 0.0913, 0.1327),
    (485.0, 0.0687, 0.2007),
    (490.0, 0.0454, 0.2950),
    (495.0, 0.0235, 0.4127),
    (500.0, 0.0082, 0.5384),
    (505.0, 0.0039, 0.6548),
    (510.0, 0.0139, 0.7502),
    (515.0, 0.0389, 0.8120),
    (520.0, 0.0743, 0.8338),
    (525.0, 0.1142, 0.8262),
    (530.0, 0.1547, 0.8059),
    (535.0, 0.1929, 0.7816),
    (540.0, 0.2296, 0.7543),
    (545.0, 0.2658, 0.7243),
    (550.0, 0.3016, 0.6923),
    (555.0, 0.3374, 0.6588),
    (560.0, 0.3731, 0.6245),
    (565.0, 0.4087, 0.5896),
    (570.0, 0.4441, 0.5547),
    (575.0, 0.4788, 0.5202),
    (580.0, 0.5125, 0.4866),
    (585.0, 0.5448, 0.4544),
    (590.0, 0.5752, 0.4242),
    (595.0, 0.6029, 0.3965),
    (600.0, 0.6270, 0.3725),
    (605.0, 0.6482, 0.3514),
    (610.0, 0.6658, 0.3340),
    (615.0, 0.6801, 0.3197),
    (620.0, 0.6915, 0.3083),
    (625.0, 0.7006, 0.2993),
    (630.0, 0.7079, 0.2920),
    (635.0, 0.7140, 0.2859),
    (640.0, 0.7190, 0.2809),
    (645.0, 0.7230, 0.2769),
    (650.0, 0.7260, 0.2740),
    (655.0, 0.7283, 0.2717),
    (660.0, 0.7300, 0.2700),
    (665.0, 0.7311, 0.2689),
    (670.0, 0.7320, 0.2680),
    (675.0, 0.7327, 0.2673),
    (680.0, 0.7334, 0.2666),
    (685.0, 0.7340, 0.2660),
    (690.0, 0.7344, 0.2656),
    (695.0, 0.7346, 0.2654),
    (700.0, 0.7347, 0.2653), ];

// is the chromaticity inside the spectral horseshoe (locus + purple line)?
fn in_locus_hull(x:f64,y:f64)-> bool {
    let n= SPECTRAL_LOCUS.len();
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let(x1,y1) =(SPECTRAL_LOCUS[i].1, SPECTRAL_LOCUS[i].2);
        let(x2, y2) =(SPECTRAL_LOCUS[j].1, SPECTRAL_LOCUS[j].2);
        if((y1>y) !=(y2>y)) && x < (x2- x1) * (y- y1) /(y2- y1) +x1{
            inside =!inside;
        }
        j =i; }
    inside }

fn clamp_to_locus(x:f64,y:f64) ->(f64, f64) {
    if in_locus_hull(x,y) {
        return (x,y);
    }
    let (wx, wy) = (0.3127, 0.3290);
    let dx =x -wx;
    let dy=y -wy;
    if dx* dx+dy *dy<1e-12 {
        return (wx,wy);
    }
    // first crossing of the ray W + t*(dx, dy) with the locus polygon
    let mut t_exit =f64::INFINITY;
    let n =SPECTRAL_LOCUS.len();
    for i in 0..n {
        let j=(i +1) % n;
        let (ax,ay)=(SPECTRAL_LOCUS[i].1,SPECTRAL_LOCUS[i].2);
        let (bx,by)=(SPECTRAL_LOCUS[j].1,SPECTRAL_LOCUS[j].2);
        let (ex,ey) =(bx -ax, by -ay);
        let det= -(dx*ey-dy*ex);
        if det.abs()< 1e-15 {
            continue; }
        let t= (-(ax - wx) * ey + ex * (ay -wy)) / det;
        let s= (dx * (ay -wy) - dy * (ax -wx)) / det;
        if (0.0..=1.0).contains(&s) && t > 1e-9 && t< t_exit {
            t_exit =t;
        }
    }
    if !t_exit.is_finite() {
        return (x,y); //the white point is strictly inside
    }
    // pull a hair inside so boundary float ambiguity cant reject the point
    let t=t_exit * 0.999;
    (wx + t * dx, wy + t * dy) }

fn diagram_rgb(x:f64, y:f64)->(u8, u8, u8) {
    if y <= 1e-9 || !in_locus_hull(x,y) {
        return (0,0, 0);
    }
    let big_x= x /y; // Y = 1
    let big_z= (1.0-x - y) / y;
    let rl= 3.2404542 * big_x - 1.5371385 - 0.4985314 * big_z;
    let gl =-0.9692660 * big_x + 1.8760108 + 0.0415560 * big_z;
    let bl =0.0556434 * big_x - 0.2040259 + 1.0572252 * big_z;
    let m = rl.max(gl).max(bl);
    if m <=1e-6{ return (0,0,0); }
    let ch=|c: f64|q8(linear_to_srgb((c / m).clamp(0.0,1.0)) as f32);
    (ch(rl), ch(gl), ch(bl))
}


/// The filled horseshoe, transparent outside the locus.
fn lab_diagram_bytes(w: u16, h: u16) -> Vec<u8> {
    let (w, h) = (w as u32, h as u32);
    let dx =(LAB_IMG_X_MAX - LAB_IMG_X_MIN)/ w as f64;
    let dy = (LAB_IMG_Y_MAX - LAB_IMG_Y_MIN)/ h as f64;
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for row in 0..h {
        let y =LAB_IMG_Y_MAX - ((row as f64 + 0.5) / h as f64) * (LAB_IMG_Y_MAX - LAB_IMG_Y_MIN);
        for col in 0..w {
            let x =LAB_IMG_X_MIN
                + ((col as f64 + 0.5) / w as f64) *(LAB_IMG_X_MAX -LAB_IMG_X_MIN);
            let mut c = diagram_rgb(x, y);
            // one extra ring of pixels just outside, so the outline has color under it
            if c == (0, 0, 0)
                && (in_locus_hull(x + dx, y)
                    || in_locus_hull(x - dx, y)
                    || in_locus_hull(x, y + dy)
                    || in_locus_hull(x, y - dy))
            { c = diagram_rgb(x + (0.3127 - x) * 0.02, y + (0.3290 - y) * 0.02, ); }
            let a = if c == (0, 0, 0) { 0 } else { 255 };
            out.extend_from_slice(&[c.0, c.1, c.2, a]); } }
    out }

/// Axis range the CIE plot shows; a little wider than the locus so the
/// wavelength labels fit between it and the axes.
const CIE_X0: f64 = -0.07;
const CIE_X1: f64 = 0.78;
const CIE_Y1: f64 = 0.88;

/// Where the horseshoe texture goes inside the plane square `sv`.
fn cie_rect(sv: Rect) -> Rect {
    let unit = (sv.h - 26.0) / CIE_Y1 as f32; // design units per 1.0 of x or y
    let frame_x = sv.x + 26.0;
    let frame_y = sv.y + 8.0;
    Rect::new(
        frame_x + (LAB_IMG_X_MIN - CIE_X0) as f32 * unit,
        frame_y + (CIE_Y1 - LAB_IMG_Y_MAX) as f32 * unit,
        (LAB_IMG_X_MAX - LAB_IMG_X_MIN) as f32 * unit,
        (LAB_IMG_Y_MAX - LAB_IMG_Y_MIN) as f32 * unit,
    )
}

// ------- color space conversions ----------

fn hsv_to_rgb_f(h:f32,s:f32,v:f32)->(f32, f32, f32) {
    let c=v*s;
    let hp=(h/60.0).rem_euclid(6.0);
    let x =c*(1.0-(hp.rem_euclid(2.0)-1.0).abs());
    let (r1,g1,b1) =match hp as u32 {
        0=> (c,x,0.0),
        1=> (x,c,0.0),
        2=> (0.0,c,x),
        3=> (0.0,x,c),
        4=> (x,0.0,c),
        _ => (c,0.0,x), };
    let m=v-c;
    (r1+m,g1+m,b1 +m)
}

fn hue_of(r:f32, g:f32,b:f32,max:f32,min: f32)-> f32 {
    let d = max -min;
    if d <= 1e-6 { 0.0 } else if (max- r).abs() <= 1e-6 {
        60.0* ((g- b)/ d).rem_euclid(6.0)}else if(max -g).abs()<=1e-6 {
        60.0 *((b- r) /d +2.0) } else {
        60.0 * ((r- g) /d +4.0) } }

fn rgb_to_hsv_f(r:f32, g:f32, b:f32) -> (f32,f32,f32) {
    let max =r.max(g).max(b);
    let min =r.min(g).min(b);
    let h = hue_of(r, g, b, max, min);
    let s = if max <= 1e-6 { 0.0 } else { (max - min) / max };
    (h, s, max) }

fn rgb_to_hsl(r:f32,g:f32,b:f32)->(f32,f32,f32) { let max = r.max(g).max(b);
    let min =r.min(g).min(b);let h = hue_of(r, g, b, max, min);
    let l =(max + min) * 0.5;let d = max - min;
    let s =if d <= 1e-6 {
        0.0 }else { (d / (1.0 - (2.0 * l - 1.0).abs()).max(1e-6)).min(1.0)
    };
    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = (h / 60.0).rem_euclid(6.0);
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c * 0.5;
    (r1 + m, g1 + m, b1 + m)
}

fn rgb_to_hwb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    (hue_of(r, g, b, max, min), min, 1.0 - max)
}

fn hwb_to_rgb(h: f32, w: f32, bl: f32) -> (f32, f32, f32) {
    if w + bl >= 1.0 {
        let gy = w / (w + bl);
        return (gy, gy, gy);
    }
    let (pr, pg, pb) = hsv_to_rgb_f(h, 1.0, 1.0);
    let k = 1.0 - w - bl;
    (pr * k + w, pg * k + w, pb * k + w)
}

fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> (f32, f32, f32, f32) {
    let k = 1.0 - r.max(g).max(b);
    if k >= 1.0 {
        (0.0, 0.0, 0.0, 1.0)
    } else {
        (
            (1.0 - r - k) / (1.0 - k),
            (1.0 - g - k) / (1.0 - k),
            (1.0 - b - k) / (1.0 - k), k,) } }

fn cmyk_to_rgb(c: f32, m:f32,y: f32,k:f32) -> (f32,f32,f32) {
    ((1.0 - c) * (1.0 - k), (1.0 - m) * (1.0 - k), (1.0 - y) * (1.0 - k)) }

// ---------- YUV / YPbPr (BT.601, analog ranges) ----------

fn rgb_to_yuv(r:f32,g: f32, b: f32) -> (f32, f32, f32) {
    let y = 0.299 *r + 0.587 * g + 0.114 * b;
    (y, 0.492 * (b - y), 0.877 * (r - y)) }

fn yuv_to_rgb(y: f32, u: f32, v: f32) -> (f32, f32, f32) {
    (
        y + 1.140 * v,
        y - 0.394 * u - 0.581 * v,
        y + 2.032 * u,
    )
}
fn rgb_to_ypbpr(r:f32, g:f32,b:f32) -> (f32, f32, f32) {
    let y =0.299*r +0.587 *g +0.114 *b;
    (y, (b -y) / 1.772, (r -y) /1.402) }

fn ypbpr_to_rgb(y: f32, pb: f32, pr: f32) -> (f32, f32, f32) {
    (
        y + 1.402 * pr,
        y - 0.344136 * pb - 0.714136 * pr,
        y + 1.772 * pb,) }

// ---------- Munsell (aka MHVC)


const MUNSELL_C_MAX: f32 = 30.0;
fn munsell_to_rgb(h:f32, v:f32, c:f32) -> (f32, f32, f32) {
    let s =(c /(c +12.0)).clamp(0.0,1.0);
    hsv_to_rgb_f(h *3.6,s,(v / 10.0).clamp(0.0, 1.0))
}

fn rgb_to_munsell(r:f32, g:f32, b: f32)->(f32, f32, f32) {
    let (h, s,v) =rgb_to_hsv_f(r, g, b);
    let c = if s >= 0.999 {
        MUNSELL_C_MAX
    } else {
        (12.0 * s/(1.0 -s)).min(MUNSELL_C_MAX)
    };
    (h / 3.6, v * 10.0, c)
}

// ---------- OkLab / OKLCH / CIE Lab --------
//prob the most complicated ima just copy and paste
fn srgb_to_linear(c:f64) -> f64 { if c <=0.04045 { c/ 12.92 }else{ ((c +0.055) / 1.055).powf(2.4) }
}

fn linear_to_srgb(c:f64) -> f64 {
    if c <=0.0031308 {c* 12.92}else { 1.055 *c.powf(1.0 /2.4) -0.055 }
}

fn rgb_to_oklch(r: f32, g: f32, b: f32) -> (f64, f64, f64) {
    let r = srgb_to_linear(r as f64);
    let g = srgb_to_linear(g as f64);
    let b = srgb_to_linear(b as f64);

    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    let L = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let bb = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

    let c = (a * a + bb * bb).sqrt();
    let mut h = bb.atan2(a).to_degrees();
    if h < 0.0 { h += 360.0; }
    (L, c, h)
}

/// OKLCH to linear sRGB, not clamped: channels outside 0..1 mean the color
/// can't be shown.
fn oklch_to_linear(lightness: f64, c: f64, h: f64) -> (f64, f64, f64) {
    let hr = h.to_radians();
    let a = c * hr.cos();
    let bb = c * hr.sin();

    let l_ = lightness + 0.3963377774 * a + 0.2158037573 * bb;
    let m_ = lightness - 0.1055613458 * a - 0.0638541728 * bb;
    let s_ = lightness - 0.0894841775 * a - 1.2914855480 * bb;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    (
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    )
}

fn oklch_to_rgb(lightness: f64, c: f64, h: f64) -> (f32, f32, f32) {
    let (r, g, b) = oklch_to_linear(lightness, c, h);
    (
        linear_to_srgb(r.clamp(0.0, 1.0)) as f32,
        linear_to_srgb(g.clamp(0.0, 1.0)) as f32,
        linear_to_srgb(b.clamp(0.0, 1.0)) as f32,
    )
}
fn rgb_to_lab(r: f32, g: f32, b: f32) -> (f64, f64, f64) {
    let rl = srgb_to_linear(r as f64);
    let gl = srgb_to_linear(g as f64);
    let bl = srgb_to_linear(b as f64);

    let x = 0.4124564 * rl + 0.3575761 * gl + 0.1804375 * bl;
    let y = 0.2126729 * rl + 0.7151522 * gl + 0.0721750 * bl;
    let z = 0.0193339 * rl + 0.1191920 * gl + 0.9503041 * bl;

    let f = |t: f64| if t > 0.008856452 { t.cbrt() } else { 7.787 * t + 16.0 / 116.0 };
    let fx = f(x / 0.95047);
    let fy = f(y);
    let fz = f(z / 1.08883);

    (116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
}

fn lab_to_rgb(l: f64, a: f64, b: f64) -> (f32, f32, f32) {
    let fy = (l + 16.0) / 116.0;
    let fx = fy + a / 500.0;
    let fz = fy - b / 200.0;

    let inv = |t: f64| if t.powi(3) > 0.008856452 { t.powi(3) } else { (116.0 * t - 16.0) / 7.787 };
    let xr = inv(fx);
    let yr = if l > 7.9996 { fy.powi(3) } else { l / 903.3 };
    let zr = inv(fz);

    let x = xr * 0.95047;
    let y = yr;
    let z = zr * 1.08883;

    let rl = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let gl = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let bl = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

    (
        linear_to_srgb(rl.clamp(0.0, 1.0)) as f32,
        linear_to_srgb(gl.clamp(0.0, 1.0)) as f32,
        linear_to_srgb(bl.clamp(0.0, 1.0)) as f32,
    )
}

fn rgb_to_xyz(r: f32, g: f32, b: f32) -> (f64, f64, f64) {
    let rl = srgb_to_linear(r as f64);
    let gl = srgb_to_linear(g as f64);
    let bl = srgb_to_linear(b as f64);
    (
        0.4124564 * rl + 0.3575761 * gl + 0.1804375 * bl,
        0.2126729 * rl + 0.7151522 * gl + 0.0721750 * bl,
        0.0193339 * rl + 0.1191920 * gl + 0.9503041 * bl,
    )
}


fn rgb_to_xy(r: f32, g: f32, b: f32) -> (f32, f32) {
    let (x, y, z) = rgb_to_xyz(r, g, b);
    let s = (x + y + z).max(1e-9);
    ((x / s) as f32, (y / s) as f32)
}


fn xyy_to_xyz(x: f64, y: f64, big_y: f64)-> (f64, f64, f64) {
    (x / y * big_y, big_y, (1.0 - x - y)/y*big_y) }


fn xyz_to_rgb_f(x: f64, y: f64, z: f64) -> (f32, f32, f32) {
    let rl = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let gl = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let bl = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    (
        linear_to_srgb(rl.clamp(0.0, 1.0)) as f32,
        linear_to_srgb(gl.clamp(0.0, 1.0)) as f32,
        linear_to_srgb(bl.clamp(0.0, 1.0)) as f32,) }
fn luma8(r:f32,g:f32,b:f32)->u8{
    q8(0.2126 * r + 0.7152 * g + 0.0722 * b) }

fn q8(v:f32) -> u8 {
    (v.clamp(0.0,1.0) * 255.0).round() as u8 }

fn hsv_to_rgb(h:f32, s:f32, v:f32) -> (u8, u8, u8) {
    let (r, g,b) = hsv_to_rgb_f(h, s, v);
    (q8(r), q8(g), q8(b)) }

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t, ) }

fn hit(rect: Rect, m: Vec2, pad: f32) -> bool {
    m.x >= rect.x - pad
        && m.x <= rect.x + rect.w + pad
        && m.y >= rect.y - pad
        && m.y <= rect.y + rect.h + pad
}



// ---------- color modes -------- i hope i didnt start this at all and just do a rgba color picker

#[derive(Clone, Copy, PartialEq)]
enum ColorMode {
    Rgba,
    Hex,
    Hsl,
    Hsla,
    Hsb,
    Cmyk,
    Hwb,
    Oklch,
    Lab,
    Gray,
    Yuv,
    Ypbpr,
    Xyz,
    Xyy,
    Munsell,
}
//-----I CANT STOP---
const MODES: [ColorMode; 15] = [
    ColorMode::Rgba,
    ColorMode::Hex,
    ColorMode::Hsl,
    ColorMode::Hsla,
    ColorMode::Hsb,
    ColorMode::Cmyk,
    ColorMode::Hwb,
    ColorMode::Oklch,
    ColorMode::Lab,
    ColorMode::Gray,
    ColorMode::Yuv,
    ColorMode::Ypbpr,
    ColorMode::Xyz,
    ColorMode::Xyy,
    ColorMode::Munsell, ];
fn mode_name(m: ColorMode) -> &'static str {
    match m {
        ColorMode::Rgba => "RGBA",
        ColorMode::Hex => "HEX",
        ColorMode::Hsl => "HSL",
        ColorMode::Hsla => "HSLA",
        ColorMode::Hsb => "HSB",
        ColorMode::Cmyk => "CMYK",
        ColorMode::Hwb => "HWB",
        ColorMode::Oklch => "OKLCH",
        ColorMode::Lab => "LAB",
        ColorMode::Gray => "GRAY",
        ColorMode::Yuv => "YUV",
        ColorMode::Ypbpr => "YPBPR",
        ColorMode::Xyz =>"XYZ",
        ColorMode::Xyy =>"XYY",
        ColorMode::Munsell =>"MUNSELL", } }

fn num_tokens(s: &str) -> Vec<f32> {
    s.split([',', ' ', '\t'])
        .filter_map(|t| {
            let t:String = t
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '-' || *c == '.')
                .collect();
            t.parse().ok() })
        .collect() }

fn parse_rgba(s: &str) -> Option<(u8, u8, u8, Option<u8>)> {
    let parts: Vec<&str> = s.split(',').map(str::trim).collect();
    if parts.len() < 3 || parts.len() > 4 {
        return None;
    }
    let v: Vec<u8> = parts.iter().map(|p| p.parse::<u8>().ok()).collect::<Option<_>>()?;
    Some((v[0], v[1], v[2], v.get(3).copied()))
}

fn parse_hex(s:&str) -> Option<((f32,f32,f32), Option<f32>)> {
    let t = s.trim().trim_start_matches('#');
    if !t.is_ascii() {
        return None; }
    // #RGB / #RGBA shorthand: double every digit
    let short: String;
    let t = if t.len() == 3 || t.len() == 4 {
        short = t.chars().flat_map(|c| [c, c]).collect();
        short.as_str()
    } else { t };
    if t.len() != 6 && t.len() != 8 {
        return None; }
    let hx = |h: &str| u8::from_str_radix(h, 16).ok();
    let r =hx(&t[0..2])?;
    let g =hx(&t[2..4])?;
    let b = hx(&t[4..6])?;
    let a = if t.len() == 8 { Some(hx(&t[6..8])? as f32 / 255.0) } else { None };
    Some(((r as f32 / 255.0, g as f32 /255.0, b as f32 /255.0), a)) }

fn parse_mode(mode: ColorMode, s: &str) -> Option<((f32,f32,f32), Option<f32>)> {
    match mode {
        ColorMode::Rgba => {
            let n = num_tokens(s);
            if n.len()< 3 {
                return None; }
            Some((
                ((n[0].clamp(0.0,255.0)) /255.0,
                    (n[1].clamp(0.0,255.0)) /255.0,
                    (n[2].clamp(0.0, 255.0)) /255.0,),
                n.get(3).map(|a| (a.clamp(0.0, 255.0)) / 255.0),)) }
        ColorMode::Hex => parse_hex(s),
        ColorMode::Hsl | ColorMode::Hsla | ColorMode::Hsb => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            let (a, b) = (n[1].clamp(0.0, 100.0) / 100.0, n[2].clamp(0.0, 100.0) / 100.0);
            let rgb = if mode == ColorMode::Hsb {
                hsv_to_rgb_f(n[0], a, b)
            } else {
                hsl_to_rgb(n[0], a, b)
            };
            Some((rgb, n.get(3).map(|a| a.clamp(0.0, 1.0))))
        }
        ColorMode::Cmyk => {
            let n = num_tokens(s);
            if n.len() < 4 {
                return None;
            }
            let p = |v: f32| v.clamp(0.0, 100.0) / 100.0;
            Some((cmyk_to_rgb(p(n[0]), p(n[1]), p(n[2]), p(n[3])), None))
        }
        ColorMode::Hwb => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            Some((
                hwb_to_rgb(n[0], n[1].clamp(0.0, 100.0) / 100.0, n[2].clamp(0.0, 100.0) / 100.0),
                None,
            ))
        }
        ColorMode::Oklch => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            let mut l = n[0];
            if l > 1.5 {
                l /= 100.0;
            }
            Some((oklch_to_rgb(l.clamp(0.0, 1.0) as f64, n[1].max(0.0) as f64, n[2] as f64), None))
        }
        ColorMode::Lab => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            Some((lab_to_rgb(n[0].clamp(0.0, 100.0) as f64,
                             n[1].clamp(-128.0, 127.0) as f64,
                             n[2].clamp(-128.0, 127.0) as f64), None))
        }
        ColorMode::Gray => {
            let n = num_tokens(s);
            if n.is_empty() {
                return None;
            }
            let g = n[0].clamp(0.0, 255.0) / 255.0;
            Some(((g, g, g), None))
        }
        ColorMode::Yuv => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            Some((yuv_to_rgb(n[0].clamp(0.0, 1.0), n[1], n[2]),
                  n.get(3).map(|a| a.clamp(0.0, 1.0))))
        }
        ColorMode::Ypbpr => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            Some((ypbpr_to_rgb(n[0].clamp(0.0, 1.0), n[1], n[2]),
                  n.get(3).map(|a| a.clamp(0.0, 1.0))))
        }
        ColorMode::Xyz => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            Some((xyz_to_rgb_f(n[0].max(0.0) as f64,
                               n[1].clamp(0.0, 1.0) as f64,
                               n[2].max(0.0) as f64),
                  n.get(3).map(|a| a.clamp(0.0, 1.0))))
        }
        ColorMode::Xyy => {
            let n = num_tokens(s);
            if n.len() < 3 || n[1] <= 0.0 {
                return None;
            }
            let (x, y, z) = xyy_to_xyz(n[0].max(0.0) as f64,
                                       n[1] as f64,
                                       n[2].clamp(0.0, 1.0) as f64);
            Some((xyz_to_rgb_f(x, y, z), n.get(3).map(|a| a.clamp(0.0, 1.0))))
        }
        ColorMode::Munsell => {
            let n = num_tokens(s);
            if n.len() < 3 {
                return None;
            }
            Some((munsell_to_rgb(n[0].clamp(0.0, 100.0),
                                 n[1].clamp(0.0, 10.0),
                                 n[2].clamp(0.0, MUNSELL_C_MAX)), None))
        }
    }
}


fn mode_values(mode: ColorMode, p: &Picker) -> String {
    let (r, g, b) = p.rgb_f32();
    let alpha = p.alpha;
    match mode {
        ColorMode::Rgba => format!("{}, {}, {}, {}", q8(r), q8(g), q8(b), q8(alpha)),
        ColorMode::Hex => format!("#{:02X}{:02X}{:02X}", q8(r), q8(g), q8(b)),
        ColorMode::Hsl => {
            let (h, s, l) = p.hsl;
            format!("{:.2}, {:.2}%, {:.2}%", h, s * 100.0, l * 100.0)
        }
        ColorMode::Hsla => {
            let (h, s, l) = p.hsl;
            format!("{:.2}, {:.2}%, {:.2}%, {:.2}", h, s * 100.0, l * 100.0, alpha)
        }
        ColorMode::Hsb => {
            let (h, s, v) = (p.hue, p.sat, p.val);
            format!("{:.2}, {:.2}%, {:.2}%", h, s * 100.0, v * 100.0)
        }
        ColorMode::Cmyk => {
            let (c, m, y, k) = p.cmyk;
            format!("{:.2}%, {:.2}%, {:.2}%, {:.2}%",
                    c * 100.0, m * 100.0, y * 100.0, k * 100.0)
        }
        ColorMode::Hwb => {
            let (h, w, bl) = p.hwb;
            format!("{:.2} {:.2}% {:.2}%", h, w * 100.0, bl * 100.0)
        }
        ColorMode::Oklch => {
            let (l, c, h) = (p.oklch.0 as f64, p.oklch.1 as f64, p.oklch.2 as f64);
            format!("{:.3} {:.3} {:.2}", l, c, h)
        }
        ColorMode::Lab => {
            let (l, a, b) = (p.lab.0 as f64, p.lab.1 as f64, p.lab.2 as f64);
            format!("{:.2}, {:.2}, {:.2}", l, a, b)
        }
        ColorMode::Gray => {
            format!("{}", luma8(r, g, b))
        }
        ColorMode::Yuv => {
            let (y, u, v) = p.yuv;
            format!("{:.2}, {:.2}, {:.2}", y, u, v)
        }
        ColorMode::Ypbpr => {
            let (y, pb, pr) = p.ypbpr;
            format!("{:.2}, {:.2}, {:.2}", y, pb, pr)
        }
        ColorMode::Xyz => {
            let (x, y, z) = p.xyz;
            format!("{:.2}, {:.2}, {:.2}", x, y, z)
        }
        ColorMode::Xyy => {
            let (cx, cy, big_y) = p.xyy;
            format!("{:.2}, {:.2}, {:.2}", cx, cy, big_y)
        }
        ColorMode::Munsell => {
            let (h, v, c) = p.munsell;
            format!("{:.2}, {:.2}, {:.2}", h, v, c)
        }
    }
}

fn format_mode(mode: ColorMode, p: &Picker) -> String {
    if mode == ColorMode::Hex {
        return mode_values(mode, p);
    }
    format!("{}({})", mode_name(mode), mode_values(mode, p))
}

// ---------- mode-aware channel sliders ----------

const OK_CHROMA_MAX: f32 = 0.37;

struct ChanUi {
    label: &'static str,
    t: f32,
    value: String,
    bytes: Vec<u8>,
    handle: Color,
}

/// Color from a gradient triple (the thumb shows the gradient's own color
/// at the thumb position — "the color of the selected part of the slider").
fn color3(c: [u8; 3]) -> Color {
    Color::from_rgba(c[0], c[1], c[2], 255)
}

/// Placeholder slot for modes that use fewer than 4 sliders (e.g. GRAY).
fn empty_chan(w: usize) -> ChanUi {
    ChanUi {
        label: "",
        t: 0.0,
        value: String::new(),
        bytes: slider_grad_bytes(w, BG, BG),
        handle: Color::new(0.0, 0.0, 0.0, 0.0),
    }
}

fn grad_stops_bytes(w: usize, stops: &[(f32, [u8; 3])]) -> Vec<u8> {
    let mut out = Vec::with_capacity(w * 4);
    for x in 0..w {
        let t = x as f32 / w as f32;
        let mut i = 0;
        while i + 1 < stops.len() && t > stops[i + 1].0 {
            i += 1;
        }
        let a = &stops[i];
        let b = &stops[(i + 1).min(stops.len() - 1)];
        let f = if b.0 > a.0 {
            ((t - a.0) / (b.0 - a.0)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        out.push((a.1[0] as f32 + (b.1[0] as f32 - a.1[0] as f32) * f) as u8);
        out.push((a.1[1] as f32 + (b.1[1] as f32 - a.1[1] as f32) * f) as u8);
        out.push((a.1[2] as f32 + (b.1[2] as f32 - a.1[2] as f32) * f) as u8);
        out.push(255);
    }
    out
}

fn rgb_grad(w: usize, r0: u8, g0: u8, b0: u8, r1: u8, g1: u8, b1: u8) -> Vec<u8> {
    slider_grad_bytes(
        w,
        Color::from_rgba(r0, g0, b0, 255),
        Color::from_rgba(r1, g1, b1, 255),
    )
}

fn alpha_grad_144(r: u8, g: u8, b: u8) -> Vec<u8> {
    slider_grad_bytes(
        144,
        Color::from_rgba(r, g, b, 0),
        Color::from_rgba(r, g, b, 255),
    )
}

fn hue_ring(hue_to_rgb: impl Fn(f32) -> [u8; 3]) -> Vec<(f32, [u8; 3])> {
    (0..=12)
        .map(|i| {
            let f = i as f32 / 12.0;
            (f, hue_to_rgb(f * 360.0))
        })
        .collect()
}

fn mode_sliders(mode: ColorMode, p: &Picker) -> [ChanUi; 4] {
    let gw: usize = 144;
    let (r, g, b) = p.rgb_f32();
    let alpha = p.alpha;
    let p_hue = p.hue;
    let (r8, g8, b8) = (q8(r), q8(g), q8(b));
    let pct = |v: f32| format!("{:.2}%", v * 100.0);
//----I hate this---
    match mode {
        ColorMode::Rgba| ColorMode::Hex => [
            ChanUi {label:"R", t: r, value: q8(r).to_string(),
                bytes: rgb_grad(gw, 0, g8, b8, 255, g8, b8),
                handle: Color::from_rgba(r8, g8, b8, 255) },
            ChanUi {label:"G", t: g,value: q8(g).to_string(),
                bytes: rgb_grad(gw, r8, 0, b8, r8, 255, b8),
                handle: Color::from_rgba(r8, g8, b8, 255) },
            ChanUi {label:"B", t: b,value: q8(b).to_string(),
                bytes: rgb_grad(gw, r8, g8, 0, r8, g8, 255),
                handle: Color::from_rgba(r8, g8, b8, 255) },
            ChanUi {label:"A", t: alpha, value: q8(alpha).to_string(),
                bytes: alpha_grad_144(r8, g8, b8),
                handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
        ],
        ColorMode::Hsl | ColorMode::Hsla => {
            // sticky H/S/L is the source of truth while in HSL mode
            let (h, s, l) = p.hsl;
            let sh = s.max(0.05);
            let lh = l.clamp(0.05, 0.95);
            let hs = |hh: f32| { let c = hsl_to_rgb(hh, sh, lh); [q8(c.0), q8(c.1), q8(c.2)] };
            let sl = |ss: f32, ll: f32| { let c = hsl_to_rgb(h, ss, ll); [q8(c.0), q8(c.1), q8(c.2)] };
            [
                ChanUi { label: "H", t: h / 360.0, value: format!("{:.2}deg", h),
                    bytes: grad_stops_bytes(gw, &hue_ring(hs)),
                    handle: color3(hs(h)) },
                ChanUi { label: "S", t: s, value: pct(s),
                    bytes: grad_stops_bytes(gw, &[(0.0, sl(0.0, l)), (0.5, sl(0.5, l)), (1.0, sl(1.0, l))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "L", t: l, value: pct(l),
                    bytes: grad_stops_bytes(gw, &[(0.0, [0, 0, 0]), (0.5, sl(s, 0.5)), (1.0, [255, 255, 255])]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Hsb => {
            // sticky hue/sat/val is the source of truth while in HSB mode
            let (h, s, v) = (p_hue, p.sat, p.val);
            let sh = s.max(0.05);
            let vh = v.max(0.05);
            let hv = |hh: f32| { let c = hsv_to_rgb_f(hh, sh, vh); [q8(c.0), q8(c.1), q8(c.2)] };
            let sv = |ss: f32, vv: f32| { let c = hsv_to_rgb_f(h, ss, vv); [q8(c.0), q8(c.1), q8(c.2)] };
            let ring = hue_ring(hv);
            let s0 = sv(0.0, v);
            let s1 = sv(1.0, v);
            [
                ChanUi { label: "H", t: h / 360.0, value: format!("{:.2}deg", h),
                    handle: color3(hv(h)),
                    bytes: grad_stops_bytes(gw, &ring) },
                ChanUi { label: "S", t: s, value: pct(s),
                    bytes: rgb_grad(gw, s0[0], s0[1], s0[2], s1[0], s1[1], s1[2]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "B", t: v, value: pct(v),
                    bytes: grad_stops_bytes(gw, &[(0.0, [0, 0, 0]), (1.0, hv(h))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Cmyk => {
            // sticky C/M/Y/K is the source of truth while in CMYK mode
            let (c, m, y, k) = p.cmyk;
            let cm = |cc: f32, mm: f32, yy: f32, kk: f32| {
                let p = cmyk_to_rgb(cc, mm, yy, kk);
                [q8(p.0), q8(p.1), q8(p.2)]
            };
            [
                ChanUi { label: "C", t: c, value: pct(c),
                    bytes: grad_stops_bytes(gw, &[(0.0, cm(0.0, m, y, k)), (1.0, cm(1.0, m, y, k))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "M", t: m, value: pct(m),
                    bytes: grad_stops_bytes(gw, &[(0.0, cm(c, 0.0, y, k)), (1.0, cm(c, 1.0, y, k))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "Y", t: y, value: pct(y),
                    bytes: grad_stops_bytes(gw, &[(0.0, cm(c, m, 0.0, k)), (1.0, cm(c, m, 1.0, k))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "K", t: k, value: pct(k),
                    bytes: grad_stops_bytes(gw, &[(0.0, cm(c, m, y, 0.0)), (1.0, cm(c, m, y, 1.0))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
            ]
        }
        ColorMode::Hwb => {
            // sticky H/W/B is the source of truth while in HWB mode
            let (h, w, bl) = p.hwb;
            let hw = |hh: f32| { let c = hwb_to_rgb(hh, 0.0, 0.0); [q8(c.0), q8(c.1), q8(c.2)] };
            let wb = |ww: f32, bb2: f32| { let c = hwb_to_rgb(h, ww, bb2); [q8(c.0), q8(c.1), q8(c.2)] };
            let ring = hue_ring(hw);
            [
                ChanUi { label: "H", t: h / 360.0, value: format!("{:.2}deg", h),
                    handle: color3(hw(h)),
                    bytes: grad_stops_bytes(gw, &ring) },
                ChanUi { label: "W", t: w, value: pct(w),
                    bytes: grad_stops_bytes(gw, &[(0.0, wb(0.0, bl)), (1.0, wb(1.0, bl))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "B", t: bl, value: pct(bl),
                    bytes: grad_stops_bytes(gw, &[(0.0, wb(w, 0.0)), (1.0, wb(w, 1.0))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Oklch => {
            // sticky L/C/h is the source of truth while in OKLCH mode
            let (l, c, h) = (p.oklch.0 as f64, p.oklch.1 as f64, p.oklch.2 as f64);
            let ch = (c as f32).max(0.05);
            let ol = |ll: f64| { let p = oklch_to_rgb(ll, c, h); [q8(p.0), q8(p.1), q8(p.2)] };
            let oc = |cc: f64| { let p = oklch_to_rgb(l, cc, h); [q8(p.0), q8(p.1), q8(p.2)] };
            let oh = |hh: f64| { let p = oklch_to_rgb(l, ch as f64, hh); [q8(p.0), q8(p.1), q8(p.2)] };
            let lstops: Vec<(f32, [u8; 3])> =
                [0.0, 0.25, 0.5, 0.75, 1.0]
                    .iter()
                    .map(|f| (*f as f32, ol(*f)))
                    .collect();
            let hring: Vec<(f32, [u8; 3])> =
                (0..=12).map(|i| { let f = i as f32 / 12.0; (f, oh((f * 360.0) as f64)) }).collect();
            [
                ChanUi { label: "L", t: l as f32, value: format!("{:.2}", l),
                    handle: Color::from_rgba(r8, g8, b8, 255),
                    bytes: grad_stops_bytes(gw, &lstops) },
                ChanUi { label: "C", t: (c as f32) / OK_CHROMA_MAX, value: format!("{:.3}", c),
                    bytes: grad_stops_bytes(gw, &[(0.0, oc(0.0)), (1.0, oc(OK_CHROMA_MAX as f64))]),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "H", t: (h as f32) / 360.0, value: format!("{:.2}deg", h),
                    handle: color3(oh(h)),
                    bytes: grad_stops_bytes(gw, &hring) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Lab => {
            // sticky L/a/b is the source of truth while in LAB mode
            let (l, a, b) = (p.lab.0 as f64, p.lab.1 as f64, p.lab.2 as f64);
            let lf = |ll: f64| { let p = lab_to_rgb(ll, a, b); [q8(p.0), q8(p.1), q8(p.2)] };
            let af = |aa: f64| { let p = lab_to_rgb(l, aa, b); [q8(p.0), q8(p.1), q8(p.2)] };
            let bf = |bb: f64| { let p = lab_to_rgb(l, a, bb); [q8(p.0), q8(p.1), q8(p.2)] };
            let lstops: Vec<(f32, [u8; 3])> =
                [0.0, 25.0, 50.0, 75.0, 100.0].iter().map(|v| (*v as f32 / 100.0, lf(*v))).collect();
            // slider position of an a/b value in -128..127
            let pos = |val: f64| ((val + 128.0) / 255.0) as f32;
            let astops: Vec<(f32, [u8; 3])> =
                [-128.0, 0.0, 127.0].iter().map(|v| (pos(*v), af(*v))).collect();
            let bstops: Vec<(f32, [u8; 3])> =
                [-128.0, 0.0, 127.0].iter().map(|v| (pos(*v), bf(*v))).collect();
            [
                ChanUi { label: "L", t: (l as f32) / 100.0, value: format!("{:.2}", l),
                    handle: Color::from_rgba(r8, g8, b8, 255),
                    bytes: grad_stops_bytes(gw, &lstops) },
                ChanUi { label: "a", t: ((a as f32) + 128.0) / 255.0, value: format!("{:.2}", a),
                    handle: Color::from_rgba(r8, g8, b8, 255),
                    bytes: grad_stops_bytes(gw, &astops) },
                ChanUi { label: "b", t: ((b as f32) + 128.0) / 255.0, value: format!("{:.2}", b),
                    handle: Color::from_rgba(r8, g8, b8, 255),
                    bytes: grad_stops_bytes(gw, &bstops) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Gray => {
            let gv = luma8(r, g, b);
            [
                ChanUi { label: "G", t: gv as f32 / 255.0, value: gv.to_string(),
                    bytes: rgb_grad(gw, 0, 0, 0, 255, 255, 255),
                    handle: Color::from_rgba(gv, gv, gv, 255) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
                empty_chan(gw),
                empty_chan(gw),
            ]
        }
        ColorMode::Yuv => {
            // sticky Y/U/V is the source of truth while in YUV mode
            let (y,u,v) =p.yuv;
            let ut =(u + 0.436) /0.872;
            let vt =(v + 0.615) /1.23;
            let u_at =|t: f32| { let p2 = yuv_to_rgb(0.5, t * 0.872 - 0.436, 0.0); [q8(p2.0.clamp(0.0, 1.0)), q8(p2.1.clamp(0.0, 1.0)), q8(p2.2.clamp(0.0, 1.0))] };
            let v_at =|t: f32| { let p2 = yuv_to_rgb(0.5, 0.0, t * 1.23 - 0.615); [q8(p2.0.clamp(0.0, 1.0)), q8(p2.1.clamp(0.0, 1.0)), q8(p2.2.clamp(0.0, 1.0))] };
            let u_stops =[(0.0, u_at(0.0)), (1.0, u_at(1.0))];
            let v_stops =[(0.0, v_at(0.0)), (1.0, v_at(1.0))];
            [
                ChanUi { label:"Y", t: y, value: format!("{:.2}", y),
                    bytes: rgb_grad(gw, 0, 0, 0, 255, 255, 255),
                    handle: color3([q8(y); 3]) },
                ChanUi { label: "U", t: ut, value: format!("{:.2}", u),
                    bytes: grad_stops_bytes(gw, &u_stops),
                    handle: color3(u_at(ut)) },
                ChanUi { label: "V", t: vt, value: format!("{:.2}", v),
                    bytes: grad_stops_bytes(gw, &v_stops),
                    handle: color3(v_at(vt)) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Ypbpr => {
            // sticky Y/Pb/Pr is the source of truth while in YPbPr mode
            let (y, pb, pr) = p.ypbpr;
            let pbt = pb + 0.5;
            let prt = pr + 0.5;
            let pb_at = |t: f32| { let p2 = ypbpr_to_rgb(0.5, t - 0.5, 0.0); [q8(p2.0.clamp(0.0, 1.0)), q8(p2.1.clamp(0.0, 1.0)), q8(p2.2.clamp(0.0, 1.0))] };
            let pr_at = |t: f32| { let p2 = ypbpr_to_rgb(0.5, 0.0, t - 0.5); [q8(p2.0.clamp(0.0, 1.0)), q8(p2.1.clamp(0.0, 1.0)), q8(p2.2.clamp(0.0, 1.0))] };
            let pb_stops = [(0.0, pb_at(0.0)), (1.0, pb_at(1.0))];
            let pr_stops = [(0.0, pr_at(0.0)), (1.0, pr_at(1.0))];
            [
                ChanUi { label: "Y", t: y, value: format!("{:.2}", y),
                    bytes: rgb_grad(gw, 0, 0, 0, 255, 255, 255),
                    handle: color3([q8(y); 3]) },
                ChanUi { label: "Pb", t: pbt, value: format!("{:.2}", pb),
                    bytes: grad_stops_bytes(gw, &pb_stops),
                    handle: color3(pb_at(pbt)) },
                ChanUi { label: "Pr", t: prt, value: format!("{:.2}", pr),
                    bytes: grad_stops_bytes(gw, &pr_stops),
                    handle: color3(pr_at(prt)) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Xyz => {
            // sticky X/Y/Z is the source of truth while in XYZ mode
            let (x, y, z) = p.xyz;
            let x_at = |t: f32| { let p2 = xyz_to_rgb_f(t as f64 * 0.9505, 0.3, 0.3); [q8(p2.0), q8(p2.1), q8(p2.2)] };
            let y_at = |t: f32| { let p2 = xyz_to_rgb_f(0.3, t as f64, 0.3); [q8(p2.0), q8(p2.1), q8(p2.2)] };
            let z_at = |t: f32| { let p2 = xyz_to_rgb_f(0.3, 0.3, t as f64 * 1.08883); [q8(p2.0), q8(p2.1), q8(p2.2)] };
            let x_stops = [(0.0, x_at(0.0)), (1.0, x_at(1.0))];
            let y_stops = [(0.0, y_at(0.0)), (1.0, y_at(1.0))];
            let z_stops = [(0.0, z_at(0.0)), (1.0, z_at(1.0))];
            [
                ChanUi { label: "X", t: (x / 0.9505) as f32, value: format!("{:.2}", x),
                    bytes: grad_stops_bytes(gw, &x_stops),
                    handle: color3(x_at((x / 0.9505) as f32)) },
                ChanUi { label: "Y", t: y as f32, value: format!("{:.2}", y),
                    bytes: grad_stops_bytes(gw, &y_stops),
                    handle: color3(y_at(y as f32)) },
                ChanUi { label: "Z", t: (z / 1.08883) as f32, value: format!("{:.2}", z),
                    bytes: grad_stops_bytes(gw, &z_stops),
                    handle: color3(z_at((z / 1.08883) as f32)) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Xyy => {
            // sticky x/y/Y is the source of truth while in xyY mode
            let (cx, cy, big_y) = p.xyy;
            let x_at = |t: f32| { let (x, y, z) = xyy_to_xyz(t as f64, 0.33, 0.5); let p2 = xyz_to_rgb_f(x, y, z); [q8(p2.0), q8(p2.1), q8(p2.2)] };
            let y_at = |t: f32| { let (x, y, z) = xyy_to_xyz(0.31, (t as f64).max(1e-4), 0.5); let p2 = xyz_to_rgb_f(x, y, z); [q8(p2.0), q8(p2.1), q8(p2.2)] };
            let x_stops = [(0.0, x_at(0.0)), (0.5, x_at(0.5)), (1.0, x_at(1.0))];
            let y_stops = [(0.0, y_at(0.0)), (0.5, y_at(0.5)), (1.0, y_at(1.0))];
            [
                ChanUi { label: "x", t: cx as f32, value: format!("{:.2}", cx),
                    bytes: grad_stops_bytes(gw, &x_stops),
                    handle: color3(x_at(cx as f32)) },
                ChanUi { label: "y", t: cy as f32, value: format!("{:.2}", cy),
                    bytes: grad_stops_bytes(gw, &y_stops),
                    handle: color3(y_at(cy as f32)) },
                ChanUi { label: "Y", t: big_y as f32, value: format!("{:.2}", big_y),
                    bytes: rgb_grad(gw, 0, 0, 0, 255, 255, 255),
                    handle: color3([q8(big_y as f32); 3]) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
            ]
        }
        ColorMode::Munsell => {
            // the hue wheel owns H; the sliders are value and chroma
            let (mh, mv, mc) = p.munsell;
            let v_at = |t: f32| { let c2 = munsell_to_rgb(mh, t * 10.0, mc); [q8(c2.0), q8(c2.1), q8(c2.2)] };
            let c_at = |t: f32| { let c2 = munsell_to_rgb(mh, mv, t * MUNSELL_C_MAX); [q8(c2.0), q8(c2.1), q8(c2.2)] };
            let v_stops = [(0.0, v_at(0.0)), (1.0, v_at(1.0))];
            let c_stops = [(0.0, c_at(0.0)), (1.0, c_at(1.0))];
            [
                ChanUi { label: "V", t: mv / 10.0, value: format!("{:.2}", mv),
                    bytes: grad_stops_bytes(gw, &v_stops),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "C", t: mc / MUNSELL_C_MAX, value: format!("{:.2}", mc),
                    bytes: grad_stops_bytes(gw, &c_stops),
                    handle: Color::from_rgba(r8, g8, b8, 255) },
                ChanUi { label: "A", t: alpha, value: q8(alpha).to_string(),
                    bytes: alpha_grad_144(r8, g8, b8),
                    handle: Color::from_rgba(r8, g8, b8, q8(alpha)) },
                empty_chan(gw),
            ]
        }
    }
}

/// Apply a dragged channel (index 0..3) value t (0..1) in the given mode.
fn set_channel(mode: ColorMode, i: usize, t: f32, p: &mut Picker) {
    let (r, g, b) = p.rgb_f32();
    match mode {
        ColorMode::Rgba | ColorMode::Hex => match i {
            0 => p.sync_rgb_f(t, g, b),
            1 => p.sync_rgb_f(r, t, b),
            2 => p.sync_rgb_f(r, g, t),
            _ => p.alpha = t,
        },
        ColorMode::Hsl | ColorMode::Hsla => {
            let (h, s, l) = p.hsl;
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (nh, ns, nl) = match i {
                0 => (hue_from_t(t), s, l),
                1 => (h, t, l),
                _ => (h, s, t),
            };
            let rgb = hsl_to_rgb(nh, ns, nl);
            p.keep_hsl = true; // the channels are the truth; don't re-derive
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
            p.hsl = (nh, ns, nl);
        }
        ColorMode::Hsb => {
            if i == 3 {
                p.alpha = t;
                return;
            }
            match i {
                0 => p.hue = hue_from_t(t),
                1 => p.sat = t,
                _ => p.val = t,
            }
            // refresh the other stickies from the new color, but keep the
            // hsv values just set (S must survive dragging V to 0)
            let (r, g, b) = p.rgb_f32();
            p.keep_hsv = true;
            p.sync_rgb_f(r, g, b);
        }
        ColorMode::Cmyk => {
            let (c, m, y, k) = p.cmyk;
            let (nc, nm, ny, nk) = (
                if i == 0 { t } else { c },
                if i == 1 { t } else { m },
                if i == 2 { t } else { y },
                if i == 3 { t } else { k },
            );
            let rgb = cmyk_to_rgb(nc, nm, ny, nk);
            if i <= 1 {
                // C/M are the spectrum-plane axes: move the dot too
                p.plane_pos = if i == 0 { (t, p.plane_pos.1) } else { (p.plane_pos.0, t) };
                p.keep_plane = true;
            }
            p.keep_cmyk = true; // CMYK is redundant in RGB: don't re-derive
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
            p.cmyk = (nc, nm, ny, nk);
        }
        ColorMode::Hwb => {
            let (h, w, bl) = p.hwb;
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (nh, nw, nbl) = match i {
                0 => (hue_from_t(t), w, bl),
                1 => (h, t, bl),
                _ => (h, w, t),
            };
            let rgb = hwb_to_rgb(nh, nw, nbl);
            if i >= 1 {
                // W/B are the spectrum-plane axes: move the dot too
                // (plane_pos is bottom-origin; blackness grows downward)
                p.plane_pos = if i == 1 { (t, p.plane_pos.1) } else { (p.plane_pos.0, 1.0 - t) };
                p.keep_plane = true;
            }
            p.keep_hwb = true;
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
            p.hwb = (nh, nw, nbl);
        }
        ColorMode::Oklch => {
            let (l, c, h) = (p.oklch.0 as f64, p.oklch.1 as f64, p.oklch.2 as f64);
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (nl, nc, nh) = match i {
                0 => (t as f64, c, h),
                1 => (l, (t * OK_CHROMA_MAX) as f64, h),
                _ => (l, c, (t * 360.0) as f64),
            };
            let rgb = oklch_to_rgb(nl, nc, nh);
            if i == 1 {
                p.plane_pos.0 = t;
                p.keep_plane = true;
            } else if i == 2 {
                p.plane_pos.1 = t;
                p.keep_plane = true;
            }
            p.keep_oklch = true; // don't recompute from clamped RGB
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
            p.oklch = (nl as f32, nc as f32, nh as f32);
        }
        ColorMode::Lab => {
            let (l, a, b) = (p.lab.0 as f64, p.lab.1 as f64, p.lab.2 as f64);
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (nl, na, nb) = match i {
                0 => ((t * 100.0) as f64, a, b),
                1 => (l, (t * 255.0 - 128.0) as f64, b),
                _ => (l, a, (t * 255.0 - 128.0) as f64),
            };
            let rgb = lab_to_rgb(nl, na, nb);
            p.keep_lab = true;
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
            p.lab = (nl as f32, na as f32, nb as f32);
        }
        ColorMode::Gray => match i {
            0 => p.sync_rgb_f(t, t, t),
            1 => p.alpha = t,
            _ => {}
        },
        ColorMode::Yuv => {
            if i == 3 {
                p.alpha = t;
                return;
            }

            let (ny, nu, nv) = match i {
                0 => (t, p.yuv.1, p.yuv.2),
                1 => (p.yuv.0, t * 0.872 - 0.436, p.yuv.2),
                _ => (p.yuv.0, p.yuv.1, t * 1.23 - 0.615),
            };
            p.yuv = (ny, nu, nv);
            p.keep_yuv = true;
            if i == 1 {
                p.plane_pos.0 = t;
                p.keep_plane = true;
            } else if i == 2 {
                p.plane_pos.1 = t;
                p.keep_plane = true;
            }
            let rgb = yuv_to_rgb(ny, nu, nv);
            p.sync_rgb_f(rgb.0.clamp(0.0, 1.0), rgb.1.clamp(0.0, 1.0),
                         rgb.2.clamp(0.0, 1.0));
        }
        ColorMode::Ypbpr => {
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (ny, npb, npr) = match i {
                0 => (t, p.ypbpr.1, p.ypbpr.2),
                1 => (p.ypbpr.0, t - 0.5, p.ypbpr.2),
                _ => (p.ypbpr.0, p.ypbpr.1, t - 0.5),
            };
            p.ypbpr = (ny, npb, npr);
            p.keep_ypbpr = true;
            if i == 1 {
                p.plane_pos.0 = t;
                p.keep_plane = true;
            } else if i == 2 {
                p.plane_pos.1 = t;
                p.keep_plane = true;
            }
            let rgb = ypbpr_to_rgb(ny, npb, npr);
            p.sync_rgb_f(rgb.0.clamp(0.0, 1.0), rgb.1.clamp(0.0, 1.0),
                         rgb.2.clamp(0.0, 1.0));
        }
        ColorMode::Xyz => {
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (nx, ny, nz) = match i {
                0 => (t as f64 * 0.9505, p.xyz.1, p.xyz.2),
                1 => (p.xyz.0, t as f64, p.xyz.2),
                _ => (p.xyz.0, p.xyz.1, t as f64 * 1.08883),
            };
            p.xyz = (nx, ny, nz);
            p.keep_xyz = true;
            let rgb = xyz_to_rgb_f(nx, ny, nz);
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
        }
        ColorMode::Xyy => {
            if i == 3 {
                p.alpha = t;
                return;
            }
            let (nx,ny,nlum) = match i {
                0 =>(t as f64,p.xyy.1.max(1e-4), p.xyy.2),
                1 =>(p.xyy.0.max(1e-4), (t as f64).max(1e-4), p.xyy.2),
                _ =>(p.xyy.0.max(1e-4), p.xyy.1.max(1e-4), t as f64),
            };
            p.xyy = (nx,ny,nlum);
            p.keep_xyy = true;
            let (x,y,z) =xyy_to_xyz(nx,ny,nlum);
            let rgb =xyz_to_rgb_f(x, y, z);
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
        }
        ColorMode::Munsell=> {
            if i == 2 {
                p.alpha = t;
                return;
            }
            let (mh,mv,mc) = p.munsell;
            let (nv,nc) = match i {
                0 => (t *10.0, mc),
                _ =>(mv, t * MUNSELL_C_MAX),};
            let rgb =munsell_to_rgb(mh,nv,nc);
            p.keep_munsell =true;
            p.sync_rgb_f(rgb.0,rgb.1, rgb.2);
            p.munsell =(mh,nv,nc); } } }
// yo my own code looks so ugly some how
// ---------- persistence (palette + hints i guess) ---------

fn exe_dir()-> std::path::PathBuf{
    std::env::current_exe()
        .ok()
        .and_then(|p|p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(||std::path::PathBuf::from(".")) }

/// Where palette.txt / state.txt live: next to the exe, except on macOS, where
/// an .app bundle may be read-only, so they go in Application Support.
fn data_dir() -> std::path::PathBuf {
    #[cfg(target_os = "macos")]
    if let Some(home) = std::env::var_os("HOME") {
        let dir = std::path::PathBuf::from(home).join("Library/Application Support/Project Color Picker");
        if std::fs::create_dir_all(&dir).is_ok() {
            return dir;
        }
    }
    exe_dir()
}

fn swatch_path() -> std::path::PathBuf {
    data_dir().join("palette.txt") }

/// How many colors can be saved, and how many of them show at once.
const SWATCH_SLOTS: usize = 100;
const SWATCH_SHOWN: usize = 8;

fn load_swatches() ->[Option<(u8, u8, u8, u8)>; SWATCH_SLOTS] {
    let mut out = [None; SWATCH_SLOTS];
    let Ok(text) = std::fs::read_to_string(swatch_path()) else {
        return out;
    };
    for (i, line) in text.lines().take(SWATCH_SLOTS).enumerate() {
        if let Some((r, g, b, a)) = parse_rgba(line) {
            out[i] = Some((r, g, b, a.unwrap_or(255)));
        }
    }
    out
}

fn save_swatches(sw: &[Option<(u8, u8, u8, u8)>; SWATCH_SLOTS]) {
    let mut s = String::new();
    for slot in sw {
        match slot {
            Some((r, g, b, a)) => s.push_str(&format!("{}, {}, {}, {}\n", r, g, b, a)),
            None => s.push_str("-\n"),
        }
    }
    let _ = std::fs::write(swatch_path(), s);
}

fn state_path() -> std::path::PathBuf {
    data_dir().join("state.txt")
}

/// What the app reopens with: mode, color and the always-on-top pin.
#[derive(Clone, Copy, PartialEq)]
struct Saved {
    mode: ColorMode,
    rgba: (u8, u8, u8, u8),
    on_top: bool,
    dim: bool,
}

fn load_state() -> Option<Saved> {
    let text = std::fs::read_to_string(state_path()).ok()?;
    let mut lines = text.lines();
    let name = lines.next()?.trim();
    let mode = MODES.iter().copied().find(|m| mode_name(*m) == name)?;
    let (r, g, b, a) = parse_rgba(lines.next()?)?;
    let on_top = lines.next().is_some_and(|l| l.trim() == "1");
    let dim = lines.next().is_some_and(|l| l.trim() == "1");
    Some(Saved { mode, rgba: (r, g, b, a.unwrap_or(255)), on_top, dim })
}

fn save_state(s: &Saved) {
    let (r, g, b, a) = s.rgba;
    let text = format!(
        "{}\n{}, {}, {}, {}\n{}\n{}\n",
        mode_name(s.mode), r, g, b, a, s.on_top as u8, s.dim as u8,
    );
    let _ = std::fs::write(state_path(), text);
}

// ---------- gradient strips --------

fn slider_grad_bytes(w: usize, left: Color, right: Color) -> Vec<u8> {
    let mut b = Vec::with_capacity(w * 4);
    for x in 0..w {
        let c = lerp_color(left, right, x as f32 / w as f32);
        b.extend_from_slice(&[
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
            (c.a * 255.0) as u8,
        ]);
    }
    b
}

// ---------- static textures ----------
// 1px-wide strips; the shape shader stretches them and rounds them into pills

fn strip_texture(rows: impl Iterator<Item = [u8; 4]>) -> Texture2D {
    let bytes: Vec<u8> = rows.flatten().collect();
    let t = Texture2D::from_rgba8(1, (bytes.len() / 4) as u16, &bytes);
    t.set_filter(FilterMode::Linear);
    t
}

fn hue_strip_texture() -> Texture2D {
    strip_texture((0..256).map(|y| {
        let (r, g, b) = hsv_to_rgb((1.0 - y as f32 / 255.0) * 360.0, 0.8, 1.0);
        [r, g, b, 255]
    }))
}

/// white at the top, black at the bottom
fn gray_strip_texture() -> Texture2D {
    strip_texture((0..=255u8).map(|y| [255 - y, 255 - y, 255 - y, 255]))
}

/// opaque at the top, transparent at the bottom; tinted with the picked color
fn fade_strip_texture() -> Texture2D {
    strip_texture((0..=255u8).map(|y| [255, 255, 255, 255 - y]))
}

fn sat_overlay_texture(w: u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(w as usize * 4);
    for x in 0..w {
        let a = (255.0 * (1.0 - x as f32 / w as f32)) as u8;
        bytes.extend_from_slice(&[255, 255, 255, a]);
    }
    Texture2D::from_rgba8(w, 1, &bytes)
}

fn val_overlay_texture(h: u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(h as usize * 4);
    for y in 0..h {
        let a = (255.0 * (y as f32 / h as f32)) as u8;
        bytes.extend_from_slice(&[0, 0, 0, a]);
    }
    Texture2D::from_rgba8(1, h, &bytes)
}

fn hsl_square_bytes(hue: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        let l = 1.0 - y as f32 /h as f32;
        for x in 0..w {
            let s = x as f32 / w as f32;
            let (r, g, bl) = hsl_to_rgb(hue, s, l);
            b.extend_from_slice(&[q8(r), q8(g), q8(bl), 255]);
        }
    }
    b
}


fn gray_square_bytes() -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for _y in 0..h {
        for x in 0..w {
            let v = q8(x as f32 / w as f32);
            b.extend_from_slice(&[v, v, v, 255]);
        }
    }
    b
}

// ----- per-mode spectrum planes: each block's axes are the mode's own
// channels, so the square always references the active color mode -----

/// CMYK: x = C, y = M (top = 100%), at the current Y and K.
fn cmyk_square_bytes(y: f32, k: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        let m = 1.0 - row as f32 / h as f32;
        for col in 0..w {
            let c = col as f32 / w as f32;
            let p = cmyk_to_rgb(c, m, y.clamp(0.0, 1.0), k.clamp(0.0, 1.0));
            b.extend_from_slice(&[q8(p.0), q8(p.1), q8(p.2), 255]);
        }
    }
    b
}

/// HWB: x = whiteness y = blackness (top = 0)at the current hue?
fn hwb_square_bytes(hue: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        let bl = row as f32 / h as f32;
        for col in 0..w {
            let wh = col as f32 / w as f32;
            let p = hwb_to_rgb(hue, wh, bl);
            // past the diagonal (W + B > 100%) every point is just a gray: dim it
            b.extend_from_slice(&gamut_px(p.0, p.1, p.2, wh + bl <= 1.0));
        }
    }
    b
}


/// OKLCH: x = chroma (0..OK_CHROMA_MAX), y = hue (top = 360), at lightness l.
fn oklch_square_bytes(l: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let l = l.clamp(0.0, 1.0) as f64;
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        let hue = ((1.0 - row as f32 / h as f32) * 360.0) as f64;
        for col in 0..w {
            let c = (col as f32 / w as f32 * OK_CHROMA_MAX) as f64;
            let lin = oklch_to_linear(l, c, hue);
            let ok = lin.0.min(lin.1).min(lin.2) >= -0.002 && lin.0.max(lin.1).max(lin.2) <= 1.002;
            let p = oklch_to_rgb(l, c, hue);
            b.extend_from_slice(&gamut_px(p.0, p.1, p.2, ok));
        }
    }
    b
}

/// Whether planes dim the points RGB can't show (the title bar toggle; off by default).
static DIM_PLANES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn dim_planes() -> bool {
    DIM_PLANES.load(std::sync::atomic::Ordering::Relaxed)
}

/// A plane pixel for an (already clamped) color. Points outside what RGB can
/// show are dimmed toward the panel color, so the displayable region stands
/// out as a shape.
fn gamut_px(r: f32, g: f32, b: f32, displayable: bool) -> [u8; 4] {
    if displayable || !dim_planes() {
        [q8(r), q8(g), q8(b), 255]
    } else {
        let dim = |c: f32| q8(0.08 + (c - 0.08) * 0.22);
        [dim(r), dim(g), dim(b), 255]
    }
}

/// One pixel of a YUV / YPbPr plane, from the unclamped conversion result.
fn chroma_plane_px(p: (f32, f32, f32)) -> [u8; 4] {
    let lo = p.0.min(p.1).min(p.2);
    let hi = p.0.max(p.1).max(p.2);
    gamut_px(p.0.clamp(0.0, 1.0), p.1.clamp(0.0, 1.0), p.2.clamp(0.0, 1.0), lo >= -0.003 && hi <= 1.003)
}

/// YUV: x = U (-0.436..0.436), y = V (-0.615..0.615, top = +0.615), at Y.
fn yuv_square_bytes(y: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        let v = (1.0 - row as f32 / h as f32) * 1.23 - 0.615;
        for col in 0..w {
            let u = col as f32 / w as f32 * 0.872 - 0.436;
            let p = yuv_to_rgb(y.clamp(0.0, 1.0), u, v);
            b.extend_from_slice(&chroma_plane_px(p));
        }
    }
    b
}

/// YPbPr: x = Pb, y = Pr (both -0.5..0.5, top = +0.5)at the current y(im not good at math tbh)
fn ypbpr_square_bytes(y: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        let pr = (1.0 - row as f32 / h as f32) - 0.5;
        for col in 0..w {
            let pb = col as f32 / w as f32 - 0.5;
            let p = ypbpr_to_rgb(y.clamp(0.0, 1.0), pb, pr);
            b.extend_from_slice(&chroma_plane_px(p));
        }
    }
    b
}

/// Munsell hue ring radii, as fractions of the plane's half-width.
const MUNSELL_R_OUT: f32 = 0.845;
const MUNSELL_R_IN: f32 = 0.60;
const MUNSELL_FAMILIES: [&str; 10] = ["R", "YR", "Y", "GY", "G", "BG", "B", "PB", "P", "RP"];

/// Munsell hue ring: the angle around the center is the Munsell hue
/// H 0..100 (clockwise from 12 o'clock), colored at the current value V and
/// chroma C. Transparent inside and outside the ring.
fn munsell_wheel_bytes(v: f32, c: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let (cx, cy) = (130.0f32, 130.0f32);
    let (r_out, r_in) = (130.0 * MUNSELL_R_OUT, 130.0 * MUNSELL_R_IN);
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        for col in 0..w {
            let dx = col as f32 + 0.5 - cx;
            let dy = row as f32 + 0.5 - cy;
            let d = dx.hypot(dy);
            let a = (0.5 - (d - r_out)).clamp(0.0, 1.0) * (0.5 + (d - r_in)).clamp(0.0, 1.0); // ~1px AA edges
            if a <= 0.0 {
                b.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            let ang = dx.atan2(-dy).to_degrees().rem_euclid(360.0);
            let p = munsell_to_rgb(ang / 3.6, v, c);
            b.extend_from_slice(&[q8(p.0), q8(p.1), q8(p.2), (a * 255.0) as u8]);
        }
    }
    b
}

/// Munsell-style notation for the app's H/V/C, e.g. "2.5G 7.1/13".
fn munsell_notation(h: f32, v: f32, c: f32) -> (String, String) {
    let h = h.rem_euclid(100.0);
    let mut family = (h / 10.0) as usize % 10;
    let mut step = h % 10.0;
    if step < 0.05 {
        // hue 0 of a family is written as 10 of the one before it
        step = 10.0;
        family = (family + 9) % 10;
    }
    (format!("{:.1}{}", step, MUNSELL_FAMILIES[family]), format!("{:.1} / {:.1}", v, c))
}

#[cfg(target_os = "windows")]
mod screen {
    #![allow(non_snake_case)]

    use std::ffi::c_void;
    use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, Ordering};

    const PER_MONITOR_AWARE_V2:isize = -4;
    const SRCCOPY:u32 =0x00CC0020;
    const WH_MOUSE_LL:i32 =14;
    const WM_LBUTTONDOWN:usize =0x0201;
    const WM_LBUTTONUP:usize=0x0202;
    const SWP_NOSIZE:u32 =0x0001;
    const SWP_NOMOVE:u32 =0x0002;
    const SM_CXSCREEN:i32 =0;
    const SM_CYSCREEN: i32 =1;

    // ----- customtitle bar (yeppie)shit i hate this ide -------
    const WM_NCCALCSIZE:u32 =0x0083;
    const WM_NCHITTEST:u32 =0x0084;
    const GWLP_WNDPROC:isize =-4;
    const GWL_STYLE:isize =-16;
    const WS_CAPTION:u32 =0x00C0_0000;
    const SWP_NOZORDER:u32 =0x0004;
    const SWP_FRAMECHANGED:u32 =0x0020;
    const HTCLIENT:isize =1;
    const HTCAPTION:isize = 2;
    const HTLEFT:isize= 10;
    const HTRIGHT:isize =11;
    const HTTOP:isize =12;
    const HTTOPLEFT:isize =13;
    const HTTOPRIGHT:isize =14;
    const HTBOTTOM:isize =15;
    const HTBOTTOMLEFT:isize =16;
    const HTBOTTOMRIGHT:isize =17;
    const WM_CLOSE:u32 =0x0010;
    const SW_MINIMIZE:i32 =6;
    const SW_MAXIMIZE:i32 =3;
    const SW_RESTORE:i32 =9;

    pub const CAPTURE_N:i32 =31;
    /// the app draws its own title bar and window buttons
    pub const CUSTOM_FRAME: bool = true;
    /// Pick Screen works here
    pub const CAN_PICK: bool = true;
    /// picking is done by the OS's own sampler (see `sampler_result`)
    pub const SYSTEM_SAMPLER: bool = false;
    pub fn sampler_result() -> Option<Option<(u8, u8, u8)>> { None }
    const OCELL:i32 =8;
    const OL_N:i32 =CAPTURE_N;
    const OLOUPE:i32 =OL_N *OCELL;
    const OW:i32=OLOUPE +16;
    const OH:i32 =OLOUPE +70;
    const WS_POPUP:u32 =0x8000_0000;
    const WS_EX_LAYERED:u32 =0x0008_0000;
    const WS_EX_TOPMOST:u32 =0x0000_0008;
    const WS_EX_TOOLWINDOW:u32 =0x0000_0080;
    const WS_EX_NOACTIVATE:u32 =0x0800_0000;
    const SW_HIDE:i32 =0;
    const SW_SHOWNOACTIVATE:i32 =4;
    const DIB_RGB_COLORS:u32 =0;
    const ULW_ALPHA:u32 =2;

    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct Size {
        cx: i32,
        cy: i32,
    }

    #[repr(C)]
    struct BlendFunction {
        blend_op: u8,
        blend_flags: u8,
        source_constant_alpha: u8,
        alpha_format: u8,
    }

    #[repr(C)]
    struct WinRect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[repr(C)]
    struct MonInfoExW {
        cb_size: u32,
        rc_monitor: WinRect,
        rc_work: WinRect,
        flags: u32,
        device: [u16; 32],
    }

    #[repr(C)]
    struct DevModeW {
        device_name:[u16; 32],
        spec_version:u16,
        driver_version:u16,
        size:u16,
        driver_extra:u16,
        fields:u32,
        position:(i32,i32),
        orientation:u32,
        fixed_output: u32,
        color: i16,
        duplex: i16,
        y_resolution: i16,
        tt_option: i16,
        collate: i16,
        form_name: [u16; 32],
        log_pixels: u16,
        bits_per_pel: u32,
        pels_width: u32,
        pels_height: u32,
        display_flags: u32,
        display_frequency: u32,
        icm_method: u32,
        icm_intent: u32,
        media_type: u32,
        dither_type: u32,
        reserved1: u32,
        reserved2: u32,
        panning_width: u32,
        panning_height: u32,
    }

    #[repr(C)]
    struct MouseLlStruct {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct WndClassW {
        style: u32,
        lpfn_wnd_proc: Option<unsafe extern "system" fn(*mut c_void, u32, usize, isize) -> isize>,
        cb_cls_extra: i32,
        cb_wnd_extra: i32,
        h_instance: *mut c_void,
        h_icon: *mut c_void,
        h_cursor: *mut c_void,
        hbr_background: *mut c_void,
        lpsz_menu_name: *const u16,
        lpsz_class_name: *const u16,
    }

    #[repr(C)]
    #[derive(Default)]
    struct BitmapInfoHeader {
        bi_size: u32,
        bi_width: i32,
        bi_height: i32,
        bi_planes: u16,
        bi_bit_count: u16,
        bi_compression: u32,
        bi_size_image: u32,
        bi_x_pels_per_meter: i32,
        bi_y_pels_per_meter: i32,
        bi_clr_used: u32,
        bi_clr_important: u32,
    }
    #[repr(C)]
    struct BitmapInfo {
        bmi_header: BitmapInfoHeader,
        bmi_colors: [u32; 1],
    }
    #[link(name = "user32")]
    unsafe extern "system" {
        fn GetCursorPos(lp_point: *mut Point) -> i32;
        fn SetCursorPos(x: i32, y: i32) -> i32;
        fn GetModuleHandleW(lp_module_name: *const u16) -> *mut c_void;
        fn GetDC(hwnd: *mut c_void) -> *mut c_void;
        fn ReleaseDC(hwnd: *mut c_void, hdc: *mut c_void) -> i32;
        fn SetThreadDpiAwarenessContext(value: isize) -> isize;
        fn GetActiveWindow() -> *mut c_void;
        fn GetDpiForWindow(hwnd: *mut c_void) -> u32;
        fn FindWindowW(lp_class_name: *const u16, lp_window_name: *const u16) -> *mut c_void;
        fn SetWindowPos(
            hwnd: *mut c_void,
            after: *mut c_void,
            x: i32, y: i32, cx: i32, cy: i32,
            flags: u32,
        ) -> i32;
        fn GetSystemMetrics(index: i32) -> i32;
        fn RegisterClassW(lp_wnd_class: *const WndClassW) -> u16;
        fn CreateWindowExW(
            ex_style: u32,
            class_name: *const u16,
            window_name: *const u16,
            style: u32,
            x: i32, y: i32, w: i32, h: i32,
            parent: *mut c_void,
            menu: *mut c_void,
            instance: *mut c_void,
            param: *mut c_void,
        ) -> *mut c_void;
        fn DefWindowProcW(hwnd: *mut c_void, msg: u32, wparam: usize, lparam: isize) -> isize;
        fn ShowWindow(hwnd: *mut c_void, cmd: i32) -> i32;
        fn IsZoomed(hwnd: *mut c_void) -> i32;
        fn PostMessageW(hwnd: *mut c_void, msg: u32, wparam: usize, lparam: isize) -> i32;
        fn GetWindowRect(hwnd: *mut c_void, rect: *mut WinRect) -> i32;
        fn MonitorFromWindow(hwnd: *mut c_void, flags: u32) -> *mut c_void;
        fn GetMonitorInfoW(hmon: *mut c_void, info: *mut MonInfoExW) -> i32;
        fn EnumDisplaySettingsW(device: *const u16, mode: u32, dm: *mut DevModeW) -> i32;
        fn GetWindowLongPtrW(hwnd: *mut c_void, index: isize) -> isize;
        fn SetWindowLongPtrW(hwnd: *mut c_void, index: isize, value: isize) -> isize;
        fn CallWindowProcW(
            prev: *mut c_void,
            hwnd: *mut c_void,
            msg: u32,
            wparam: usize,
            lparam: isize,
        ) -> isize;
        fn UpdateLayeredWindow(
            hwnd: *mut c_void,
            hdc_dst: *mut c_void,
            ppt_dst: *mut Point,
            psize: *mut Size,
            hdc_src: *mut c_void,
            ppt_src: *mut Point,
            cr_key: u32,
            pblend: *mut BlendFunction,
            dw_flags: u32,
        ) -> i32;
        fn SetWindowsHookExW(
            id_hook: i32,
            lpfn: Option<unsafe extern "system" fn(i32, usize, isize) -> isize>,
            hmod: *mut c_void,
            thread_id: u32,
        ) -> *mut c_void;
        fn UnhookWindowsHookEx(hhk: *mut c_void) -> i32;
        fn CallNextHookEx(hhk: *mut c_void, ncode: i32, wparam: usize, lparam: isize) -> isize;
    }
    #[link(name = "gdi32")]
    unsafe extern "system" {
        fn CreateCompatibleDC(hdc: *mut c_void) -> *mut c_void;
        fn CreateCompatibleBitmap(hdc: *mut c_void, w: i32, h: i32) -> *mut c_void;
        fn CreateDIBSection(
            hdc: *mut c_void,
            bmi: *mut BitmapInfo,
            usage: u32,
            bits: *mut *mut c_void,
            h_section: *mut c_void,
            offset: u32,
        ) -> *mut c_void;
        fn SelectObject(hdc: *mut c_void, h: *mut c_void) -> *mut c_void;
        fn BitBlt(
            dst: *mut c_void, dx: i32, dy: i32, w: i32, h: i32,
            src: *mut c_void, sx: i32, sy: i32, rop: u32,
        ) -> i32;
        fn GetDIBits(
            hdc: *mut c_void, bmp: *mut c_void, start: u32, lines: u32,
            bits: *mut u8, bmi: *mut BitmapInfo, usage: u32,
        ) -> i32;
        fn DeleteObject(h: *mut c_void) -> i32;
        fn DeleteDC(hdc: *mut c_void) -> i32;
    }
    #[link(name = "winmm")]
    unsafe extern "system" {
        fn timeBeginPeriod(period: u32) -> u32;
    }

    #[link(name = "dwmapi")]
    unsafe extern "system" {
        fn DwmSetWindowAttribute(
            hwnd: *mut c_void,
            attr: u32,
            value: *mut c_void,
            size: u32,
        ) -> i32;
    }

    // ----- taskbar / window icon (WM_SETICON) -----
    const WM_SETICON: u32 = 0x0080;
    const ICON_SMALL: usize = 0;
    const ICON_BIG: usize = 1;

    #[link(name = "user32")]
    unsafe extern "system" {
        // plain CreateIcon: this API has no A/W variants (no string params),
        // so "CreateIconW" doesn't exist in user32.lib
        fn CreateIcon(
            instance: *mut c_void,
            width: i32,
            height: i32,
            planes: u8,
            bits_pixel: u8,
            and_bits: *const u8,
            xor_bits: *const u8,
        ) -> *mut c_void;
        fn SendMessageW(hwnd: *mut c_void, msg: u32, wparam: usize, lparam: isize) -> isize;
    }

    fn make_hicon(size: u32) -> *mut c_void {
        let png = include_bytes!("../assets/icon.png");
        let img = image::load_from_memory(png).expect("icon").to_rgba8();
        let img = image::imageops::resize(&img, size, size, image::imageops::FilterType::Lanczos3);
        let (w, h) = (img.width() as i32, img.height() as i32);

        let mut color: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);
        for px in img.pixels() {
            color.extend_from_slice(&[px[2], px[1], px[0], px[3]]); // BGRA
        }
        let stride = ((w as usize + 15) / 16) * 2;
        let mask = vec![0u8; stride * h as usize]; // 0 = opaque, alpha decides

        unsafe {
            CreateIcon(std::ptr::null_mut(), w, h, 1, 32, mask.as_ptr(), color.as_ptr())
        }
    }

    /// The taskbar/alt-tab read ICON_BIG, the title bar ICON_SMALL — set both.
    pub fn set_window_icons() -> bool {
        unsafe {
            let hwnd = find_hwnd();
            if hwnd.is_null() { return false; }
            for (kind, size) in [(ICON_SMALL, 16u32), (ICON_BIG, 32u32)] {
                let hicon = make_hicon(size);
                if !hicon.is_null() {
                    SendMessageW(hwnd, WM_SETICON, kind, hicon as isize);
                }
            }
            true
        }
    }

    static SWALLOW: AtomicBool = AtomicBool::new(false);
    static CLICKED: AtomicBool = AtomicBool::new(false);
    static CLICK_X: AtomicI32 = AtomicI32::new(0);
    static CLICK_Y: AtomicI32 = AtomicI32::new(0);
    static HOOK: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

    static OV_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
    static OV_DC: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
    static OV_BITS: AtomicPtr<u8> = AtomicPtr::new(std::ptr::null_mut());

    static OLD_PROC: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
    static CAP_H: AtomicI32 = AtomicI32::new(0);
    static LOG_H: AtomicI32 = AtomicI32::new(0);
    static BTN_ZONE: AtomicI32 = AtomicI32::new(0);
    static MOUSE_IN: AtomicBool = AtomicBool::new(true);

    unsafe extern "system" fn overlay_wnd_proc(
        hwnd: *mut c_void,
        msg: u32,
        wparam: usize,
        lparam: isize,
    ) -> isize {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }

    unsafe extern "system" fn mouse_hook(ncode: i32, wparam: usize, lparam: isize) -> isize {
        if ncode >= 0
            && (wparam == WM_LBUTTONDOWN || wparam == WM_LBUTTONUP)
            && SWALLOW.load(Ordering::Relaxed)
        {
            if wparam == WM_LBUTTONDOWN {
                let p = lparam as *const MouseLlStruct;
                CLICK_X.store((*p).x, Ordering::Relaxed);
                CLICK_Y.store((*p).y, Ordering::Relaxed);
                CLICKED.store(true, Ordering::Relaxed);
            }
            return 1;
        }
        unsafe { CallNextHookEx(std::ptr::null_mut(), ncode, wparam, lparam) }
    }

    pub fn start_pick() {
        SWALLOW.store(true, Ordering::SeqCst);
        let h = unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), std::ptr::null_mut(), 0) };
        HOOK.store(h, Ordering::SeqCst);
    }

    pub fn stop_pick() {
        SWALLOW.store(false, Ordering::SeqCst);
        let h = HOOK.swap(std::ptr::null_mut(), Ordering::SeqCst);
        if !h.is_null() {
            unsafe { UnhookWindowsHookEx(h) };
        }
    }

    pub fn take_click() -> Option<(i32, i32)> {
        if CLICKED.swap(false, Ordering::SeqCst) {
            Some((CLICK_X.load(Ordering::SeqCst), CLICK_Y.load(Ordering::SeqCst)))
        } else {
            None
        }
    }

    fn find_hwnd() -> *mut c_void {
        unsafe {
            let class: Vec<u16> = "MINIQUADAPP\0".encode_utf16().collect();
            let mut hwnd = FindWindowW(class.as_ptr(), std::ptr::null());
            if hwnd.is_null() {
                hwnd = GetActiveWindow();
            }
            hwnd
        }
    }

    pub fn apply_win11_style(caption_bgr: u32) -> bool {
        unsafe {
            let hwnd = find_hwnd();
            if hwnd.is_null() {
                return false;
            }
            let set_u32 = |attr: u32, v: u32| {
                DwmSetWindowAttribute(hwnd, attr, &v as *const u32 as *mut c_void, 4);
            };
            set_u32(20, 1); // dark mode i guess????
            set_u32(34, caption_bgr); // border color
            set_u32(33, 2); // rounded corners
        }
        true
    }

    /// DPI scale fr
    pub fn dpi_scale() -> f32 {
        unsafe { let hwnd = find_hwnd();
            if hwnd.is_null() {
                return 1.0;
            }
            let dpi =with_physical_screen(|| GetDpiForWindow(hwnd));
            if dpi == 0 {1.0} else { (dpi as f32 /96.0).max(1.0) } }
    }

    pub fn set_topmost(on: bool) {
        unsafe {
            let hwnd = find_hwnd();
            if hwnd.is_null() {
                return;
            }
            let after = if on { -1isize } else { -2isize } as *mut c_void;
            SetWindowPos(hwnd, after, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE); } }

    // ----- custom title bar -1.draw buttons whatever -----

    unsafe extern "system" fn frame_proc(
        hwnd: *mut c_void,
        msg:u32,
        wparam: usize,
        lparam: isize,
    ) -> isize {
        unsafe {
            let old = OLD_PROC.load(Ordering::SeqCst);
            match msg {
                // whole window is client; DWM keeps rounded corners, shadow,
                // snap and native resize behavior
                WM_NCCALCSIZE if wparam != 0 => 0,
                WM_NCHITTEST => {
                    let def = CallWindowProcW(old, hwnd, msg, wparam, lparam);
                    if def != HTCLIENT { return def; }
                    let cap = CAP_H.load(Ordering::Relaxed);
                    let lh = LOG_H.load(Ordering::Relaxed);
                    let zone = BTN_ZONE.load(Ordering::Relaxed);
                    if cap <= 0 || lh <= 0 { return HTCLIENT; }
                    let mut rc = WinRect { left: 0, top: 0, right: 0, bottom: 0 };
                    if GetWindowRect(hwnd, &mut rc) == 0 { return HTCLIENT; }
                    let w = rc.right - rc.left;
                    let h = rc.bottom - rc.top;
                    let scale = h as f32 / lh as f32;
                    let mut pt = Point { x: 0, y: 0 };
                    GetCursorPos(&mut pt);
                    let x = pt.x - rc.left;
                    let y = pt.y - rc.top;
                    let bw = (8.0 * scale) as i32;
                    let zone = (zone as f32 * scale) as i32; // caption buttons area
                    let cap_px = (cap as f32 * scale) as i32;
                    let left = x < bw;
                    let right = x >= w - bw;
                    let top = y < bw;
                    let bottom = y >= h - bw;
                    if top && left { HTTOPLEFT }
                    else if top && right { HTTOPRIGHT }
                    else if bottom && left { HTBOTTOMLEFT }
                    else if bottom && right { HTBOTTOMRIGHT }
                    else if top { HTTOP }
                    else if bottom { HTBOTTOM }
                    else if left { HTLEFT }
                    else if right { HTRIGHT }
                    else if y < cap_px { if x >= w - zone { HTCLIENT } else { HTCAPTION } }
                    else { HTCLIENT } }
                _ => CallWindowProcW(old, hwnd, msg, wparam, lparam), } } }
    pub fn enable_custom_frame(cap_h: i32, logical_h: i32, btn_zone: i32) -> bool {
        unsafe {
            CAP_H.store(cap_h, Ordering::SeqCst);
            LOG_H.store(logical_h, Ordering::SeqCst);
            BTN_ZONE.store(btn_zone, Ordering::SeqCst);
            if OLD_PROC.load(Ordering::SeqCst).is_null() {
                let hwnd = find_hwnd();
                if hwnd.is_null() { return false; }
                let old = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
                if old == 0 { return false; }
                OLD_PROC.store(old as *mut c_void, Ordering::SeqCst);
                SetWindowLongPtrW(hwnd, GWLP_WNDPROC, frame_proc as *mut c_void as isize);

                // hide the native caption buttons: remove WS_CAPTION while
                // keeping WS_THICKFRAME (resize + rounded corners + shadow)
                let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
                SetWindowLongPtrW(hwnd, GWL_STYLE, (style & !WS_CAPTION) as isize);
                SetWindowPos(
                    hwnd,
                    std::ptr::null_mut(),
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                ); }
            true } }

    /// Resize the window to an exact size in physical pixels, keeping its position.
    pub fn set_window_px(w: i32, h: i32) {
        unsafe {
            let hwnd = find_hwnd();
            if !hwnd.is_null() {
                SetWindowPos(hwnd, std::ptr::null_mut(), 0, 0, w, h, SWP_NOMOVE | SWP_NOZORDER);
            }
        }
    }

    pub fn frame_tick(cap_h: i32, logical_h: i32, btn_zone: i32) {
        CAP_H.store(cap_h, Ordering::Relaxed);
        LOG_H.store(logical_h, Ordering::Relaxed);
        BTN_ZONE.store(btn_zone, Ordering::Relaxed);
        unsafe {
            let hwnd = find_hwnd();
            if !hwnd.is_null() {
                let mut rc = WinRect { left: 0, top: 0, right: 0, bottom: 0 };
                let mut pt = Point { x: 0, y: 0 };
                if GetWindowRect(hwnd, &mut rc) != 0 && GetCursorPos(&mut pt) != 0 {
                    let inside = pt.x >= rc.left && pt.x < rc.right
                        && pt.y >= rc.top && pt.y < rc.bottom;MOUSE_IN.store(inside, Ordering::Relaxed); } } }
    }

    pub fn mouse_in_window() -> bool { MOUSE_IN.load(Ordering::Relaxed) }
    pub fn set_high_res_timer() { unsafe { timeBeginPeriod(1); } }
    //for performance!!!
    pub fn monitor_refresh_hz() ->u32 {
        unsafe {
            let mut device = [0u16; 32];
            let hwnd = find_hwnd();
            if !hwnd.is_null() {
                const MONITOR_DEFAULTTONEAREST: u32 = 2;
                let hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
                if !hmon.is_null() {
                    let mut mi = MonInfoExW {
                        cb_size: std::mem::size_of::<MonInfoExW>() as u32,
                        rc_monitor: WinRect { left: 0, top: 0, right: 0, bottom: 0 },
                        rc_work: WinRect { left: 0, top: 0, right: 0, bottom: 0 },
                        flags: 0,
                        device: [0; 32],
                    };
                    if GetMonitorInfoW(hmon, &mut mi) != 0 {
                        device = mi.device;
                    } } }
            let dev_ptr: *const u16 = if device[0] == 0 {
                std::ptr::null() } else {
                device.as_ptr() };
            let mut best: u32 = 0;
            let mut i: u32 = 0;
            loop { let mut dm: DevModeW = std::mem::zeroed();
                dm.size = std::mem::size_of::<DevModeW>() as u16;if EnumDisplaySettingsW(dev_ptr, i, &mut dm) == 0 || i > 1024 {
                    break; }
                if dm.display_frequency > best { best = dm.display_frequency; }
                i += 1; }
            if best >= 30 { return best; }
            // fallback: whatever the current mode runs at
            let mut dm: DevModeW = std::mem::zeroed();
            dm.size = std::mem::size_of::<DevModeW>() as u16;
            const ENUM_CURRENT_SETTINGS: u32 = 0xFFFF_FFFF;
            if EnumDisplaySettingsW(dev_ptr, ENUM_CURRENT_SETTINGS, &mut dm) != 0
                && dm.display_frequency >= 30
            { dm.display_frequency } else { 60 }
        }
    }

    pub fn minimize_window() {
        unsafe { let hwnd = find_hwnd();if !hwnd.is_null() { ShowWindow(hwnd, SW_MINIMIZE); } }
    }

    pub fn toggle_maximize_window() {
        unsafe {
            let hwnd = find_hwnd();
            if !hwnd.is_null() {ShowWindow(hwnd, if IsZoomed(hwnd) != 0 { SW_RESTORE } else { SW_MAXIMIZE }); } } }

    pub fn is_maximized() -> bool {
        unsafe {
            let hwnd = find_hwnd();
            !hwnd.is_null() && IsZoomed(hwnd) != 0
        }
    }

    pub fn close_window() {
        unsafe {
            let hwnd = find_hwnd();
            if !hwnd.is_null() { PostMessageW(hwnd, WM_CLOSE, 0, 0); }
        }
    }

    pub fn overlay_show() {
        unsafe {
            if OV_HWND.load(Ordering::SeqCst).is_null() {
                let cls_name: Vec<u16> = "tool1_loupe\0".encode_utf16().collect();
                let mut wc: WndClassW = std::mem::zeroed();
                wc.lpfn_wnd_proc = Some(overlay_wnd_proc);
                wc.h_instance = GetModuleHandleW(std::ptr::null());
                wc.lpsz_class_name = cls_name.as_ptr();
                RegisterClassW(&wc);

                let hwnd = CreateWindowExW(
                    WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                    cls_name.as_ptr(),
                    std::ptr::null(),
                    WS_POPUP,
                    -2000, -2000, OW, OH,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    wc.h_instance,
                    std::ptr::null_mut(), );
                if hwnd.is_null() { return; }

                let screen_dc = GetDC(std::ptr::null_mut());
                let mem = CreateCompatibleDC(screen_dc);
                ReleaseDC(std::ptr::null_mut(), screen_dc);

                let mut bmi = BitmapInfo {
                    bmi_header: BitmapInfoHeader {
                        bi_size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                        bi_width: OW,
                        bi_height: -OH,
                        bi_planes: 1,
                        bi_bit_count: 32,
                        ..Default::default()
                    },
                    bmi_colors: [0],
                };
                let mut bits: *mut c_void = std::ptr::null_mut();
                let bmp = CreateDIBSection(
                    mem, &mut bmi, DIB_RGB_COLORS, &mut bits,
                    std::ptr::null_mut(), 0,
                );
                SelectObject(mem, bmp);

                OV_HWND.store(hwnd, Ordering::SeqCst);
                OV_DC.store(mem, Ordering::SeqCst);
                OV_BITS.store(bits as *mut u8, Ordering::SeqCst); }
            ShowWindow(OV_HWND.load(Ordering::SeqCst), SW_SHOWNOACTIVATE); } }

    pub fn overlay_hide() {
        unsafe {
            let hwnd = OV_HWND.load(Ordering::SeqCst);
            if !hwnd.is_null() {
                ShowWindow(hwnd, SW_HIDE);
            }
        }
    }

    fn glyph_cols(c: u8) -> Option<[u8; 5]> {
        Some(match c {
            b'0' => [0x3E, 0x51, 0x49, 0x45, 0x3E],
            b'1' => [0x00, 0x42, 0x7F, 0x40, 0x00],
            b'2' => [0x42, 0x61, 0x51, 0x49, 0x46],
            b'3' => [0x21, 0x41, 0x45, 0x4B, 0x31],
            b'4' => [0x18, 0x14, 0x12, 0x7F, 0x10],
            b'5' => [0x27, 0x45, 0x45, 0x45, 0x39],
            b'6' => [0x3C, 0x4A, 0x49, 0x49, 0x30],
            b'7' => [0x01, 0x71, 0x09, 0x05, 0x03],
            b'8' => [0x36, 0x49, 0x49, 0x49, 0x36],
            b'9' => [0x06, 0x49, 0x49, 0x29, 0x1E],
            b'a' => [0x20, 0x54, 0x54, 0x54, 0x78],
            b'b' => [0x7C, 0x54, 0x54, 0x54, 0x28],
            b'c' => [0x38, 0x44, 0x44, 0x44, 0x20],
            b'd' => [0x38, 0x44, 0x44, 0x48, 0x7F],
            b'e' => [0x38, 0x54, 0x54, 0x54, 0x18],
            b'f' => [0x08, 0x7E, 0x09, 0x01, 0x02],
            b'g' => [0x0C, 0x52, 0x52, 0x52, 0x3E],
            b'h' => [0x7F, 0x08, 0x04, 0x04, 0x78],
            b'i' => [0x00, 0x44, 0x7D, 0x40, 0x00],
            b'k' => [0x7F, 0x10, 0x28, 0x44, 0x00],
            b'l' => [0x00, 0x41, 0x7F, 0x40, 0x00],
            b'm' => [0x7C, 0x04, 0x18, 0x04, 0x78],
            b'n' => [0x7C, 0x08, 0x04, 0x04, 0x78],
            b'o' => [0x38, 0x44, 0x44, 0x44, 0x38],
            b'p' => [0x7C, 0x14, 0x14, 0x14, 0x08],
            b'r' => [0x7C, 0x08, 0x04, 0x04, 0x08],
            b's' => [0x48, 0x54, 0x54, 0x54, 0x20],
            b't' => [0x04, 0x3F, 0x44, 0x40, 0x20],
            b'u' => [0x3C, 0x40, 0x40, 0x20, 0x7C],
            b'v' => [0x1C, 0x20, 0x40, 0x20, 0x1C],
            b'w' => [0x3C, 0x40, 0x30, 0x40, 0x3C],
            b'x' => [0x44, 0x28, 0x10, 0x28, 0x44],
            b'y' => [0x0C, 0x50, 0x50, 0x50, 0x3C],
            b':' => [0x00, 0x36, 0x36, 0x00, 0x00],
            b'+' => [0x08, 0x08, 0x3E, 0x08, 0x08],
            b'|' => [0x00, 0x7F, 0x7F, 0x00, 0x00],
            b'#' => [0x14, 0x7F, 0x14, 0x7F, 0x14],
            b'-' => [0x08, 0x08, 0x08, 0x08, 0x08],
            _ => return None,
        })
    }

    fn text_width(s: &str, scale: i32) -> i32 { (s.len() as i32 * 6 - 1).max(0) * scale }

    fn draw_text_px(img: &mut [u8], s: &str, x: i32, y: i32, scale: i32, col: [u8; 3]) {
        let mut cx = x;
        for ch in s.bytes() {
            if ch == b' ' {
                cx += 6 * scale;
                continue;
            }
            if let Some(cols) = glyph_cols(ch.to_ascii_lowercase()) {
                for (gx, bits) in cols.iter().enumerate() {
                    for row in 0..7u8 {
                        if bits & (1 << row) != 0 {
                            for sy in 0..scale {
                                for sx in 0..scale {
                                    let px = cx + gx as i32 * scale + sx;
                                    let py = y + row as i32 * scale + sy;
                                    if px >= 0 && px < OW && py >= 0 && py < OH {
                                        let i = ((py * OW + px) * 4) as usize;
                                        img[i] = col[0];
                                        img[i + 1] = col[1];
                                        img[i + 2] = col[2];img[i + 3] = 255; }
                                }
                            }
                        }
                    }
                }//wow this is clean and i didnt even do it myself w ide(no)
            }//i think the ai is fixing too much things
            //i just tell the ai to fix my typo and it did all that shit
            cx += 6 * scale;
        }
    }

    pub fn overlay_update(px: &[u8], sr: u8, sg: u8, sb: u8, _gx: i32, _gy: i32) {
        unsafe {
            let hwnd = OV_HWND.load(Ordering::SeqCst);
            let dc = OV_DC.load(Ordering::SeqCst);
            let bits = OV_BITS.load(Ordering::SeqCst);
            if hwnd.is_null() || dc.is_null() || bits.is_null() { return; }

            let (vx, vy) = cursor_virtual();
            let scr_w = GetSystemMetrics(SM_CXSCREEN);
            let scr_h = GetSystemMetrics(SM_CYSCREEN);
            let gap = 24;
            let mut ox = vx + gap;
            let mut oy = vy + gap;
            if ox + OW > scr_w {
                ox = vx - OW - gap;
            }
            if oy + OH > scr_h {
                oy = vy - OH - gap;
            }
            ox = ox.clamp(0, (scr_w - OW).max(0));
            oy = oy.clamp(0, (scr_h - OH).max(0));

            let mut img = vec![0u8; (OW * OH * 4) as usize];
            let rad = 12.0f32;

            for y in 0..OH {
                for x in 0..OW {
                    let fx = x as f32 + 0.5;
                    let fy = y as f32 + 0.5;
                    let cx = fx.clamp(rad, OW as f32 - rad);
                    let cy = fy.clamp(rad, OH as f32 - rad);
                    let d = (fx - cx).hypot(fy - cy) - rad;
                    let (r, g, b, a) = if d > 0.5 {
                        (0, 0, 0, 0u32)
                    } else if d >= -0.5 {
                        (205u32, 210u32, 220u32, 230u32)
                    } else {
                        (20u32, 23u32, 28u32, 247u32)
                    };
                    let i = ((y * OW + x) * 4) as usize;
                    img[i] = r as u8;
                    img[i + 1] = g as u8;
                    img[i + 2] = b as u8;
                    img[i + 3] = a as u8;
                }
            }

            let mut put = |x: i32, y: i32, r: u8, g: u8, b: u8, a: u8| {
                if x >= 0 && x < OW && y >= 0 && y < OH {
                    let i = ((y * OW + x) * 4) as usize;
                    img[i] = r;
                    img[i + 1] = g;
                    img[i + 2] = b;
                    img[i + 3] = a;
                }
            };

            let lx0 = 8;
            let ly0 = 8;
            let n = OL_N as usize;
            let cc0 = (OL_N / 2) * OCELL;

            for ly in 0..OLOUPE {
                for lx in 0..OLOUPE {
                    let tx = (lx / OCELL).clamp(0, n as i32 - 1) as usize;
                    let ty = (ly / OCELL).clamp(0, n as i32 - 1) as usize;
                    let ti = (ty * n + tx) * 4;
                    let mut r = px[ti] as i32;
                    let mut g = px[ti + 1] as i32;
                    let mut b = px[ti + 2] as i32;

                    if lx % OCELL == 0 || ly % OCELL == 0 {
                        let lum = (r * 299 + g * 587 + b * 114) / 1000;
                        if lum > 110 {
                            r = r * 55 / 100;
                            g = g * 55 / 100;
                            b = b * 55 / 100;
                        } else {
                            r = (r * 55 + 255 * 45) / 100;
                            g = (g * 55 + 255 * 45) / 100;
                            b = (b * 55 + 255 * 45) / 100;
                        }
                    }

                    let relx = lx - cc0;
                    let rely = ly - cc0;
                    let in_cx = relx >= 0 && relx < OCELL;
                    let in_cy = rely >= 0 && rely < OCELL;

                    if (in_cx && (rely == -1 || rely == OCELL))
                        || (in_cy && (relx == -1 || relx == OCELL))
                    {
                        r = 255;
                        g = 255;
                        b = 255;
                    }

                    if in_cx && in_cy {
                        let on_outline =
                            relx < 2 || relx >= OCELL - 2 || rely < 2 || rely >= OCELL - 2;
                        let on_cross =
                            relx == 3 || relx == 4 || rely == 3 || rely == 4;
                        if on_outline || on_cross {
                            r = 226;
                            g = 74;
                            b = 62;
                        }
                    }

                    put(lx0 + lx, ly0 + ly, r as u8, g as u8, b as u8, 255);
                }
            }

            let swx = 8;
            let swy = ly0 + OLOUPE + 8;
            let sww = 56;
            let swh = 16;
            for yy in 0..swh {
                for xx in 0..sww {
                    if yy == 0 || yy == swh - 1 || xx == 0 || xx == sww - 1 {
                        put(swx + xx, swy + yy, 60, 60, 60, 255);
                    } else {
                        put(swx + xx, swy + yy, sr, sg, sb, 255);
                    }
                }
            }
            let hex_str = format!("#{:02X}{:02X}{:02X}", sr, sg, sb);
            draw_text_px(&mut img, &hex_str, swx + sww + 8, swy + 2, 2, [217, 222, 230]);

            let h1 = "click: pick + copy hex";
            let h2 = "arrows: nudge | esc: cancel";
            let hint_col = [140u8, 145, 152];
            draw_text_px(&mut img, h1, (OW - text_width(h1, 1)) / 2, swy + swh + 6, 1, hint_col);
            draw_text_px(&mut img, h2, (OW - text_width(h2, 1)) / 2, swy + swh + 20, 1, hint_col);

            for i in (0..img.len()).step_by(4) {
                let a = img[i + 3] as u32;
                img[i] = ((img[i] as u32 * a) / 255) as u8;
                img[i + 1] = ((img[i + 1] as u32 * a) / 255) as u8;
                img[i + 2] = ((img[i + 2] as u32 * a) / 255) as u8;
            }

            std::ptr::copy_nonoverlapping(img.as_ptr(), bits, img.len());

            let mut dst = Point { x: ox, y: oy };
            let mut size = Size { cx: OW, cy: OH };
            let mut src = Point { x: 0, y: 0 };
            let mut blend = BlendFunction {
                blend_op: 0,
                blend_flags: 0,
                source_constant_alpha: 255,
                alpha_format: 1,
            };
            UpdateLayeredWindow(
                hwnd,
                std::ptr::null_mut(),
                &mut dst,
                &mut size,
                dc,
                &mut src,
                0,
                &mut blend,
                ULW_ALPHA,
            );
        }
    }

    fn cursor_virtual() -> (i32, i32) {
        let mut p = Point { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut p) };
        (p.x, p.y)
    }

    fn with_physical_screen<T>(f: impl FnOnce() -> T) -> T {
        unsafe {
            let old = SetThreadDpiAwarenessContext(PER_MONITOR_AWARE_V2);
            let out = f();
            if old != 0 {
                SetThreadDpiAwarenessContext(old);
            }
            out
        }
    }

    pub fn cursor() -> (i32, i32) {
        with_physical_screen(|| {
            let mut p = Point { x: 0, y: 0 };
            unsafe { GetCursorPos(&mut p) };
            (p.x, p.y)
        })
    }

    pub fn set_cursor(x: i32, y: i32) {
        with_physical_screen(|| {
            unsafe { SetCursorPos(x, y) };
        });
    }

    pub fn region(cx: i32, cy: i32, w: i32, h: i32) -> Vec<u8> {
        with_physical_screen(|| unsafe {
            let mut out = vec![0u8; (w * h * 4) as usize];

            let screen = GetDC(std::ptr::null_mut());
            let mem = CreateCompatibleDC(screen);
            let bmp = CreateCompatibleBitmap(screen, w, h);
            let old_sel = SelectObject(mem, bmp);
            BitBlt(mem, 0, 0, w, h, screen, cx - w / 2, cy - h / 2, SRCCOPY);

            let mut bmi = BitmapInfo {
                bmi_header: BitmapInfoHeader {
                    bi_size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                    bi_width: w,
                    bi_height: -h,
                    bi_planes: 1,
                    bi_bit_count: 32,
                    ..Default::default()
                },
                bmi_colors: [0],
            };
            SelectObject(mem, old_sel);
            GetDIBits(mem, bmp, 0, h as u32, out.as_mut_ptr(), &mut bmi, 0);

            DeleteObject(bmp);
            DeleteDC(mem);
            ReleaseDC(std::ptr::null_mut(), screen);

            for i in (0..out.len()).step_by(4) {
                out.swap(i, i + 2);
                out[i + 3] = 255; }out }) } }
//shhhh u would never know
// ---------- macOS: native title bar, the system color sampler, window level ----------
#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)] // objc 0.2's msg_send! checks an old clippy cfg
mod screen {
    use block::ConcreteBlock;
    use objc::runtime::{Class, Object};
    use objc::{class, msg_send, sel, sel_impl};
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicPtr, Ordering};

    pub const CAPTURE_N: i32 = 31;
    pub const CUSTOM_FRAME: bool = false;
    pub const CAN_PICK: bool = true;
    pub const SYSTEM_SAMPLER: bool = true;

    /// What the sampler handed back: Some(Some(rgb)) picked, Some(None) cancelled.
    static RESULT: Mutex<Option<Option<(u8, u8, u8)>>> = Mutex::new(None);
    /// The open NSColorSampler, released once it has answered.
    static SAMPLER: AtomicPtr<Object> = AtomicPtr::new(std::ptr::null_mut());

    fn q(v: f64) -> u8 {
        (v.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    /// Opens macOS's own eyedropper (NSColorSampler, macOS 10.15+): it brings
    /// its magnifier, handles the click and Esc, and needs no screen-recording
    /// permission.
    pub fn start_pick() {
        *RESULT.lock().unwrap() = None;
        let Some(cls) = Class::get("NSColorSampler") else {
            *RESULT.lock().unwrap() = Some(None); // older macOS: nothing to show
            return;
        };
        unsafe {
            let sampler: *mut Object = msg_send![cls, new];
            SAMPLER.store(sampler, Ordering::SeqCst);
            let handler = ConcreteBlock::new(|color: *mut Object| {
                let picked = if color.is_null() {
                    None
                } else {
                    let srgb: *mut Object = msg_send![class!(NSColorSpace), sRGBColorSpace];
                    let c: *mut Object = msg_send![color, colorUsingColorSpace: srgb];
                    if c.is_null() {
                        None
                    } else {
                        let r: f64 = msg_send![c, redComponent];
                        let g: f64 = msg_send![c, greenComponent];
                        let b: f64 = msg_send![c, blueComponent];
                        Some((q(r), q(g), q(b)))
                    }
                };
                *RESULT.lock().unwrap() = Some(picked);
            })
            .copy();
            let _: () = msg_send![sampler, showSamplerWithSelectionHandler: &*handler];
        }
    }

    pub fn sampler_result() -> Option<Option<(u8, u8, u8)>> {
        let res = RESULT.lock().unwrap().take();
        if res.is_some() {
            let s = SAMPLER.swap(std::ptr::null_mut(), Ordering::SeqCst);
            if !s.is_null() {
                unsafe { let _: () = msg_send![s, release]; }
            }
        }
        res
    }

    /// Float the app's windows above others (NSFloatingWindowLevel = 3).
    pub fn set_topmost(on: bool) {
        unsafe {
            let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            let windows: *mut Object = msg_send![app, windows];
            let count: usize = msg_send![windows, count];
            for i in 0..count {
                let w: *mut Object = msg_send![windows, objectAtIndex: i];
                let level: isize = if on { 3 } else { 0 };
                let _: () = msg_send![w, setLevel: level];
            }
        }
    }

    pub fn stop_pick() {}
    pub fn take_click() -> Option<(i32, i32)> { None }
    pub fn set_window_icons() -> bool { true }
    pub fn apply_win11_style(_caption_bgr: u32) -> bool { true }
    pub fn dpi_scale() -> f32 { 1.0 }
    pub fn enable_custom_frame(_cap: i32, _lh: i32, _zone: i32) -> bool { true }
    pub fn set_window_px(_w: i32, _h: i32) {}
    pub fn frame_tick(_cap: i32, _lh: i32, _zone: i32) {}
    pub fn mouse_in_window() -> bool { true }
    pub fn monitor_refresh_hz() -> u32 { 120 }
    pub fn set_high_res_timer() {}
    pub fn minimize_window() {}
    pub fn toggle_maximize_window() {}
    pub fn is_maximized() -> bool { false }
    pub fn close_window() {}
    pub fn overlay_show() {}
    pub fn overlay_hide() {}
    pub fn overlay_update(_px: &[u8], _r: u8, _g: u8, _b: u8, _x: i32, _y: i32) {}
    pub fn cursor() -> (i32, i32) { (0, 0) }
    pub fn set_cursor(_x: i32, _y: i32) {}
    pub fn region(_cx: i32, _cy: i32, w: i32, h: i32) -> Vec<u8> {
        vec![0u8; (w * h * 4) as usize]
    }
}

// ---------- anything else (Linux, ...): the picker without screen picking ----------
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod screen {
    pub const CAPTURE_N: i32 = 31;
    pub const CUSTOM_FRAME: bool = false;
    pub const CAN_PICK: bool = false;
    pub const SYSTEM_SAMPLER: bool = false;
    pub fn sampler_result() -> Option<Option<(u8, u8, u8)>> { None }
    pub fn start_pick() {}
    pub fn stop_pick() {}
    pub fn take_click() -> Option<(i32, i32)> { None }
    pub fn set_window_icons() -> bool { true }
    pub fn apply_win11_style(_caption_bgr: u32) -> bool { true }
    pub fn dpi_scale() -> f32 { 1.0 }
    pub fn set_topmost(_on: bool) {}
    pub fn enable_custom_frame(_cap: i32, _lh: i32, _zone: i32) -> bool { true }
    pub fn set_window_px(_w: i32, _h: i32) {}
    pub fn frame_tick(_cap: i32, _lh: i32, _zone: i32) {}
    pub fn mouse_in_window() -> bool { true }
    pub fn monitor_refresh_hz() -> u32 { 60 }
    pub fn set_high_res_timer() {}
    pub fn minimize_window() {}
    pub fn toggle_maximize_window() {}
    pub fn is_maximized() -> bool { false }
    pub fn close_window() {}
    pub fn overlay_show() {}
    pub fn overlay_hide() {}
    pub fn overlay_update(_px: &[u8], _r: u8, _g: u8, _b: u8, _x: i32, _y: i32) {}
    pub fn cursor() -> (i32, i32) { (0, 0) }
    pub fn set_cursor(_x: i32, _y: i32) {}
    pub fn region(_cx: i32, _cy: i32, w: i32, h: i32) -> Vec<u8> {
        vec![0u8; (w * h * 4) as usize]
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Widget {
    SatVal,
    Hue,
    Alpha,
    Chan(usize),
    SwatchScroll,
}

#[derive(Clone, Copy, PartialEq)]
struct Picker {
    hue: f32,
    sat: f32,
    val: f32,
    alpha: f32,
    hsl_s: f32,
    lab: (f32, f32, f32),
    oklch: (f32, f32, f32),
    cie_xy: (f64, f64),
    hsl: (f32, f32, f32),           // H deg, S, L
    cmyk: (f32, f32, f32, f32),
    hwb: (f32, f32, f32),           // H deg, W, B
    plane_pos: (f32, f32),
    yuv: (f32, f32, f32),
    ypbpr: (f32, f32, f32),
    xyz: (f64, f64, f64),
    xyy: (f64, f64, f64),
    munsell: (f32, f32, f32),
    keep_lab: bool,
    keep_oklch: bool,
    keep_cie: bool,
    keep_hsl: bool,
    keep_cmyk: bool,
    keep_hwb: bool,
    keep_yuv: bool,
    keep_ypbpr: bool,
    keep_xyz: bool,
    keep_xyy: bool,
    keep_munsell: bool,
    keep_plane: bool,
    keep_hsv: bool,
}

impl Picker {
    fn new() -> Self {
        let mut p = Self {
            hue: 205.0, sat: 0.85, val: 0.95, alpha: 1.0,
            hsl_s: 0.85,
            lab: (50.0, 0.0, 0.0),
            oklch: (0.5, 0.1, 205.0),
            cie_xy: (0.3127, 0.3290),
            hsl: (205.0, 0.85, 0.5),
            cmyk: (0.0, 0.0, 0.0, 0.0),
            hwb: (205.0, 0.0, 0.05),
            plane_pos: (0.5, 0.5),
            yuv: (0.0, 0.0, 0.0),
            ypbpr: (0.0, 0.0, 0.0),
            xyz: (0.0, 0.0, 0.0),
            xyy: (0.3127, 0.3290, 0.5),
            munsell: (0.0, 0.0, 0.0),
            keep_lab: false,
            keep_oklch: false,
            keep_cie: false,
            keep_hsl: false,
            keep_cmyk: false,
            keep_hwb: false,
            keep_plane: false,
            keep_yuv: false,
            keep_ypbpr: false,
            keep_xyz: false,
            keep_xyy: false,
            keep_munsell: false,
            keep_hsv: false, };
        let (r, g, b) = hsv_to_rgb_f(p.hue, p.sat, p.val);
        p.refresh_derived(r, g, b);
        p
    }

    fn rgb_f32(&self) -> (f32, f32, f32) {
        hsv_to_rgb_f(self.hue, self.sat, self.val)
    }

    fn rgb(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.rgb_f32();
        (q8(r), q8(g), q8(b))
    }

    fn color(&self) -> Color {
        let (r, g, b) = self.rgb();
        Color::from_rgba(r, g, b, q8(self.alpha))
    }

    fn sync_hsv(&mut self, r: u8, g: u8, b: u8) {
        self.sync_rgb_f(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    }

    fn refresh_derived(&mut self, r: f32, g: f32, b: f32) {
        if !self.keep_lab {
            let (l, a, bb) = rgb_to_lab(r, g, b);
            self.lab = (l as f32, a as f32, bb as f32);
        }
        if !self.keep_oklch {
            let (ll, c, h) = rgb_to_oklch(r, g, b);
            self.oklch = (ll as f32, c as f32, h as f32);
        }
        if !self.keep_cie {
            let (x, y) = rgb_to_xy(r, g, b);
            self.cie_xy = (x as f64, y as f64);
        }
        if !self.keep_hsl {
            let (h, s, l) = rgb_to_hsl(r, g, b);
            // hue is undefined at S=0: keep the previous sticky hue so the
            // H slider thumb stays where the user put it through black/gray
            let h = if s > 0.0005 { h } else { self.hsl.0 };
            self.hsl = (h, s, l);
        }
        if !self.keep_cmyk {
            self.cmyk = rgb_to_cmyk(r, g, b);
        }
        if !self.keep_hwb {
            let (h, w, bl) = rgb_to_hwb(r, g, b);
            // w + b >= 1
            let h = if w + bl < 0.9995 { h } else { self.hwb.0 };
            self.hwb = (h, w, bl);
        }
        if !self.keep_yuv {
            self.yuv = rgb_to_yuv(r, g, b);
        }
        if !self.keep_ypbpr {
            self.ypbpr = rgb_to_ypbpr(r, g, b);
        }
        if !self.keep_xyz {
            self.xyz = rgb_to_xyz(r, g, b);
        }
        if !self.keep_xyy {
            let (cx, cy) = rgb_to_xy(r, g, b);
            let big_y = rgb_to_xyz(r, g, b).1;
            self.xyy = (cx as f64, cy as f64, big_y);
        }
        if !self.keep_munsell {
            let (h, v, c) = rgb_to_munsell(r, g, b);
            // hue is undefined near C=0 (gray): keep the previous one
            let h = if c > 0.05 { h } else { self.munsell.0 };
            self.munsell = (h, v, c);
        }
        self.keep_lab = false;
        self.keep_oklch = false;
        self.keep_cie = false;
        self.keep_hsl = false;
        self.keep_cmyk = false;
        self.keep_hwb = false;
        self.keep_yuv = false;
        self.keep_ypbpr = false;
        self.keep_xyz = false;
        self.keep_xyy = false;
        self.keep_munsell = false;

    }

    fn sync_rgb_f(&mut self, r: f32, g: f32, b: f32) {
        let (r, g, b) = (
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
        );
        let (h, s, v) = rgb_to_hsv_f(r, g, b);
        let keep_hsv = self.keep_hsv;
        self.keep_hsv = false;
        if !keep_hsv {
            if s > 0.0001 { self.hue = h; }self.sat = s;self.val = v; }

        if r.max(g).max(b) - r.min(g).min(b) > 0.003 {
            let (_, hsl_s, _) = rgb_to_hsl(r, g, b);
            self.hsl_s = hsl_s;
        }
        self.refresh_derived(r, g, b);
    }
}

// ---------- ui kit ----------
// every rounded shape (panels, pills, thumbs, icons, shadows) is one quad run
// through a signed-distance shader, so edges stay crisp at any window size

const SHAPE_VS: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying mediump vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    color = color0 / 255.0;
    uv = texcoord;
}"#;

const SHAPE_FS: &str = r#"#version 100
precision mediump float;
varying mediump vec2 uv;
varying lowp vec4 color;
uniform sampler2D Texture;
uniform vec2 u_size;     // quad size, physical px
uniform float u_radius;  // corner radius, px
uniform float u_stroke;  // > 0: outline of this width instead of a fill
uniform float u_blur;    // edge softness, px (1 = plain antialiasing)
uniform float u_checker; // > 0: composite over a checkerboard of this cell size
uniform vec2 u_inset;    // texture is squeezed inward by this uv fraction
void main() {
    vec2 p = (uv - 0.5) * u_size;
    vec2 half_size = u_size * 0.5 - (u_blur - 1.0) * 0.5;
    vec2 q = abs(p) - (half_size - u_radius);
    float d = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - u_radius;
    float a = clamp(0.5 - d / u_blur, 0.0, 1.0);
    if (u_stroke > 0.0) a *= clamp(0.5 + d + u_stroke, 0.0, 1.0);
    vec2 t = clamp((uv - u_inset) / (1.0 - 2.0 * u_inset), 0.002, 0.998);
    vec4 c = texture2D(Texture, t) * color;
    if (u_checker > 0.0) {
        vec2 g = floor(uv * u_size / u_checker);
        float k = mod(g.x + g.y, 2.0);
        c = vec4(mix(mix(vec3(1.0), vec3(0.76), k), c.rgb, c.a), 1.0);
    }
    gl_FragColor = vec4(c.rgb, c.a * a);
}"#;

fn with_alpha(c: Color, a: f32) -> Color {
    Color::new(c.r, c.g, c.b, a)
}

/// First loadable font from the OS font folders; `None` falls back to
/// macroquad's built-in font.
fn system_font(names: &[&str]) -> Option<Font> {
    let mut dirs: Vec<std::path::PathBuf> = Vec::new();
    if let Some(win) = std::env::var_os("WINDIR") {
        dirs.push(std::path::PathBuf::from(win).join("Fonts"));
    }
    for d in ["/System/Library/Fonts/Supplemental", "/usr/share/fonts/truetype/dejavu"] {
        dirs.push(std::path::PathBuf::from(d));
    }
    for dir in &dirs {
        for name in names {
            if let Ok(bytes) = std::fs::read(dir.join(name)) {
                if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                    return Some(font);
                }
            }
        }
    }
    None
}

struct Shape<'a> {
    radius: f32,
    stroke: f32,
    blur: f32,
    checker: f32,
    inset: Vec2,
    tex: Option<&'a Texture2D>,
    rot: f32,
}

impl Default for Shape<'_> {
    fn default() -> Self {
        Self { radius: 0.0, stroke: 0.0, blur: 0.0, checker: 0.0, inset: Vec2::ZERO, tex: None, rot: 0.0 }
    }
}

struct Ui {
    mat: Material,
    font: Option<Font>,
    bold: Option<Font>,
    /// physical pixels per design unit
    px: f32,
}

impl Ui {
    fn new() -> Self {
        let mat = load_material(
            ShaderSource::Glsl { vertex: SHAPE_VS, fragment: SHAPE_FS },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                uniforms: vec![
                    UniformDesc::new("u_size", UniformType::Float2),
                    UniformDesc::new("u_radius", UniformType::Float1),
                    UniformDesc::new("u_stroke", UniformType::Float1),
                    UniformDesc::new("u_blur", UniformType::Float1),
                    UniformDesc::new("u_checker", UniformType::Float1),
                    UniformDesc::new("u_inset", UniformType::Float2),
                ],
                ..Default::default()
            },
        )
        .expect("shape shader");
        Self {
            mat,
            font: system_font(&["segoeui.ttf", "Arial.ttf", "DejaVuSans.ttf"]),
            bold: system_font(&["seguisb.ttf", "segoeuib.ttf", "Arial Bold.ttf", "DejaVuSans-Bold.ttf"]),
            px: 1.0,
        }
    }

    /// Point the camera at window space measured in design units.
    fn begin(&mut self, scale: f32) {
        let (sw, sh) = (screen_width(), screen_height());
        self.px = scale * screen_dpi_scale();
        set_camera(&Camera2D {
            target: vec2(sw * 0.5 / scale, sh * 0.5 / scale),
            zoom: vec2(2.0 * scale / sw, 2.0 * scale / sh),
            ..Default::default()
        });
    }

    /// Restrict drawing to `r`, or lift the restriction with `None`.
    fn clip(&self, r: Option<Rect>) {
        let px = |v: f32| (v * self.px).round() as i32;
        let gl = unsafe { get_internal_gl() };
        gl.quad_gl.scissor(r.map(|r| (px(r.x), px(r.y), px(r.w), px(r.h))));
    }

    /// Round a design-space coordinate to the physical pixel grid.
    fn snap(&self, v: f32) -> f32 {
        (v * self.px).round() / self.px
    }

    fn shape(&self, r: Rect, s: Shape, color: Color) {
        // draw_rectangle_ex runs u along the quad's height
        let size = if s.rot != 0.0 { vec2(r.h, r.w) } else { vec2(r.w, r.h) } * self.px;
        let stroke = if s.stroke > 0.0 { (s.stroke * self.px).max(1.0) } else { 0.0 };
        self.mat.set_uniform("u_size", size);
        self.mat.set_uniform("u_radius", s.radius.min(r.w.min(r.h) * 0.5) * self.px);
        self.mat.set_uniform("u_stroke", stroke);
        self.mat.set_uniform("u_blur", (s.blur * self.px).max(1.0));
        self.mat.set_uniform("u_checker", s.checker * self.px);
        self.mat.set_uniform("u_inset", s.inset);
        gl_use_material(&self.mat);
        match s.tex {
            Some(t) => draw_texture_ex(t, r.x, r.y, color, DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            }),
            None if s.rot != 0.0 => draw_rectangle_ex(
                r.x + r.w * 0.5, r.y + r.h * 0.5, r.w, r.h,
                DrawRectangleParams { offset: vec2(0.5, 0.5), rotation: s.rot, color },
            ),
            None => draw_rectangle(r.x, r.y, r.w, r.h, color),
        }
        gl_use_default_material();
    }

    fn fill(&self, r: Rect, radius: f32, color: Color) {
        self.shape(r, Shape { radius, ..Default::default() }, color);
    }

    fn stroke(&self, r: Rect, radius: f32, width: f32, color: Color) {
        self.shape(r, Shape { radius, stroke: width, ..Default::default() }, color);
    }

    /// One-physical-pixel outline, however far the ui is scaled.
    fn hair(&self, r: Rect, radius: f32, color: Color) {
        self.stroke(r, radius, 1.0 / self.px, color);
    }

    /// Soft drop shadow under `r`.
    fn shadow(&self, r: Rect, radius: f32, blur: f32, color: Color) {
        let grown = Rect::new(r.x - blur * 0.5, r.y - blur * 0.5 + blur * 0.2, r.w + blur, r.h + blur);
        self.shape(grown, Shape { radius, blur, ..Default::default() }, color);
    }

    fn tex(&self, r: Rect, radius: f32, tex: &Texture2D) {
        self.shape(r, Shape { radius, tex: Some(tex), ..Default::default() }, WHITE);
    }

    /// `color` (with its alpha) over a transparency checkerboard.
    fn checker(&self, r: Rect, radius: f32, cell: f32, color: Color) {
        self.shape(r, Shape { radius, checker: cell, ..Default::default() }, color);
    }

    /// A gradient track with fully rounded ends. The gradient spans the
    /// thumb's travel (GRIP_HALF in from each end), so the color under the
    /// thumb is exact and the thumb never sticks out past the track.
    fn strip(&self, r: Rect, tex: &Texture2D, checker: f32, tint: Color) {
        let radius = r.w.min(r.h) * 0.5;
        let inset = if r.w >= r.h { vec2(GRIP_HALF / r.w, 0.0) } else { vec2(0.0, GRIP_HALF / r.h) };
        self.shape(r, Shape { radius, tex: Some(tex), checker, inset, ..Default::default() }, tint);
    }

    fn ring(&self, c: Vec2, radius: f32, width: f32, color: Color) {
        self.stroke(Rect::new(c.x - radius, c.y - radius, radius * 2.0, radius * 2.0), radius, width, color);
    }

    /// Antialiased round-capped line.
    fn line(&self, a: Vec2, b: Vec2, width: f32, color: Color) {
        let d = b - a;
        let len = d.length() + width;
        let c = (a + b) * 0.5;
        self.shape(
            Rect::new(c.x - len * 0.5, c.y - width * 0.5, len, width),
            Shape { radius: width * 0.5, rot: d.y.atan2(d.x) + 1e-6, ..Default::default() },
            color,
        );
    }

    /// One-physical-pixel axis-aligned rule.
    fn hairline(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let one = 1.0 / self.px;
        draw_rectangle(self.snap(x), self.snap(y), w.max(one), h.max(one), color);
    }

    /// The plane's cursor: an open ring, so the color under it stays visible.
    fn reticle(&self, c: Vec2, radius: f32) {
        self.ring(c, radius + 0.75, 2.75, Color::new(0.0, 0.0, 0.0, 0.45));
        self.ring(c, radius, 1.25, WHITE);
    }

    /// Slider handle: a white frame laid across the track, filled with the
    /// color the track has at that point.
    fn marker(&self, c: Vec2, w: f32, h: f32, fill: Color) {
        let centered = |w: f32, h: f32| Rect::new(c.x - w * 0.5, c.y - h * 0.5, w, h);
        self.fill(centered(w + 2.0, h + 2.0), 2.5, Color::new(0.0, 0.0, 0.0, 0.6));
        self.fill(centered(w, h), 1.5, WHITE);
        self.checker(centered(w - 2.6, h - 2.6), 0.5, 2.0, fill);
    }

    /// Sunken input box.
    fn field(&self, r: Rect, fill: Color, edge: Color) {
        self.fill(r, 5.0, fill);
        self.hair(r, 5.0, edge);
    }

    fn face(&self, bold: bool) -> Option<&Font> {
        if bold { self.bold.as_ref().or(self.font.as_ref()) } else { self.font.as_ref() }
    }

    /// Glyphs are rasterized at the on-screen pixel size and scaled back into
    /// design units, so text is sharp at any window scale.
    fn raster(&self, size: f32) -> (u16, f32) {
        let fs = (size * self.px).round().max(4.0);
        (fs as u16, size / fs)
    }

    fn width(&self, s: &str, size: f32, bold: bool) -> f32 {
        let (fs, k) = self.raster(size);
        measure_text(s, self.face(bold), fs, k).width
    }

    /// Text vertically centered on `cy`; `align` 0 = starts at x, 0.5 =
    /// centered on x, 1 = ends at x.
    fn label(&self, s: &str, x: f32, cy: f32, size: f32, bold: bool, color: Color, align: f32) {
        self.text(self.face(bold), s, x, cy, size, color, align);
    }

    fn text(&self, font: Option<&Font>, s: &str, x: f32, cy: f32, size: f32, color: Color, align: f32) {
        let (fs, k) = self.raster(size);
        let x = if align > 0.0 { x - measure_text(s, font, fs, k).width * align } else { x };
        draw_text_ex(s, self.snap(x), self.snap(cy + size * 0.36), TextParams {
            font,
            font_size: fs,
            font_scale: k,
            color,
            ..Default::default()
        });
    }

    /// Primary button: solid white, like Photon's OK.
    fn button(&self, r: Rect, hot: bool) {
        self.fill(r, 5.0, if hot { WHITE } else { Color::from_rgba(226, 226, 229, 255) });
    }

    // ----- icons (drawn, so they scale with everything else) -----

    fn icon_copy(&self, c: Vec2, color: Color, bg: Color) {
        let back = Rect::new(c.x - 2.0, c.y - 5.5, 7.5, 8.5);
        let front = Rect::new(c.x - 5.5, c.y - 3.0, 7.5, 8.5);
        self.stroke(back, 2.0, 1.2, color);
        self.fill(Rect::new(front.x - 1.0, front.y - 1.0, front.w + 1.0, front.h + 1.0), 2.5, bg);
        self.stroke(front, 2.0, 1.2, color);
    }

    fn icon_check(&self, c: Vec2, color: Color) {
        let mid = c + vec2(-1.5, 3.5);
        self.line(c + vec2(-5.0, 0.0), mid, 1.6, color);
        self.line(mid, c + vec2(5.0, -3.5), 1.6, color);
    }

    fn icon_chevron(&self, c: Vec2, up: bool, color: Color) {
        let s = if up { -1.0 } else { 1.0 };
        let tip = c + vec2(0.0, 1.8 * s);
        self.line(c + vec2(-3.5, -1.8 * s), tip, 1.4, color);
        self.line(tip, c + vec2(3.5, -1.8 * s), 1.4, color);
    }

    fn icon_pin(&self, c: Vec2, color: Color) {
        let (sin, cos) = 0.6f32.sin_cos();
        let at = |x: f32, y: f32| c + vec2(x * cos - y * sin, x * sin + y * cos);
        self.line(at(0.0, -5.5), at(0.0, -2.2), 4.0, color); // head
        self.line(at(-3.8, 0.0), at(3.8, 0.0), 1.3, color);  // collar
        self.line(at(0.0, 0.5), at(0.0, 6.5), 1.2, color);   // needle
    }
}

fn hex_string(r: u8, g: u8, b: u8, a: u8) -> String {
    if a == 255 {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}

fn edit_char_ok(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, ',' | ' ' | '.' | '#' | '%' | '-')
}

/// Parse typed/pasted text in the active mode. `#hex` works in every mode,
/// and a `name(...)` wrapper around the values is ignored.
fn parse_any(mode: ColorMode, s: &str) -> Option<((f32, f32, f32), Option<f32>)> {
    let t = s.trim();
    if !t.is_ascii() || t.len() > 96 {
        return None;
    }
    let t = t.rsplit_once('(').map_or(t, |x| x.1).trim_end_matches(')').trim();
    if t.starts_with('#') { parse_hex(t) } else { parse_mode(mode, t) }
}

fn apply_text(mode: ColorMode, s: &str, p: &mut Picker) -> bool {
    match parse_any(mode, s) {
        Some((rgb, a)) => {
            p.sync_rgb_f(rgb.0, rgb.1, rgb.2);
            if let Some(a) = a {
                p.alpha = a;
            }
            true
        }
        None => false,
    }
}

/// Half the thickness of a slider thumb along its track.
const GRIP_HALF: f32 = 2.75;

/// How far one mouse-wheel notch moves a channel slider (in 0..1).
fn chan_step(mode: ColorMode, label: &str) -> f32 {
    match (mode, label) {
        (_, "A") | (ColorMode::Rgba | ColorMode::Hex | ColorMode::Gray, _) => 1.0 / 255.0,
        (_, "H") => 1.0 / 360.0,
        _ => 0.01,
    }
}

fn draw_slider(ui: &Ui, bar: Rect, label_x: f32, value_x: f32, ch: &ChanUi, grad: &Texture2D, hot: bool) {
    let cy = bar.y + bar.h * 0.5;
    ui.label(ch.label, label_x, cy, 11.5, false, TEXT_DIM, 0.0);
    ui.strip(bar, grad, if ch.label == "A" { 4.0 } else { 0.0 }, WHITE);
    ui.marker(vec2(bar.x + GRIP_HALF + clamp01(ch.t) * (bar.w - GRIP_HALF * 2.0), cy), GRIP_HALF * 2.0, bar.h + if hot { 10.0 } else { 8.0 }, ch.handle);
    let value_box = Rect::new(value_x - 54.0, cy - 10.0, 54.0, 20.0);
    ui.field(value_box, SURFACE, STROKE);
    // the unit sits dimmed after the number, as in Photon's value boxes
    let (number, unit) = match ch.value.strip_suffix('%') {
        Some(n) => (n, "%"),
        None => match ch.value.strip_suffix("deg") {
            Some(n) => (n, if ui.font.is_some() { "\u{b0}" } else { "d" }),
            None => (ch.value.as_str(), ""),
        },
    };
    let unit_w = if unit.is_empty() { 0.0 } else { ui.width(unit, 11.0, false) + 2.0 };
    ui.label(unit, value_x - 6.0, cy, 11.0, false, TEXT_DIM, 1.0);
    ui.label(number, value_x - 6.0 - unit_w, cy, 11.0, false, TEXT, 1.0);
}

/// The CIE 1931 xy plot: grid and axes, the horseshoe `img` placed at `r`,
/// its outline with wavelength ticks, and the sRGB gamut with its white point.
fn draw_cie(ui: &Ui, r: Rect, img: &Texture2D) {
    let to_screen = |x: f64, y: f64| vec2(
        r.x + ((x - LAB_IMG_X_MIN) / (LAB_IMG_X_MAX - LAB_IMG_X_MIN)) as f32 * r.w,
        r.y + (1.0 - (y - LAB_IMG_Y_MIN) / (LAB_IMG_Y_MAX - LAB_IMG_Y_MIN)) as f32 * r.h,
    );
    let top_left = to_screen(CIE_X0, CIE_Y1);
    let bottom_right = to_screen(CIE_X1, 0.0);
    let frame = Rect::new(top_left.x, top_left.y, bottom_right.x - top_left.x, bottom_right.y - top_left.y);

    // grid every 0.1, axes on the left and bottom
    let grid = Color::new(1.0, 1.0, 1.0, 0.055);
    let axis = Color::new(1.0, 1.0, 1.0, 0.22);
    for i in 0..=8 {
        let v = 0.1 * i as f64;
        if i <= 7 {
            let gx = to_screen(v, 0.0).x;
            ui.hairline(gx, frame.y, 0.0, frame.h, grid);
            ui.label(&format!("{:.1}", v), gx, frame.y + frame.h + 8.0, 8.5, false, TEXT_DIM, 0.5);
        }
        let gy = to_screen(0.0, v).y;
        ui.hairline(frame.x, gy, frame.w, 0.0, grid);
        ui.label(&format!("{:.1}", v), frame.x - 5.0, gy, 8.5, false, TEXT_DIM, 1.0);
    }
    ui.hairline(frame.x, frame.y, 0.0, frame.h, axis);
    ui.hairline(frame.x, frame.y + frame.h, frame.w, 0.0, axis);
    ui.label("y", frame.x + 6.0, frame.y + 6.0, 9.5, true, TEXT_DIM, 0.0);
    ui.label("x", frame.x + frame.w - 5.0, frame.y + frame.h - 8.0, 9.5, true, TEXT_DIM, 1.0);

    ui.tex(r, 0.0, img);

    // spectral locus + line of purples
    let pts: Vec<Vec2> = SPECTRAL_LOCUS
        .iter()
        .map(|p| to_screen(p.1, p.2))
        .collect();
    let outline = Color::new(1.0, 1.0, 1.0, 0.9);
    for i in 0..pts.len() {
        ui.line(pts[i], pts[(i + 1) % pts.len()], 1.1, outline);
    }

    // wavelength ticks, pointing away from the white point
    let white = to_screen(0.3127, 0.3290);
    for nm in [470.0f64, 480.0, 490.0, 500.0, 510.0, 520.0, 540.0, 560.0, 580.0, 600.0, 620.0] {
        let p = match SPECTRAL_LOCUS.iter().find(|p| p.0 == nm) {
            Some(p) => p,
            None => continue,
        };
        let sp = to_screen(p.1, p.2);
        let out = (sp - white).normalize_or_zero();
        ui.line(sp, sp + out * 3.5, 1.0, outline);
        ui.label(&format!("{}", nm as i32), sp.x + out.x * 12.0, sp.y + out.y * 9.5, 8.0, false,
                 Color::from_rgba(196, 196, 201, 255), 0.5);
    }

    // sRGB gamut and its D65 white point
    let gamut = [to_screen(0.64, 0.33), to_screen(0.30, 0.60), to_screen(0.15, 0.06)];
    let edge = Color::new(0.0, 0.0, 0.0, 0.5);
    for i in 0..3 {
        ui.line(gamut[i], gamut[(i + 1) % 3], 1.0, edge);
    }
    ui.label("sRGB", gamut[1].x + 7.0, gamut[1].y + 15.0, 8.0, true, Color::new(0.0, 0.0, 0.0, 0.6), 0.5);
    ui.ring(white, 2.5, 1.0, Color::new(0.0, 0.0, 0.0, 0.6));
}

/// Munsell ring dressing: family ticks and names around it, the notation in the middle.
fn draw_munsell_reference(ui: &Ui, sv: Rect, hvc: (f32, f32, f32)) {
    let center = vec2(sv.x + sv.w * 0.5, sv.y + sv.h * 0.5);
    let half = sv.w * 0.5;
    let dir = |hue: f32| {
        let a = (hue * 3.6).to_radians();
        vec2(a.sin(), -a.cos())
    };
    for k in 0..10 {
        let edge = dir(k as f32 * 10.0);
        ui.line(center + edge * half * (MUNSELL_R_OUT + 0.012), center + edge * half * (MUNSELL_R_OUT + 0.05),
                1.0, Color::new(1.0, 1.0, 1.0, 0.35));
        let mid = center + dir(k as f32 * 10.0 + 5.0) * half * (MUNSELL_R_OUT + 0.085);
        ui.label(MUNSELL_FAMILIES[k], mid.x, mid.y, 8.5, true, TEXT_DIM, 0.5);
    }
    let (hue, value_chroma) = munsell_notation(hvc.0, hvc.1, hvc.2);
    ui.label(&hue, center.x, center.y - 7.0, 17.0, true, TEXT, 0.5);
    ui.label(&value_chroma, center.x, center.y + 11.0, 11.0, false, TEXT_DIM, 0.5);
}

/// Percent grid for the CMYK and HWB planes: a line and a number every 25%
/// on both axes. `color_at(u, v)` is the plane's color at a point (u across,
/// v down, both 0..1), used to pick black or white ink for each label.
fn draw_percent_grid(
    ui: &Ui, sv: Rect, x_name: &str, y_name: &str, y_grows_down: bool,
    color_at: impl Fn(f32, f32) -> (f32, f32, f32),
) {
    let ink = |u: f32, v: f32, strong: bool| {
        let (r, g, b) = color_at(u, v);
        let a = if strong { 1.0 } else { 0.8 };
        if 0.299 * r + 0.587 * g + 0.114 * b > 0.55 {
            Color::new(0.0, 0.0, 0.0, a * 0.85)
        } else {
            Color::new(1.0, 1.0, 1.0, a)
        }
    };
    let grid = Color::new(0.5, 0.5, 0.5, 0.3);
    let (edge_u, edge_v) = (6.0 / sv.w, 8.0 / sv.h); // where the labels sit, as plane fractions
    for k in 1..4 {
        let t = k as f32 * 0.25;
        let x = sv.x + t * sv.w;
        let y = sv.y + t * sv.h;
        ui.hairline(x, sv.y, 0.0, sv.h, grid);
        ui.hairline(sv.x, y, sv.w, 0.0, grid);
        ui.label(&format!("{}", k * 25), x, sv.y + sv.h - 8.0, 8.0, false, ink(t, 1.0 - edge_v, false), 0.5);
        let y_value = if y_grows_down { k * 25 } else { 100 - k * 25 };
        ui.label(&format!("{}", y_value), sv.x + 6.0, y - 7.0, 8.0, false, ink(edge_u, t - edge_v, false), 0.0);
    }
    ui.label(x_name, sv.x + sv.w - 7.0, sv.y + sv.h - 9.0, 10.0, true, ink(1.0 - edge_u, 1.0 - edge_v, true), 1.0);
    ui.label(y_name, sv.x + 7.0, sv.y + 10.0, 10.0, true, ink(edge_u, edge_v, true), 0.0);
}

/// Axes for the OKLCH plane: chroma along the bottom, hue up the side.
fn draw_oklch_reference(ui: &Ui, sv: Rect) {
    let grid = Color::new(1.0, 1.0, 1.0, 0.08);
    let text = Color::new(1.0, 1.0, 1.0, 0.82);
    for k in 1..=3 {
        let x = sv.x + (k as f32 * 0.1 / OK_CHROMA_MAX) * sv.w;
        ui.hairline(x, sv.y, 0.0, sv.h, grid);
        ui.label(&format!("{:.1}", k as f32 * 0.1), x, sv.y + sv.h - 8.0, 8.0, false, text, 0.5);
    }
    for k in 1..6 {
        let y = sv.y + (1.0 - k as f32 / 6.0) * sv.h;
        ui.hairline(sv.x, y, sv.w, 0.0, grid);
        let deg = if ui.font.is_some() { "\u{b0}" } else { "" };
        ui.label(&format!("{}{}", k * 60, deg), sv.x + 6.0, y - 7.0, 8.0, false, text, 0.0);
    }
    ui.label("C", sv.x + sv.w - 7.0, sv.y + sv.h - 9.0, 10.0, true, WHITE, 1.0);
    ui.label("H", sv.x + 7.0, sv.y + 10.0, 10.0, true, WHITE, 0.0);
}

/// Axes for the YUV / YPbPr planes: a faint grid every 0.1, the two zero
/// axes with ticks, and a number every 0.2.
fn draw_uv_reference(ui: &Ui, sv: Rect, u_range: f32, v_range: f32, u_label: &str, v_label: &str) {
    let cx = sv.x + sv.w * 0.5;
    let cy = sv.y + sv.h * 0.5;
    let grid = Color::new(1.0, 1.0, 1.0, 0.07);
    let axis = Color::new(1.0, 1.0, 1.0, 0.6);
    let text = Color::new(1.0, 1.0, 1.0, 0.82);

    let step_x = sv.w * 0.05 / u_range;
    let step_y = sv.h * 0.05 / v_range;
    for k in 1..=(u_range / 0.1) as i32 {
        for side in [-1.0f32, 1.0] {
            let x = cx + side * k as f32 * step_x;
            ui.hairline(x, sv.y, 0.0, sv.h, grid);
            ui.hairline(x, cy - 2.5, 0.0, 5.0, axis);
            if k % 2 == 0 {
                ui.label(&format!("{:+.1}", side * k as f32 * 0.1), x, cy + 9.0, 8.0, false, text, 0.5);
            }
        }
    }
    for k in 1..=(v_range / 0.1) as i32 {
        for side in [-1.0f32, 1.0] {
            let y = cy - side * k as f32 * step_y;
            ui.hairline(sv.x, y, sv.w, 0.0, grid);
            ui.hairline(cx - 2.5, y, 5.0, 0.0, axis);
            if k % 2 == 0 {
                ui.label(&format!("{:+.1}", side * k as f32 * 0.1), cx + 6.0, y, 8.0, false, text, 0.0);
            }
        }
    }
    ui.hairline(sv.x, cy, sv.w, 0.0, axis);
    ui.hairline(cx, sv.y, 0.0, sv.h, axis);
    ui.label(u_label, sv.x + sv.w - 7.0, cy - 9.0, 10.0, true, WHITE, 1.0);
    ui.label(v_label, cx - 7.0, sv.y + 10.0, 10.0, true, WHITE, 1.0);
}

#[macroquad::main(conf)]
async fn main() {

    let mut ui = Ui::new();

    let mut picker = Picker::new();
    let mut mode = ColorMode::Rgba;
    let mut on_top = false;
    if let Some(st) = load_state() {
        mode = st.mode;
        on_top = st.on_top;
        DIM_PLANES.store(st.dim, std::sync::atomic::Ordering::Relaxed);
        picker.sync_hsv(st.rgba.0, st.rgba.1, st.rgba.2);
        picker.alpha = st.rgba.3 as f32 / 255.0;
    }
    let mut persisted = Saved { mode, rgba: (0, 0, 0, 0), on_top, dim: dim_planes() };
    let mut top_applied = !on_top;

    let mut active: Option<Widget> = None;
    let mut picking = false;
    let mut saved_picker: Option<Picker> = None;
    let mut last_nudge: f64 = 0.0;

    let mut editing = false;
    let mut edit_buf = String::new();
    // right after opening, the whole value is "selected": typing replaces it
    let mut edit_fresh = false;
    let mut next_backspace: f64 = 0.0;
    let mut bad_at: f64 = f64::NEG_INFINITY;
    // which copy button shows its checkmark (1 = values, 2 = hex), and since when
    let mut copied: (u8, f64) = (0, f64::NEG_INFINITY);

    let mut dd_open = false;
    // the color the "current" swatch holds: the one at launch, then the last one copied
    let mut kept = { let (r, g, b) = picker.rgb(); (r, g, b, q8(picker.alpha)) };
    let mut tip_text: &'static str = "";
    let mut tip_since: f64 = 0.0;
    let mut cursor_now = CursorIcon::Default;

    let mut swatches = load_swatches();
    // scroll position of the swatch row, in swatches: where it is, and where it is gliding to
    let mut sw_pos: f32 = 0.0;
    let mut sw_goal: f32 = 0.0;

    let mut styled = screen::apply_win11_style(TITLE_CAPTION_BGR);
    let mut framed = false;
    let mut iconed = screen::set_window_icons();
    // the CIE diagram is slow to rasterize, so it's built the first time a mode shows it
    let mut lab_img: Option<Texture2D> = None;
    let icon_tex = {
        let img = image::load_from_memory(include_bytes!("../assets/icon.png"))
            .expect("Failed to load icon")
            .to_rgba8();
        let img = image::imageops::resize(&img, 40, 40, image::imageops::FilterType::Lanczos3);
        let t = Texture2D::from_rgba8(40, 40, &img.into_raw());
        t.set_filter(FilterMode::Linear);
        t
    };

    let hue_tex = hue_strip_texture();
    let gray_tex = gray_strip_texture();
    let fade_tex = fade_strip_texture();
    let sat_tex = sat_overlay_texture(260);
    let val_tex = val_overlay_texture(260);
    let hsl_sq = Texture2D::from_rgba8(260, 260, &hsl_square_bytes(picker.hue));
    hsl_sq.set_filter(FilterMode::Linear);
    let mut hsl_sq_hue = picker.hue;
    let gray_sq = Texture2D::from_rgba8(260, 260, &gray_square_bytes());
    gray_sq.set_filter(FilterMode::Linear);

    let plane_sq = Texture2D::from_rgba8(260, 260, &vec![0u8; 260 * 260 * 4]);
    plane_sq.set_filter(FilterMode::Linear);
    let mut plane_key = (u8::MAX, f32::MAX, f32::MAX);
    let mut plane_pos_mode = ColorMode::Rgba;
    let mut plane_pos_key = (u8::MAX, u8::MAX, u8::MAX, u8::MAX);
    let chan_w = 144.0 as u16;
    let chan_tex: [Texture2D; 4] = [
        Texture2D::from_rgba8(chan_w, 1, &vec![0u8; chan_w as usize * 4]),
        Texture2D::from_rgba8(chan_w, 1, &vec![0u8; chan_w as usize * 4]),
        Texture2D::from_rgba8(chan_w, 1, &vec![0u8; chan_w as usize * 4]),
        Texture2D::from_rgba8(chan_w, 1, &vec![0u8; chan_w as usize * 4]),
    ];
    for t in &chan_tex {
        t.set_filter(FilterMode::Linear);
    }
    let mut chans = mode_sliders(mode, &picker);
    let mut last_key = [i64::MIN; 39];
    screen::set_high_res_timer();
    let refresh_hz = screen::monitor_refresh_hz().max(30) as f64;
    loop {
        let frame_start = get_time();
        let now = frame_start;
        clear_background(BG);

        if !styled {
            styled = screen::apply_win11_style(TITLE_CAPTION_BGR);
        }
        if !iconed {
            iconed = screen::set_window_icons();
        }
        let (sw, sh) = (screen_width(), screen_height());
        let scale = (sw / CONTENT_W).min(sh / (TITLE_H + CONTENT_H)).max(0.05);
        // title bar metrics in logical screen px, for native drag/resize hit-testing
        let cap_px = (TITLE_H * scale) as i32;
        let zone_px = (TITLE_BTN_W * 5.0 * scale) as i32;
        if !framed {
            framed = screen::enable_custom_frame(cap_px, sh as i32, zone_px);
            if framed {
                // the frame is all client area now: size it to the design canvas exactly
                let dpi = screen::dpi_scale();
                screen::set_window_px((WINDOW_W * dpi).round() as i32, (WINDOW_H * dpi).round() as i32);
            }
        }
        if framed && !top_applied {
            screen::set_topmost(on_top);
            top_applied = true;
        }
        screen::frame_tick(cap_px, sh as i32, zone_px);
        ui.begin(scale);

        // window size in design units; the content block is centered below the title bar
        let (vw, vh) = (sw / scale, sh / scale);
        let o = vec2(
            ui.snap((vw - CONTENT_W) * 0.5),
            ui.snap(TITLE_H + (vh - TITLE_H - CONTENT_H) * 0.5),
        );
        let at = |x: f32, y: f32, w: f32, h: f32| Rect::new(o.x + x, o.y + y, w, h);

        let (mx, my) = mouse_position();
        let m = if screen::mouse_in_window()
            || is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Right)
        {
            Vec2::new(mx / scale, my / scale)
        } else {
            Vec2::new(-1.0e6, -1.0e6)
        };
        let pressed = is_mouse_button_pressed(MouseButton::Left);
        let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
            || is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper); // Cmd on macOS
        let mut cursor = CursorIcon::Default;
        let mut tip: Option<(&'static str, Rect)> = None;

        // ----- layout -----
        let sv = at(14.0, 14.0, 232.0, 232.0);
        let lab_rect = cie_rect(sv);
        let hue_bar = at(257.0, 14.0, 18.0, 232.0);
        let alpha_bar = at(284.0, 14.0, 18.0, 232.0);
        let swatch_new = at(314.0, 14.0, 84.0, 36.0);
        let swatch_kept = at(314.0, 50.0, 84.0, 36.0);
        let pick_btn = at(410.0, 14.0, 96.0, 30.0);
        let mode_btn = at(410.0, 56.0, 96.0, 30.0);
        let bars: [Rect; 4] = std::array::from_fn(|i| at(334.0, 113.0 + i as f32 * 25.0, 112.0, 12.0));
        let val_field = at(14.0, 256.0, 283.0, 24.0);
        let sw_row = at(314.0, 215.0, 192.0, 20.0);
        let sw_track = at(314.0, 240.0, 192.0, 5.0);
        let sw_thumb_w = 26.0;
        let sw_max = (SWATCH_SLOTS - SWATCH_SHOWN) as f32;
        sw_pos += (sw_goal - sw_pos) * (1.0 - (-get_frame_time() * 16.0).exp());
        if (sw_goal - sw_pos).abs() < 0.002 {
            sw_pos = sw_goal;
        }
        let hex_field = at(314.0, 256.0, 192.0, 24.0);
        // pointer position along a vertical bar / a channel slider, as 0..1
        let bar_t = |bar: Rect| clamp01((m.y - bar.y - GRIP_HALF) / (bar.h - GRIP_HALF * 2.0));
        let chan_t = |bar: Rect| clamp01((m.x - bar.x - GRIP_HALF) / (bar.w - GRIP_HALF * 2.0));

        // ------- screen-pick mode ----------(alr)
        let was_picking = picking;
        if picking && screen::SYSTEM_SAMPLER {
            // the OS sampler runs the pick on its own; poll for its answer
            match screen::sampler_result() {
                Some(Some((pr, pg, pb))) => {
                    picker.sync_hsv(pr, pg, pb);
                    clipboard_set(&format!("#{:02X}{:02X}{:02X}", pr, pg, pb));
                    copied = (2, now);
                    picking = false;
                }
                Some(None) => {
                    if let Some(p) = saved_picker {
                        picker = p;
                    }
                    picking = false;
                }
                None => {}
            }
        } else if picking {
            let (mut gx, mut gy) = screen::cursor();
            if now - last_nudge > 0.05 {
                let (nx, ny) = (gx, gy);
                if is_key_down(KeyCode::Left) { gx -= 1; }
                if is_key_down(KeyCode::Right) { gx += 1; }
                if is_key_down(KeyCode::Up) { gy -= 1; }
                if is_key_down(KeyCode::Down) { gy += 1; }
                if (gx, gy) != (nx, ny) {
                    screen::set_cursor(gx, gy);
                    last_nudge = now;
                }
            }

            let px = screen::region(gx, gy, screen::CAPTURE_N, screen::CAPTURE_N);
            let c = ((screen::CAPTURE_N as usize / 2) * screen::CAPTURE_N as usize
                + screen::CAPTURE_N as usize / 2) * 4;
            picker.sync_hsv(px[c], px[c + 1], px[c + 2]);

            screen::overlay_update(&px, px[c], px[c + 1], px[c + 2], gx, gy);

            if let Some((cx, cy)) = screen::take_click() {
                let p = screen::region(cx, cy, 1, 1);
                picker.sync_hsv(p[0], p[1], p[2]);
                let (pr, pg, pb) = picker.rgb();
                clipboard_set(&format!("#{:02X}{:02X}{:02X}", pr, pg, pb));
                copied = (2, now);
                picking = false;
                screen::stop_pick();
                screen::overlay_hide();
            } else if is_key_pressed(KeyCode::Escape) {
                if let Some(p) = saved_picker {
                    picker = p;
                }
                picking = false;
                screen::stop_pick();
                screen::overlay_hide();
            }
        }
        let start_pick = |picker: &Picker, saved: &mut Option<Picker>| {
            *saved = Some(*picker);
            screen::start_pick();
            screen::overlay_show();
        };

        // -------- text edit mode ----------
        if editing {
            while let Some(ch) = get_char_pressed() {
                if edit_char_ok(ch) && edit_buf.len() < 48 {
                    if edit_fresh {
                        edit_buf.clear();
                        edit_fresh = false;
                    }
                    edit_buf.push(ch);
                }
            }
            if ctrl && is_key_pressed(KeyCode::A) {
                edit_fresh = true;
            }
            if ctrl && is_key_pressed(KeyCode::V) {
                if let Some(clip) = clipboard_get() {
                    let clip = clip.trim();
                    let inner = clip.rsplit_once('(').map_or(clip, |x| x.1);
                    let s: String = inner.chars().filter(|c| edit_char_ok(*c)).take(48).collect();
                    if !s.is_empty() {
                        edit_buf = s;
                        edit_fresh = false;
                    }
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                if edit_fresh { edit_buf.clear(); } else { edit_buf.pop(); }
                edit_fresh = false;
                next_backspace = now + 0.4; // pause before auto-repeat
            } else if is_key_down(KeyCode::Backspace) && now > next_backspace {
                edit_buf.pop();
                next_backspace = now + 0.04;
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                if apply_text(mode, &edit_buf, &mut picker) { editing = false; } else { bad_at = now; }
            } else if is_key_pressed(KeyCode::Escape) {
                editing = false;
            } else if pressed && !hit(val_field, m, 0.0) {
                // clicking away commits a valid value and drops an invalid one
                apply_text(mode, &edit_buf, &mut picker);
                editing = false;
            }
        } else {
            while get_char_pressed().is_some() {} // don't let stray typing queue up
            if !was_picking {
                if ctrl && is_key_pressed(KeyCode::C) {
                    clipboard_set(&mode_values(mode, &picker));
                    copied = (1, now);
                } else if ctrl && is_key_pressed(KeyCode::V) {
                    let ok = clipboard_get().is_some_and(|clip| apply_text(mode, &clip, &mut picker));
                    if !ok { bad_at = now; }
                } else if !ctrl && screen::CAN_PICK && is_key_pressed(KeyCode::P) {
                    dd_open = false;
                    picking = true;
                    start_pick(&picker, &mut saved_picker);
                }
            }
        }

        // nothing below reacts while the screen picker or the mode menu is up
        let free = !was_picking && !picking && !dd_open && !editing;

        if free && pressed && active.is_none() {
            active = if hit(sv, m, 0.0) { Some(Widget::SatVal)
            } else if hit(hue_bar, m, 8.0) { Some(Widget::Hue)
            } else if hit(alpha_bar, m, 8.0) { Some(Widget::Alpha)
            } else if hit(sw_track, m, 4.0) { Some(Widget::SwatchScroll)
            } else {
                (0..4).find(|i| !chans[*i].label.is_empty() && hit(bars[*i], m, 8.0)).map(Widget::Chan)
            };
        }

        // mouse wheel: fine-step whichever slider is under the cursor
        let wheel = mouse_wheel().1;
        if wheel != 0.0 && free && active.is_none() {
            let dir = wheel.signum();
            if hit(hue_bar, m, 4.0) {
                if mode == ColorMode::Gray {
                    let (r, g, b) = picker.rgb_f32();
                    let gv = clamp01((luma8(r, g, b) as f32 + dir) / 255.0);
                    picker.sync_rgb_f(gv, gv, gv);
                } else {
                    picker.hue = (picker.hue + dir).clamp(0.0, 359.9999);
                    let (r, g, b) = picker.rgb_f32();
                    picker.keep_hsv = true;
                    picker.sync_rgb_f(r, g, b);
                }
            } else if hit(alpha_bar, m, 4.0) {
                picker.alpha = clamp01(picker.alpha + dir / 255.0);
            } else if hit(sw_row, m, 3.0) || hit(sw_track, m, 4.0) {
                // wheel down moves further along the saved colors
                sw_goal = (sw_goal - dir * 2.0).clamp(0.0, sw_max);
            } else if let Some(i) = (0..4).find(|i| !chans[*i].label.is_empty() && hit(bars[*i], m, 8.0)) {
                let t = clamp01(chans[i].t + dir * chan_step(mode, chans[i].label));
                set_channel(mode, i, t, &mut picker);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            match active {
                Some(Widget::SatVal) => {
                    if mode == ColorMode::Gray {
                        let gv = clamp01((m.x - sv.x) / sv.w);
                        picker.sync_rgb_f(gv, gv, gv); } else if mode == ColorMode::Lab {

                        let u = clamp01((m.x - lab_rect.x) / lab_rect.w);
                        let v = 1.0 - clamp01((m.y - lab_rect.y) / lab_rect.h);
                        let (cxx, cyy) = clamp_to_locus(lab_img_x(u), lab_img_y(v));
                        let (dr, dg, db) = diagram_rgb(cxx, cyy);

                        if (dr, dg, db) != (0, 0, 0) {
                            picker.keep_cie = true;
                            picker.sync_rgb_f(
                                dr as f32 / 255.0,
                                dg as f32 / 255.0,
                                db as f32 / 255.0,
                            );
                            picker.cie_xy = (cxx, cyy);
                        }
                    } else if mode == ColorMode::Xyz || mode == ColorMode::Xyy {

                        let u = clamp01((m.x - lab_rect.x) / lab_rect.w);
                        let v = 1.0 - clamp01((m.y - lab_rect.y) / lab_rect.h);
                        let (cxx, cyy) = clamp_to_locus(lab_img_x(u), lab_img_y(v));
                        if cyy > 1e-3 {
                            let big_y = rgb_to_xyz(picker.rgb_f32().0,
                                                   picker.rgb_f32().1,
                                                   picker.rgb_f32().2).1;
                            let (x, y, z) = xyy_to_xyz(cxx, cyy, big_y);
                            let rgb = xyz_to_rgb_f(x, y, z);
                            picker.keep_cie = true;
                            picker.keep_xyz = true;
                            picker.keep_xyy = true;
                            picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                            picker.cie_xy = (cxx, cyy);
                            picker.xyz = (x, y, z);
                            picker.xyy = (cxx, cyy, big_y);
                        }
                    } else if mode == ColorMode::Cmyk {
                        let c = clamp01((m.x - sv.x) / sv.w);
                        let mm = clamp01(1.0 - (m.y - sv.y) / sv.h);
                        let y_cur = picker.cmyk.2;
                        let k_cur = picker.cmyk.3;
                        let rgb = cmyk_to_rgb(c, mm, y_cur, k_cur);
                        picker.keep_plane = true;
                        picker.plane_pos = (c, mm);
                        picker.keep_cmyk = true;
                        picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                        picker.cmyk = (c, mm, y_cur, k_cur);
                    } else if mode == ColorMode::Hwb {

                        let w = clamp01((m.x - sv.x) / sv.w);
                        let bl = clamp01((m.y - sv.y) / sv.h);
                        let rgb = hwb_to_rgb(picker.hue, w, bl);
                        picker.keep_plane = true;
                        picker.plane_pos = (w, 1.0 - bl);
                        picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                    } else if mode == ColorMode::Oklch {

                        let l = picker.oklch.0;
                        let c = clamp01((m.x - sv.x) / sv.w) * OK_CHROMA_MAX;
                        let h = (1.0 - clamp01((m.y - sv.y) / sv.h)) * 360.0;
                        picker.keep_oklch = true;
                        let rgb = oklch_to_rgb(l as f64, c as f64, h as f64);
                        picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                        picker.oklch = (l, c, h);
                        picker.keep_plane = true;
                        picker.plane_pos = ((c / OK_CHROMA_MAX).clamp(0.0, 1.0),
                                            (h / 360.0).clamp(0.0, 1.0));
                    } else if mode == ColorMode::Yuv {

                        let y = picker.yuv.0;
                        let u = clamp01((m.x - sv.x) / sv.w) * 0.872 - 0.436;
                        let v = (1.0 - clamp01((m.y - sv.y) / sv.h)) * 1.23 - 0.615;
                        let rgb = yuv_to_rgb(y, u, v);
                        picker.keep_plane = true;
                        picker.keep_yuv = true; // u,v are the dragged point
                        picker.plane_pos = (clamp01((m.x - sv.x) / sv.w),
                                            clamp01(1.0 - (m.y - sv.y) / sv.h));
                        picker.yuv.1 = u;
                        picker.yuv.2 = v;
                        picker.sync_rgb_f(rgb.0.clamp(0.0, 1.0), rgb.1.clamp(0.0, 1.0),
                                          rgb.2.clamp(0.0, 1.0));
                    } else if mode == ColorMode::Ypbpr {

                        let y = picker.ypbpr.0;
                        let pb = clamp01((m.x - sv.x) / sv.w) - 0.5;
                        let pr = (1.0 - clamp01((m.y - sv.y) / sv.h)) - 0.5;
                        let rgb = ypbpr_to_rgb(y, pb, pr);
                        picker.keep_plane = true;
                        picker.keep_ypbpr = true; // pb,pr are the dragged point
                        picker.plane_pos = (clamp01((m.x - sv.x) / sv.w),
                                            clamp01(1.0 - (m.y - sv.y) / sv.h));
                        picker.ypbpr.1 = pb;
                        picker.ypbpr.2 = pr;
                        picker.sync_rgb_f(rgb.0.clamp(0.0, 1.0), rgb.1.clamp(0.0, 1.0),
                                          rgb.2.clamp(0.0, 1.0));
                    } else if mode == ColorMode::Munsell {

                        let dx = m.x - (sv.x + sv.w * 0.5);
                        let dy = m.y - (sv.y + sv.h * 0.5);
                        let mh = dx.atan2(-dy).to_degrees().rem_euclid(360.0) / 3.6;
                        let (_, mv, mc) = picker.munsell;
                        let rgb = munsell_to_rgb(mh, mv, mc);
                        picker.keep_munsell = true;
                        picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                        picker.munsell = (mh, mv, mc);
                    } else if matches!(mode, ColorMode::Hsl | ColorMode::Hsla) {
                        let s = clamp01((m.x - sv.x) / sv.w);
                        let l = clamp01(1.0 - (m.y - sv.y) / sv.h);
                        picker.hsl_s = s;
                        let rgb = hsl_to_rgb(picker.hue, s, l);
                        picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                    } else {
                        picker.sat = clamp01((m.x - sv.x) / sv.w);
                        picker.val = clamp01(1.0 - (m.y - sv.y) / sv.h);
                        let (r, g, b) = picker.rgb_f32();
                        picker.keep_hsv = true;
                        picker.sync_rgb_f(r, g, b);
                    }
                }
                Some(Widget::Hue) if mode == ColorMode::Gray => {

                    let gv = 1.0 - bar_t(hue_bar);
                    picker.sync_rgb_f(gv, gv, gv);
                }
                Some(Widget::Hue) => {
                    picker.hue = hue_from_t(1.0 - bar_t(hue_bar));

                    let (r, g, b) = picker.rgb_f32();
                    picker.keep_hsv = true;
                    picker.sync_rgb_f(r, g, b);
                }
                Some(Widget::Alpha) => {
                    picker.alpha = 1.0 - bar_t(alpha_bar)
                }
                Some(Widget::Chan(i)) => {
                    set_channel(mode, i, chan_t(bars[i]), &mut picker);
                }
                Some(Widget::SwatchScroll) => {
                    let t = clamp01((m.x - sw_track.x - sw_thumb_w * 0.5) / (sw_track.w - sw_thumb_w));
                    sw_goal = t * sw_max;
                    sw_pos = sw_goal; // the bar follows the cursor exactly
                }
                None => {}
            }
        } else {
            active = None;
        }

        let (r, g, b) = picker.rgb();
        let (rf, gf, bf) = picker.rgb_f32();
        let a8 = q8(picker.alpha);
        let opaque = Color::from_rgba(r, g, b, 255);

        // --------- rebuild slider UI color OR alpha changed not again... ----------
        let k10 = |v:f32| (v * 10.0) as i64;
        let k10k = |v:f32| (v * 10_000.0) as i64;
        let key: [i64; 39] = [
            mode as i64, r as i64, g as i64, b as i64, a8 as i64,
            k10(picker.hue), k10k(picker.sat), k10k(picker.val),
            k10(picker.hsl.0), k10k(picker.hsl.1), k10k(picker.hsl.2),
            k10k(picker.cmyk.0), k10k(picker.cmyk.1), k10k(picker.cmyk.2), k10k(picker.cmyk.3),
            k10(picker.hwb.0), k10k(picker.hwb.1), k10k(picker.hwb.2),
            (picker.lab.0 * 10.0) as i64, (picker.lab.1 * 10.0) as i64, (picker.lab.2 * 10.0) as i64,
            (picker.oklch.0 * 1000.0) as i64, (picker.oklch.1 * 1000.0) as i64, k10(picker.oklch.2),
            k10k(picker.yuv.0), k10k(picker.yuv.1), k10k(picker.yuv.2),
            k10k(picker.ypbpr.0), k10k(picker.ypbpr.1), k10k(picker.ypbpr.2),
            (picker.xyz.0 * 10_000.0) as i64, (picker.xyz.1 * 10_000.0) as i64,
            (picker.xyz.2 * 10_000.0) as i64,
            (picker.xyy.0 * 10_000.0) as i64, (picker.xyy.1 * 10_000.0) as i64,
            (picker.xyy.2 * 10_000.0) as i64,
            k10(picker.munsell.0), (picker.munsell.1 * 1000.0) as i64,
            (picker.munsell.2 * 1000.0) as i64,
        ];
        if last_key != key {
            last_key = key;
            chans = mode_sliders(mode, &picker);
            for i in 0..4 {
                chan_tex[i].update(&Image { width: chan_w, height: 1, bytes: chans[i].bytes.clone() });
            }
        }

        let (hr, hg, hb) = hsv_to_rgb(picker.hue, 1.0, 1.0);
        let hsl_mode = matches!(mode, ColorMode::Hsl | ColorMode::Hsla);
        let plane_mode = matches!(mode, ColorMode::Cmyk | ColorMode::Hwb | ColorMode::Oklch
            | ColorMode::Yuv | ColorMode::Ypbpr | ColorMode::Munsell);
        let diagram_mode = matches!(mode, ColorMode::Lab | ColorMode::Xyz | ColorMode::Xyy);
        if (picker.hue - hsl_sq_hue).abs() > 0.25 {
            hsl_sq_hue = picker.hue;
            hsl_sq.update(&Image { width: 260, height: 260,
                bytes: hsl_square_bytes(picker.hue) });
        }
        let plane_seed: Option<(u8, f32, f32)> = match mode {
            ColorMode::Cmyk => Some((0, picker.cmyk.2, picker.cmyk.3)),
            ColorMode::Hwb => Some((1, picker.hue / 360.0, 0.0)),
            ColorMode::Oklch => Some((2, picker.oklch.0, 0.0)),
            ColorMode::Yuv => Some((3, picker.yuv.0, 0.0)),
            ColorMode::Ypbpr => Some((4, picker.ypbpr.0, 0.0)),
            ColorMode::Munsell => {
                let (_, mv, mc) = picker.munsell;
                Some((5, mv / 10.0, mc / MUNSELL_C_MAX))
            }
            _ => None,
        };

        let plane_color_key = (r, g, b, a8);
        if picker.keep_plane {
            picker.keep_plane = false;
            plane_pos_key = plane_color_key;
        } else if plane_pos_mode != mode || plane_pos_key != plane_color_key {
            plane_pos_mode = mode;
            plane_pos_key = plane_color_key;
            picker.plane_pos = match mode {
                ColorMode::Cmyk => (picker.cmyk.0, picker.cmyk.1),
                ColorMode::Hwb => {
                    let (_, w4, b4) = picker.hwb;(clamp01(w4), clamp01(1.0 - b4))
                }
                ColorMode::Oklch => (
                    clamp01(picker.oklch.1 / OK_CHROMA_MAX),
                    clamp01(picker.oklch.2 / 360.0),
                ),
                ColorMode::Yuv => {
                    let (_, u, v) = picker.yuv;
                    (clamp01((u + 0.436) / 0.872), clamp01((v + 0.615) / 1.23))
                }
                ColorMode::Ypbpr => {
                    let (_, pb, pr) = picker.ypbpr;
                    (clamp01(pb + 0.5), clamp01(pr + 0.5))
                }
                _ => picker.plane_pos,
            };
        }
        if let Some((kind, p0, p1)) = plane_seed {
            if plane_key.0 != kind
                || (plane_key.1 - p0).abs() + (plane_key.2 - p1).abs() > 0.004
            {
                plane_key = (kind, p0, p1);
                let bytes = match kind {
                    0 => cmyk_square_bytes(p0, p1),
                    1 => hwb_square_bytes(p0 * 360.0),
                    2 => oklch_square_bytes(p0),
                    3 => yuv_square_bytes(p0),
                    4 => ypbpr_square_bytes(p0),
                    _ => munsell_wheel_bytes(p0 * 10.0, p1 * MUNSELL_C_MAX),
                };
                plane_sq.update(&Image { width: 260, height: 260, bytes });
            }
        }

        // ---------- color plane ----------
        let plane_r = 6.0;
        if mode == ColorMode::Gray {
            ui.tex(sv, plane_r, &gray_sq);
        } else if diagram_mode {
            // LAB/xyZ/xyY: the cie 1931 chromaticity diagram + reference lines
            ui.fill(sv, plane_r, Color::from_rgba(21, 21, 20, 255));
            let img = lab_img.get_or_insert_with(|| {
                let t = Texture2D::from_rgba8(LAB_TEX_W, LAB_TEX_H, &lab_diagram_bytes(LAB_TEX_W, LAB_TEX_H));
                t.set_filter(FilterMode::Linear);
                t
            });
            draw_cie(&ui, lab_rect, img);
        } else if plane_mode {
            if mode == ColorMode::Munsell {
                ui.fill(sv, plane_r, Color::from_rgba(21, 21, 20, 255)); // well behind the ring
            }
            ui.tex(sv, plane_r, &plane_sq);
            if mode == ColorMode::Munsell {
                draw_munsell_reference(&ui, sv, picker.munsell);
            } else if mode == ColorMode::Oklch {
                draw_oklch_reference(&ui, sv);
            } else if mode == ColorMode::Cmyk {
                let (_, _, y, k) = picker.cmyk;
                draw_percent_grid(&ui, sv, "C", "M", false, |u, v| cmyk_to_rgb(u, 1.0 - v, y, k));
            } else if mode == ColorMode::Hwb {
                let hue = picker.hue;
                draw_percent_grid(&ui, sv, "W", "B", true, |u, v| {
                    if u + v > 1.0 && dim_planes() { (0.1, 0.1, 0.1) } else { hwb_to_rgb(hue, u, v) }
                });
            } else if mode == ColorMode::Yuv {
                draw_uv_reference(&ui, sv, 0.436, 0.615, "U", "V");
            } else if mode == ColorMode::Ypbpr {
                draw_uv_reference(&ui, sv, 0.5, 0.5, "Pb", "Pr");
            }
        } else if hsl_mode {
            ui.tex(sv, plane_r, &hsl_sq);
        } else {
            ui.fill(sv, plane_r, Color::from_rgba(hr, hg, hb, 255));
            ui.tex(sv, plane_r, &sat_tex);
            ui.tex(sv, plane_r, &val_tex);
        }
        ui.hair(sv, plane_r, STROKE);

        let (cx, cy) = if mode == ColorMode::Gray {
            (sv.x + (luma8(rf, gf, bf) as f32 / 255.0) * sv.w, sv.y + sv.h * 0.5)
        } else if diagram_mode {
            let (cxx, cyy) = picker.cie_xy;
            let u = lab_img_u(cxx);
            let v = lab_img_v(cyy);
            (lab_rect.x + u * lab_rect.w, lab_rect.y + (1.0 - v) * lab_rect.h)
        } else if mode == ColorMode::Munsell {
            let (mh, _, _) = picker.munsell;
            let a = (mh * 3.6f32).to_radians();
            let rr = sv.w * 0.5 * (MUNSELL_R_OUT + MUNSELL_R_IN) * 0.5;
            (sv.x + sv.w * 0.5 + a.sin() * rr, sv.y + sv.h * 0.5 - a.cos() * rr)
        } else if plane_mode {
            (sv.x + picker.plane_pos.0 * sv.w, sv.y + (1.0 - picker.plane_pos.1) * sv.h)
        } else if hsl_mode {
            let (_, s0, l) = rgb_to_hsl(rf, gf, bf);
            let s = if rf.max(gf).max(bf) - rf.min(gf).min(bf) < 0.003 {
                clamp01(picker.hsl_s)
            } else {
                s0
            };
            (sv.x + s * sv.w, sv.y + (1.0 - l) * sv.h)
        } else {
            (sv.x + picker.sat * sv.w, sv.y + (1.0 - picker.val) * sv.h)
        };
        let grab = |w: Widget, over: bool| {
            if active == Some(w) || (free && active.is_none() && over) { 1.0 } else { 0.0 }
        };
        ui.reticle(vec2(cx, cy), 5.0 + grab(Widget::SatVal, false));

        // ---------- hue + alpha bars ----------
        let bar_mark = |bar: Rect, t: f32, w: Widget, fill: Color| {
            let big = grab(w, hit(bar, m, 8.0));
            ui.marker(vec2(bar.x + bar.w * 0.5, bar.y + GRIP_HALF + t * (bar.h - GRIP_HALF * 2.0)),
                      bar.w + 8.0 + big * 2.0, GRIP_HALF * 2.0, fill);
        };
        if mode == ColorMode::Gray {
            ui.strip(hue_bar, &gray_tex, 0.0, WHITE);
            let gv = luma8(rf, gf, bf);
            bar_mark(hue_bar, 1.0 - gv as f32 / 255.0, Widget::Hue, Color::from_rgba(gv, gv, gv, 255));
        } else {
            ui.strip(hue_bar, &hue_tex, 0.0, WHITE);
            let (tr, tg, tb) = hsv_to_rgb(picker.hue, 0.8, 1.0); // the strip's own softened color
            bar_mark(hue_bar, 1.0 - picker.hue / 360.0, Widget::Hue, Color::from_rgba(tr, tg, tb, 255));
        }
        ui.strip(alpha_bar, &fade_tex, 4.0, opaque);
        bar_mark(alpha_bar, 1.0 - picker.alpha, Widget::Alpha, picker.color());

        // ---------- new / current ----------
        // each half is a rounded rect with its inner edge squared off by a second piece
        let kept_color = Color::from_rgba(kept.0, kept.1, kept.2, kept.3);
        ui.checker(swatch_new, 5.0, 6.0, picker.color());
        ui.checker(Rect::new(swatch_new.x, swatch_new.y + 12.0, swatch_new.w, swatch_new.h - 12.0), 0.0, 6.0, picker.color());
        ui.checker(swatch_kept, 5.0, 6.0, kept_color);
        ui.checker(Rect::new(swatch_kept.x, swatch_kept.y, swatch_kept.w, 12.0), 0.0, 6.0, kept_color);
        ui.hair(Rect::new(swatch_new.x, swatch_new.y, swatch_new.w, swatch_new.h * 2.0), 5.0, STROKE_HI);
        // names in whichever of black / white reads on each half (as seen over the checkerboard)
        let name_ink = |c: Color| {
            let seen = (0.299 * c.r + 0.587 * c.g + 0.114 * c.b) * c.a + 0.88 * (1.0 - c.a);
            if seen > 0.58 { Color::new(0.0, 0.0, 0.0, 0.7) } else { Color::new(1.0, 1.0, 1.0, 0.8) }
        };
        ui.label("New", swatch_new.x + 7.0, swatch_new.y + 10.0, 9.5, true, name_ink(picker.color()), 0.0);
        ui.label("Current", swatch_kept.x + 7.0, swatch_kept.y + swatch_kept.h - 10.0, 9.5, true, name_ink(kept_color), 0.0);
        if free && hit(swatch_kept, m, 0.0) {
            cursor = CursorIcon::Pointer;
            tip = Some(("Go back to this color (the last one you copied)", swatch_kept));
            if pressed && active.is_none() {
                picker.sync_hsv(kept.0, kept.1, kept.2);
                picker.alpha = kept.3 as f32 / 255.0;
            }
        }

        // ---------- screen pick button ----------
        let pick_hot = free && hit(pick_btn, m, 0.0);
        if screen::CAN_PICK {
            ui.button(pick_btn, pick_hot);
        } else {
            ui.fill(pick_btn, 5.0, SURFACE);
        }
        ui.label("Pick Screen", pick_btn.x + pick_btn.w * 0.5, pick_btn.y + pick_btn.h * 0.5, 11.5, true,
                 if screen::CAN_PICK { Color::from_rgba(12, 12, 13, 255) } else { TEXT_DIM }, 0.5);
        if pick_hot && !screen::CAN_PICK {
            tip = Some(("Screen picking isn't available on this system", pick_btn));
        } else if pick_hot {
            tip = Some(("Pick a color from anywhere on screen  (P)", pick_btn));
            if pressed && active.is_none() {
                picking = true;
                start_pick(&picker, &mut saved_picker);
            }
        }

        // ---------- mode dropdown ----------
        let mode_hot = !was_picking && !picking && !editing && hit(mode_btn, m, 0.0);
        ui.field(mode_btn, if mode_hot || dd_open { SURFACE_HI } else { SURFACE }, STROKE);
        let mode_cy = mode_btn.y + mode_btn.h * 0.5;
        ui.label(mode_name(mode), mode_btn.x + 9.0, mode_cy, 11.5, false, TEXT, 0.0);
        ui.icon_chevron(vec2(mode_btn.x + mode_btn.w - 12.0, mode_cy), false, TEXT_DIM);
        if mode_hot && pressed && active.is_none() {
            dd_open = !dd_open;
        }

        // ---------- channel sliders ----------
        for i in 0..4 {
            if chans[i].label.is_empty() {
                continue; // unused slot (GRAY has only G + A)
            }
            let hot = grab(Widget::Chan(i), hit(bars[i], m, 8.0)) > 0.0;
            draw_slider(&ui, bars[i], o.x + 314.0, o.x + 506.0, &chans[i], &chan_tex[i], hot);
        }

        // ---------- swatches: a window of 8 onto 100 slots, with a scrollbar ----------
        ui.clip(Some(Rect::new(sw_row.x - 1.0, sw_row.y - 1.0, sw_row.w + 2.0, sw_row.h + 2.0)));
        for slot in 0..=SWATCH_SHOWN {
            let i = sw_pos.floor() as usize + slot;
            if i >= SWATCH_SLOTS {
                break;
            }
            let cell = Rect::new(sw_row.x + (i as f32 - sw_pos) * (172.0 / 7.0), sw_row.y, 20.0, 20.0);
            let over = free && hit(cell, m, 1.0) && hit(sw_row, m, 1.0);
            match swatches[i] {
                Some((sr, sg, sb, sa)) => ui.checker(cell, 5.0, 5.0, Color::from_rgba(sr, sg, sb, sa)),
                None => ui.fill(cell, 5.0, if over { SURFACE_HI } else { SURFACE }),
            }
            ui.hair(cell, 5.0, if over { Color::new(1.0, 1.0, 1.0, 0.6) } else { STROKE });
            if !over {
                continue;
            }
            tip = Some((
                if swatches[i].is_some() { "Click: use  |  Right-click: replace  |  Middle-click: clear" }
                else { "Click to save the current color" },
                cell,
            ));
            if active.is_none() {
                if is_mouse_button_pressed(MouseButton::Middle) {
                    swatches[i] = None;
                    save_swatches(&swatches);
                } else if is_mouse_button_pressed(MouseButton::Right)
                    || (pressed && swatches[i].is_none())
                {
                    swatches[i] = Some((r, g, b, a8));
                    save_swatches(&swatches);
                } else if pressed {
                    if let Some((sr, sg, sb, sa)) = swatches[i] {
                        picker.sync_hsv(sr, sg, sb);
                        picker.alpha = sa as f32 / 255.0;
                    }
                }
            }
        }

        ui.clip(None);
        let scroll_hot = grab(Widget::SwatchScroll, hit(sw_track, m, 4.0)) > 0.0;
        ui.fill(sw_track, 2.5, SURFACE);
        let thumb_x = sw_track.x + sw_pos / sw_max * (sw_track.w - sw_thumb_w);
        ui.fill(Rect::new(thumb_x, sw_track.y, sw_thumb_w, sw_track.h), 2.5,
                if scroll_hot { Color::from_rgba(190, 190, 195, 255) } else { Color::from_rgba(112, 112, 118, 255) });
        if free && active.is_none() && hit(sw_track, m, 4.0) {
            tip = Some(("Scroll through the 100 color slots (the mouse wheel works too)", sw_track));
        }

        // ---------- value field: click to type, button to copy ----------
        let values_text = mode_values(mode, &picker);
        let field_cy = val_field.y + val_field.h * 0.5;
        let side_btn = Rect::new(val_field.x + val_field.w - 28.0, val_field.y, 28.0, val_field.h);
        let field_on = !was_picking && !picking && !dd_open;
        let over_field = field_on && hit(val_field, m, 0.0);
        let over_side = over_field && hit(side_btn, m, 0.0);
        let bad = now - bad_at < 0.9;
        let field_bg = if editing { POPOVER } else { SURFACE };
        ui.field(val_field, field_bg, if bad { DANGER } else if editing { Color::new(1.0, 1.0, 1.0, 0.55) } else { STROKE });
        let shown = if editing { edit_buf.clone() } else { format_mode(mode, &picker) };
        let room = val_field.w - 10.0 - 30.0;
        let mut fs = 12.0;
        while fs > 9.0 && ui.width(&shown, fs, false) > room {
            fs -= 0.5;
        }
        let tw = ui.width(&shown, fs, false);
        if editing && edit_fresh && !shown.is_empty() {
            ui.fill(Rect::new(val_field.x + 7.0, field_cy - 9.0, tw + 6.0, 18.0), 3.0, Color::new(1.0, 1.0, 1.0, 0.18));
        }
        ui.label(&shown, val_field.x + 10.0, field_cy, fs, false, TEXT, 0.0);
        if editing && !edit_fresh && (now * 2.0).fract() < 0.5 {
            ui.hairline(val_field.x + 11.0 + tw, field_cy - 7.0, 0.0, 14.0, TEXT);
        }
        let side_c = vec2(side_btn.x + 14.0, field_cy);
        if over_side {
            ui.fill(Rect::new(side_btn.x + 4.0, side_btn.y + 3.0, 20.0, 18.0), 4.0, SURFACE_HI);
        }
        let side_bg = if over_side { SURFACE_HI } else { field_bg };
        if editing {
            ui.icon_check(side_c, TEXT);
        } else if copied.0 == 1 && now - copied.1 < 1.2 {
            ui.icon_check(side_c, TEXT);
        } else {
            ui.icon_copy(side_c, if over_side { TEXT } else { TEXT_DIM }, side_bg);
        }
        if over_side {
            cursor = CursorIcon::Pointer;
            tip = Some((if editing { "Apply  (Enter)" } else { "Copy values  (Ctrl+C)" }, side_btn));
            if pressed && active.is_none() {
                if editing {
                    if apply_text(mode, &edit_buf, &mut picker) { editing = false; } else { bad_at = now; }
                } else {
                    clipboard_set(&values_text);
                    copied = (1, now);
                }
            }
        } else if over_field {
            cursor = CursorIcon::Text;
            if editing {
                if pressed { edit_fresh = false; }
            } else {
                tip = Some(("Click to type a value  |  Ctrl+V pastes a color", val_field));
                if pressed && active.is_none() {
                    editing = true;
                    edit_fresh = true;
                    edit_buf = values_text.clone();
                }
            }
        }

        // ---------- hex field (click to copy) ----------
        let hex_text = hex_string(r, g, b, a8);
        let hex_hot = free && hit(hex_field, m, 0.0);
        ui.field(hex_field, if hex_hot { SURFACE_HI } else { SURFACE }, STROKE);
        ui.label(&hex_text, hex_field.x + 10.0, field_cy, 12.5, true, TEXT, 0.0);
        let hex_c = vec2(hex_field.x + hex_field.w - 14.0, field_cy);
        if copied.0 == 2 && now - copied.1 < 1.2 {
            ui.icon_check(hex_c, TEXT);
        } else {
            ui.icon_copy(hex_c, if hex_hot { TEXT } else { TEXT_DIM }, if hex_hot { SURFACE_HI } else { SURFACE });
        }
        if hex_hot {
            tip = Some(("Copy hex", hex_field));
            if pressed && active.is_none() {
                clipboard_set(&hex_text);
                copied = (2, now);
            }
        }
        if copied.1 == now {
            kept = (r, g, b, a8);
        }

        // ---------- mode menu (drawn last so it sits above the sliders) ----------
        if dd_open {
            let cols = 3usize;
            let rows = MODES.len().div_ceil(cols);
            let (pad, cell_h) = (4.0, 20.0);
            let pop = Rect::new(o.x + 328.0, mode_btn.y + mode_btn.h + 2.0,
                                178.0, pad * 2.0 + rows as f32 * cell_h);
            let cell_w = (pop.w - pad * 2.0) / cols as f32;
            ui.shadow(pop, 8.0, 24.0, Color::new(0.0, 0.0, 0.0, 0.7));
            ui.fill(pop, 8.0, POPOVER);
            ui.hair(pop, 8.0, STROKE_HI);
            let mut chose = None;
            for (i, md) in MODES.iter().enumerate() {
                let cell = Rect::new(
                    pop.x + pad + (i % cols) as f32 * cell_w,
                    pop.y + pad + (i / cols) as f32 * cell_h,
                    cell_w, cell_h,
                );
                let over = m.x >= cell.x && m.x < cell.x + cell.w && m.y >= cell.y && m.y < cell.y + cell.h;
                let current = *md == mode;
                if over {
                    ui.fill(cell, 4.0, Color::new(1.0, 1.0, 1.0, 0.09));
                }
                ui.label(mode_name(*md), cell.x + 6.0, cell.y + cell.h * 0.5, 11.0,
                         current, if over || current { WHITE } else { TEXT_DIM }, 0.0);
                if over && pressed {
                    chose = Some(*md);
                }
            }
            match chose {
                Some(md) => {
                    mode = md;
                    dd_open = false;
                }
                None => {
                    let outside = !hit(pop, m, 2.0) && !hit(mode_btn, m, 0.0);
                    if (outside && pressed) || is_key_pressed(KeyCode::Escape) {
                        dd_open = false;
                    }
                }
            }
        }

        // ---------- title bar ----------
        draw_rectangle(0.0, 0.0, vw, TITLE_H, BG);
        ui.hairline(0.0, TITLE_H, vw, 0.0, STROKE);
        let title_cy = TITLE_H * 0.5;
        let version = "v1.0.1-ai-redesigned-edition";
        if screen::CUSTOM_FRAME {
            draw_texture_ex(&icon_tex, 8.0, title_cy - 9.0, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(18.0, 18.0)), ..Default::default() });
            ui.label(TITLE_TEXT, 32.0, title_cy, 12.0, true, TEXT, 0.0);
            let title_w = ui.width(TITLE_TEXT, 12.0, true);
            ui.label(version, 32.0 + title_w + 8.0, title_cy, 11.0, false, with_alpha(TEXT_DIM, 0.7), 0.0);
        } else {
            // the system title bar above already shows the name
            ui.label(version, 14.0, title_cy, 11.0, false, with_alpha(TEXT_DIM, 0.7), 0.0);
        }

        let maxed = screen::is_maximized();
        // dim, pin, then (with our own frame) minimize, maximize, close
        let buttons: u8 = if screen::CUSTOM_FRAME { 5 } else { 2 };
        for kind in 0..buttons {
            let rect = Rect::new(vw - (buttons - kind) as f32 * TITLE_BTN_W, 0.0, TITLE_BTN_W, TITLE_H);
            let hov = !was_picking && !picking && hit(rect, m, 0.0);
            let c = vec2(rect.x + rect.w * 0.5, title_cy);
            if hov {
                let bg = if kind == 4 { Color::from_rgba(196, 43, 28, 255) } else { Color::new(1.0, 1.0, 1.0, 0.09) };
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
            }
            let ink = if hov { WHITE } else { TEXT_DIM };
            let behind = if hov { Color::from_rgba(50, 50, 48, 255) } else { BG };
            match kind {
                0 => {
                    // half-filled circle: bright when dimming is on
                    let col = if dim_planes() { WHITE } else { with_alpha(ink, 0.7) };
                    ui.fill(Rect::new(c.x - 5.0, c.y - 5.0, 10.0, 10.0), 5.0, col);
                    draw_rectangle(c.x, c.y - 6.0, 6.5, 12.0, behind);
                    ui.ring(c, 5.0, 1.2, col);
                }
                1 => {
                    ui.icon_pin(c + vec2(1.0, 0.0), if on_top { WHITE } else { with_alpha(ink, 0.7) });
                }
                2 => ui.line(c - vec2(4.5, 0.0), c + vec2(4.5, 0.0), 1.2, ink),
                3 => {
                    if maxed {
                        ui.stroke(Rect::new(c.x - 2.5, c.y - 5.5, 8.0, 8.0), 1.5, 1.2, ink);
                        let front = Rect::new(c.x - 5.5, c.y - 2.5, 8.0, 8.0);
                        ui.fill(front, 1.5, behind);
                        ui.stroke(front, 1.5, 1.2, ink);
                    } else {
                        ui.stroke(Rect::new(c.x - 4.5, c.y - 4.5, 9.0, 9.0), 1.5, 1.2, ink);
                    }
                }
                _ => {
                    ui.line(c - vec2(4.0, 4.0), c + vec2(4.0, 4.0), 1.2, ink);
                    ui.line(c + vec2(-4.0, 4.0), c + vec2(4.0, -4.0), 1.2, ink);
                }
            }
            if hov {
                if kind == 0 {
                    tip = Some((
                        if dim_planes() { "Dim colors outside the displayable range: on" }
                        else { "Dim colors outside the displayable range: off" },
                        rect,
                    ));
                } else if kind == 1 {
                    tip = Some((if on_top { "Always on top: on" } else { "Keep window on top" }, rect));
                }
                if pressed && active.is_none() {
                    match kind {
                        0 => {
                            DIM_PLANES.store(!dim_planes(), std::sync::atomic::Ordering::Relaxed);
                            plane_key = (u8::MAX, f32::MAX, f32::MAX); // redraw the plane
                        }
                        1 => {
                            on_top = !on_top;
                            screen::set_topmost(on_top);
                        }
                        2 => screen::minimize_window(),
                        3 => screen::toggle_maximize_window(),
                        _ => screen::close_window(),
                    }
                }
            }
        }

        // ---------- tooltip ----------
        match tip {
            Some((text, anchor)) if !is_mouse_button_down(MouseButton::Left) => {
                if tip_text != text {
                    tip_text = text;
                    tip_since = now;
                }
                if now - tip_since > 0.55 {
                    let w = ui.width(text, 11.5, false) + 16.0;
                    let h = 22.0;
                    let x = (anchor.x + anchor.w * 0.5 - w * 0.5).clamp(6.0, (vw - w - 6.0).max(6.0));
                    let mut y = anchor.y + anchor.h + 6.0;
                    if y + h > vh - 4.0 {
                        y = anchor.y - h - 6.0;
                    }
                    let tr = Rect::new(ui.snap(x), ui.snap(y), w, h);
                    ui.field(tr, Color::from_rgba(60, 60, 56, 255), STROKE_HI);
                    ui.label(text, tr.x + 8.0, tr.y + h * 0.5, 11.5, false, TEXT, 0.0);
                }
            }
            _ => tip_text = "",
        }

        if picking {
            cursor = CursorIcon::Crosshair;
        }
        if cursor != cursor_now {
            cursor_now = cursor;
            set_mouse_cursor(cursor);
        }

        // remember the color, mode and pin for next launch (once the mouse is let go)
        let state = Saved { mode, rgba: (r, g, b, a8), on_top, dim: dim_planes() };
        if state != persisted && !picking && !is_mouse_button_down(MouseButton::Left) {
            persisted = state;
            save_state(&state);
        }

        set_default_camera();
        next_frame().await;
        let budget = 1.0 / refresh_hz;
        let spent = get_time() - frame_start;
        if spent < budget {
            std::thread::sleep(std::time::Duration::from_secs_f64(budget - spent)); } } }
