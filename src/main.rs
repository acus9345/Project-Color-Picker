#![windows_subsystem = "windows"]

use std::io::Error;
use macroquad::prelude::*;

// change them whenever you want
const SCALE_X: f32 = 0.75;
const SCALE_Y: f32 = 0.70;
//oh god
const WINDOW_W: f32 = 540.0 * SCALE_X;
const WINDOW_H: f32 = 388.0 * SCALE_Y;

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
//yea background color change it to whatever u want
const BG:Color=Color::from_rgba(32, 32, 29, 255);
const UI_TEXT:Color= Color::new(0.85, 0.87, 0.90, 1.0);
const HINT:Color= Color::new(0.55, 0.57, 0.62, 1.0);
const BORDER:Color = Color::new(0.0, 0.0, 0.0, 0.35);

// ----- customizable title bar -----
const TITLE_BAR_H:f32= 28.0;
const TITLE_BG:Color = Color::from_rgba(45, 45, 42, 255);
const TITLE_TEXT:&str= "Project Color picker";
const TITLE_BTN_W:f32 = 46.0;
/// caption/border color matching TITLE_BG (COLORREF = 0x00BBGGRR??? i guess).
const TITLE_CAPTION_BGR:u32 = 0x002A_2D_2D;

const DESIGN_W:f32 =540.0;
const DESIGN_H:f32= 388.0; // content + title bar
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


fn fit_rect(bounds: Rect,w: f32, h: f32)-> Rect {
    let s = (bounds.w/w).min(bounds.h/h);
    Rect::new(
        bounds.x+(bounds.w-w*s)*0.5,
        bounds.y+(bounds.h-h*s)*0.5,
        w*s,
        h*s, ) }

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


const LAB_GRID_RGB: (u8,u8,u8) =(62,62,62);


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
            if c == (0, 0, 0)
                && (in_locus_hull(x + dx, y)
                    || in_locus_hull(x - dx, y)
                    || in_locus_hull(x, y + dy)
                    || in_locus_hull(x, y - dy))
            { c = diagram_rgb(x + (0.3127 - x) * 0.02, y + (0.3290 - y) * 0.02, ); }
            if c == (0, 0, 0) && (on_grid(x, dx * 0.5) || on_grid(y, dy * 0.5)) {
                c = LAB_GRID_RGB; }
            out.extend_from_slice(&[c.0, c.1, c.2, 255]); } }
    out }

fn on_grid(v:f64,half_px:f64)->bool {
    let nearest = (v/0.1).round() * 0.1;
    (v -nearest).abs() <=half_px}

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

fn oklch_to_rgb(L: f64, c: f64, h: f64) -> (f32, f32, f32) {
    let hr = h.to_radians();
    let a = c * hr.cos();
    let bb = c * hr.sin();

    let l_ = L + 0.3963377774 * a + 0.2158037573 * bb;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * bb;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * bb;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

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


fn sample_grad(bytes: &[u8], t: f32) -> Color {
    let n = (bytes.len() / 4) as i32;
    let idx = ((t * (n - 1) as f32).round().clamp(0.0, (n - 1) as f32)) as usize;
    let i = idx * 4;
    Color::from_rgba(bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3])
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
            // sticky H/gw/B is the source of truth while in HWB mode
            let (h, w, bl) = p.hwb;
            let hw = |hh: f32| { let c = hwb_to_rgb(hh, 0.0, 0.0); [q8(c.0), q8(c.1), q8(c.2)] };
            let wb = |ww: f32, bb2: f32| { let c = hwb_to_rgb(h, ww, bb2); [q8(c.0), q8(c.1), q8(c.2)] };
            let ring = hue_ring(hw);
            [
                ChanUi { label: "H", t: h / 360.0, value: format!("{:.2}deg", h),
                    handle: color3(hw(h)),
                    bytes: grad_stops_bytes(gw, &ring) },
                ChanUi { label: "gw", t: w, value: pct(w),
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

fn swatch_path() -> std::path::PathBuf {
    exe_dir().join("palette.txt") }

fn load_swatches() ->[Option<(u8, u8, u8, u8)>; 8] {
    let mut out = [None; 8];
    let Ok(text) = std::fs::read_to_string(swatch_path()) else {
        return out;
    };
    for (i, line) in text.lines().take(8).enumerate() {
        if let Some((r, g, b, a)) = parse_rgba(line) {
            out[i] = Some((r, g, b, a.unwrap_or(255)));
        }
    }
    out
}

fn save_swatches(sw: &[Option<(u8, u8, u8, u8)>; 8]) {
    let mut s = String::new();
    for slot in sw {
        match slot {
            Some((r, g, b, a)) => s.push_str(&format!("{}, {}, {}, {}\n", r, g, b, a)),
            None => s.push_str("-\n"),
        }
    }
    let _ = std::fs::write(swatch_path(), s);
}

fn hints_path() -> std::path::PathBuf {
    exe_dir().join("hints.txt")
}

fn load_hints() -> [bool; 4] {
    let mut out = [true; 4];
    if let Ok(text) = std::fs::read_to_string(hints_path()) {
        for (i, line) in text.lines().take(4).enumerate() {
            out[i] = line.trim() == "1";
        }
    }
    out
}

fn save_hints(h: &[bool; 4]) {
    let s: String = h.iter().map(|v| if *v { "1\n" } else { "0\n" }).collect();
    let _ = std::fs::write(hints_path(), s);
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

fn alpha_grad_bytes(cr: u8, cg: u8, cb: u8) -> Vec<u8> {
    let (w, h) = (22usize, 258usize);
    let r = 11.0;
    let mut b = vec![0u8; w * h * 4];
    for t in 0..h {
        let yc = t as f32 + 0.5;
        let half = if yc < r {
            (r * r - (r - yc) * (r - yc)).sqrt()
        } else if yc > h as f32 - r {
            let d = yc - h as f32 + r;
            (r * r - d * d).sqrt()
        } else {
            r
        };
        let alpha = (255.0 * (1.0 - (t as f32 + 1.0) / (h as f32 + 2.0))).round() as u8;
        for x in 0..w {
            if (x as f32 + 0.5 - r).abs() <= half {
                let i = (t * w + x) * 4;
                b[i] = cr;
                b[i + 1] = cg;
                b[i + 2] = cb;
                b[i + 3] = alpha;
            }
        }
    }
    b
}

fn draw_stadium_rim(x: f32, y: f32, w: f32, h: f32, r: f32, fill: Color) {
    if h > w {
        let r = r.min(w * 0.5);
        draw_circle(x + r, y + r, r + 1.0, fill);
        draw_circle(x + r, y + h - r, r + 1.0, fill);
        draw_rectangle(x, y + r, w, h - 2.0 * r, fill);
    } else {
        let r = r.min(h * 0.5);
        draw_circle(x + r, y + r, r + 1.0, fill);
        draw_circle(x + w - r, y + r, r + 1.0, fill);
        draw_rectangle(x + r, y, w - 2.0 * r, h, fill);
    }
}

// ---------- static textures ----------

fn pill_texture(w: u16, h: u16, r: u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(w as usize * h as usize * 4);
    let (wf, hf, rf) = (w as f32, h as f32, r as f32);
    for y in 0..h {
        for x in 0..w {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let cx = px.clamp(rf, wf - rf);
            let cy = py.clamp(rf, hf - rf);
            let d = (px - cx).hypot(py - cy) - rf;
            let a = (0.5 - d).clamp(0.0, 1.0);
            bytes.extend_from_slice(&[255, 255, 255, (a * 255.0) as u8]);
        }
    }
    let t = Texture2D::from_rgba8(w, h, &bytes);
    t.set_filter(FilterMode::Linear);
    t
}

fn hue_bar_texture(w: u16, h: u16, r: u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(w as usize * h as usize * 4);
    let (wf, hf, rf) = (w as f32, h as f32, r as f32);
    for y in 0..h {
        let (r, g, b) = hsv_to_rgb(y as f32 / h as f32 * 360.0, 1.0, 1.0);
        for x in 0..w {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let dx = if px < rf { rf - px } else if px > wf - rf { px - (wf - rf) } else { 0.0 };
            let dy = if py < rf { rf - py } else if py > hf - rf { py - (hf - rf) } else { 0.0 };
            if dx * dx + dy * dy <= rf * rf {
                bytes.extend_from_slice(&[r, g, b, 255]);
            } else {
                bytes.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    Texture2D::from_rgba8(w, h, &bytes)
}

fn gray_v_bar_texture(w:u16,h:u16,r:u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(w as usize * h as usize * 4);
    let (wf, hf, rf) = (w as f32, h as f32, r as f32);
    for y in 0..h {
        let g =q8(1.0 - y as f32 / h as f32);
        for x in 0..w{
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let dx = if px < rf { rf - px } else if px > wf - rf { px - (wf - rf) } else { 0.0 };
            let dy = if py < rf { rf - py } else if py > hf - rf { py - (hf - rf) } else { 0.0 };
            if dx * dx + dy * dy <= rf * rf {
                bytes.extend_from_slice(&[g, g, g, 255]);
            } else {
                bytes.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    Texture2D::from_rgba8(w, h, &bytes)
}

fn checker_texture(w: u16, h: u16, cell: u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(w as usize * h as usize * 4);
    for y in 0..h {
        for x in 0..w {
            let c = if ((x / cell) + (y / cell)) % 2 == 0 {
                [255, 255, 255, 255]
            } else {
                [158, 158, 158, 255]
            };
            bytes.extend_from_slice(&c);
        }
    }
    Texture2D::from_rgba8(w, h, &bytes)
}

fn checker_pill_texture(w: u16, h: u16, cell: u16, r: u16) -> Texture2D {
    let mut bytes = Vec::with_capacity(w as usize * h as usize * 4);
    let (wf, hf, rf) = (w as f32, h as f32, r as f32);
    for y in 0..h {
        for x in 0..w {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let dx = if px < rf { rf - px } else if px > wf - rf { px - (wf - rf) } else { 0.0 };
            let dy = if py < rf { rf - py } else if py > hf - rf { py - (hf - rf) } else { 0.0 };
            if dx * dx + dy * dy <= rf * rf {
                let c = if ((x / cell) + (y / cell)) % 2 == 0 {
                    [255, 255, 255, 255]
                } else {
                    [158, 158, 158, 255]
                };
                bytes.extend_from_slice(&c);
            } else {
                bytes.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    Texture2D::from_rgba8(w, h, &bytes)
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
            let p = hwb_to_rgb(hue, col as f32 / w as f32, bl);
            b.extend_from_slice(&[q8(p.0), q8(p.1), q8(p.2), 255]);
        }
    }
    b
}


fn oklch_square_bytes(l: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        let hue = (1.0 - row as f32 / h as f32) * 360.0;
        for col in 0..w {
            let c = (col as f32 / w as f32 * OK_CHROMA_MAX) as f64;
            let p = oklch_to_rgb(l.clamp(0.0, 1.0) as f64, c, hue as f64);
            b.extend_from_slice(&[q8(p.0), q8(p.1), q8(p.2), 255]);
        }
    }
    b
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
            b.extend_from_slice(&[q8(p.0.clamp(0.0, 1.0)), q8(p.1.clamp(0.0, 1.0)),
                                  q8(p.2.clamp(0.0, 1.0)), 255]);
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
            b.extend_from_slice(&[q8(p.0.clamp(0.0, 1.0)), q8(p.1.clamp(0.0, 1.0)),
                                  q8(p.2.clamp(0.0, 1.0)), 255]);
        }
    }
    b
}

/// Munsell hue wheel: the angle around the center is the Munsell hue
/// H 0..100 (clockwise from 12 o'clock, like an HSV wheel), colored at the
/// current value V and chroma C. Transparent outside the disc.
fn munsell_wheel_bytes(v: f32, c: f32) -> Vec<u8> {
    let (w, h) = (260usize, 260usize);
    let (cx, cy) = (129.5f32, 129.5f32);
    let r_max = 126.0f32;
    let mut b = Vec::with_capacity(w * h * 4);
    for row in 0..h {
        for col in 0..w {
            let dx = col as f32 + 0.5 - cx;
            let dy = row as f32 + 0.5 - cy;
            let d = dx.hypot(dy);
            let a = (0.5 - (d - r_max)).clamp(0.0, 1.0); // ~1px AA edge
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

    pub fn frame_tick(cap_h: i32, logical_h: i32) {
        CAP_H.store(cap_h, Ordering::Relaxed);
        LOG_H.store(logical_h, Ordering::Relaxed);
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
#[cfg(not(target_os = "windows"))]
mod screen {
    pub const CAPTURE_N: i32 = 31;
    pub fn start_pick() {}
    pub fn stop_pick() {}
    pub fn take_click() -> Option<(i32, i32)> { None }
    pub fn set_window_icons() -> bool { true }
    pub fn apply_win11_style(_caption_bgr: u32) -> bool { true }
    pub fn dpi_scale() -> f32 { 1.0 }
    pub fn set_topmost(_on: bool) {}
    pub fn enable_custom_frame(_cap: i32, _lh: i32, _zone: i32) -> bool { true }
    pub fn frame_tick(_cap: i32, _lh: i32) {}
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

fn draw_slider(
    bar: Rect,
    label: &str,
    value_text: &str,
    t: f32,
    grad: &Texture2D,
    handle: Color,
    end_l: Color,
    end_r: Color,
    checker: Option<&Texture2D>,
) {
    let r = (bar.h * 0.5).min(6.0);
    let cy = bar.y + r;

    draw_circle(bar.x + r, cy, r + 1.0, BG);
    draw_circle(bar.x + bar.w - r, cy, r + 1.0, BG);
    draw_rectangle(bar.x + r, bar.y, bar.w - 2.0 * r, bar.h, BG);

    if let Some(tex) = checker {
        draw_texture_ex(tex, bar.x + 1.0, bar.y + 1.0, WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(bar.w - 2.0 * 1.0, bar.h - 2.0 * 1.0)),
                            ..Default::default()
                        });
    }

    draw_texture_ex(grad, bar.x + r, bar.y + 1.0, WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(r, 0.0, bar.w - 2.0 * r, 1.0)),
                        dest_size: Some(vec2(bar.w - 2.0 * r, bar.h - 2.0 * 1.0)),
                        ..Default::default()
                    });

    draw_circle(bar.x + r, cy, r - 1.0, end_l);
    draw_circle(bar.x + bar.w - r, cy, r - 1.0, end_r);

    draw_text(label, bar.x, bar.y - 5.0, 18.0, UI_TEXT);
    let dims = measure_text(value_text, None, 18.0 as u16, 1.0);
    draw_text(value_text, bar.x + bar.w - dims.width, bar.y - 5.0, 18.0, UI_TEXT);


    let (cx, cyy) = (bar.x + r + t * (bar.w - 2.0 * r), cy);
    draw_circle(cx, cyy, 8.0, WHITE);
    draw_circle(cx, cyy, 6.0, handle);
    draw_circle_lines(cx, cyy, 8.0, 1.5, BLACK);
}

fn draw_lab_reference(r: Rect) {
    let to_screen = |x: f64, y: f64| vec2(
        r.x + lab_img_u(x) * r.w,
        r.y + (1.0 - lab_img_v(y)) * r.h,
    );

    let axis = Color::new(1.0, 1.0, 1.0, 0.55);
    let tick_txt = Color::new(1.0, 1.0, 1.0, 0.80);
    draw_line(r.x, r.y + r.h, r.x + r.w, r.y + r.h, 1.0, axis);
    draw_line(r.x, r.y, r.x, r.y + r.h, 1.0, axis);
    for i in 1..8 {
        let gx = 0.1 * i as f64;
        if gx > LAB_IMG_X_MIN && gx < LAB_IMG_X_MAX {
            let lx = r.x + lab_img_u(gx) * r.w;
            draw_line(lx, r.y + r.h, lx, r.y + r.h + 3.0, 1.0, axis);
            let t = format!("{:.1}", gx);
            let tw = measure_text(&t, None, 9.0 as u16, 1.0).width;
            draw_text(&t, lx - tw * 0.5, r.y + r.h + 13.0, 9.0, tick_txt);
        }
    }
    for i in 1..9 {
        let gy = 0.1 * i as f64;
        if gy > LAB_IMG_Y_MIN && gy < LAB_IMG_Y_MAX {
            let ly = r.y + (1.0 - lab_img_v(gy)) * r.h;
            draw_line(r.x - 3.0, ly, r.x, ly, 1.0, axis);
            let t = format!("{:.1}", gy);
            let tw = measure_text(&t, None, 9.0 as u16, 1.0).width;
            draw_text(&t, r.x - 5.0 - tw, ly + 3.0, 9.0, tick_txt);
        }
    }


    let pts: Vec<Vec2> = SPECTRAL_LOCUS
        .iter()
        .map(|p| to_screen(p.1, p.2))
        .collect();
    let outline = Color::new(1.0, 1.0, 1.0, 0.85);
    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[(i + 1) % pts.len()];
        draw_line(a.x, a.y, b.x, b.y, 1.0, outline);
    }
    let white = to_screen(0.3127, 0.3290);
    for nm in [380.0f64, 460.0, 470.0, 480.0, 490.0, 500.0, 520.0, 540.0,
               560.0, 580.0, 600.0, 620.0, 700.0] {
        let p = match SPECTRAL_LOCUS.iter().find(|p| p.0 == nm) {
            Some(p) => p,
            None => continue,
        };
        let sp = to_screen(p.1, p.2);
        let d = (sp - white).normalize_or_zero();
        let lp = sp + d * 13.0;
        let t = format!("{}", nm as i32);
        let tw = measure_text(&t, None, 9.0 as u16, 1.0).width;
        draw_text(&t, lp.x - tw * 0.5, lp.y + 3.0, 9.0,
                  Color::new(1.0, 0.45, 0.32, 0.95));
    }
}
fn draw_uv_reference(sv: Rect, range: f32, u_label: &str, v_label: &str) {
    let cx = sv.x + sv.w * 0.5;
    let cy = sv.y + sv.h * 0.5;
    let line = Color::new(1.0, 1.0, 1.0, 0.85);
    draw_line(sv.x, cy, sv.x + sv.w, cy, 1.0, line); // V = 0
    draw_line(cx, sv.y, cx, sv.y + sv.h, 1.0, line); // U = 0

    let step = sv.w * (0.1 / (range * 2.0));
    let n = (range / 0.1) as i32;
    let tick = Color::new(1.0, 1.0, 1.0, 0.8);
    let lbl = Color::new(1.0, 1.0, 1.0, 0.9);
    for k in 1..=n {
        let d = k as f32 * step;
        draw_line(cx + d, cy - 2.5, cx + d, cy + 2.5, 1.0, tick);
        draw_line(cx - d, cy - 2.5, cx - d, cy + 2.5, 1.0, tick);
        draw_line(cx - 2.5, cy - d, cx + 2.5, cy - d, 1.0, tick);
        draw_line(cx - 2.5, cy + d, cx + 2.5, cy + d, 1.0, tick);
        let tp = format!("+.{k}");
        let tm = format!("-.{k}");
        let tw = measure_text(&tp, None, 8.0 as u16, 1.0).width;
        let twm = measure_text(&tm, None, 8.0 as u16, 1.0).width;
        draw_text(&tp, cx + d - tw * 0.5, cy + 11.0, 8.0, lbl);
        draw_text(&tm, cx - d - twm * 0.5, cy + 11.0, 8.0, lbl);
        draw_text(&tp, cx + 4.0, cy - d + 3.0, 8.0, lbl);
        draw_text(&tm, cx + 4.0, cy + d + 3.0, 8.0, lbl);
    }
    draw_text(u_label, sv.x + sv.w - 11.0, cy - 4.0, 9.0, lbl);
    draw_text(v_label, cx + 5.0, sv.y + 10.0, 9.0, lbl);
}

#[macroquad::main(conf)]
async fn main() {

    let dpi = screen::dpi_scale();
    if dpi > 1.0 {
        request_new_screen_size(WINDOW_W * dpi, WINDOW_H * dpi);
    }

    let mut picker = Picker::new();
    let mut active: Option<Widget> = None;
    let mut copied_at: f64 = f64::NEG_INFINITY;
    let mut picking = false;
    let mut saved_picker: Option<Picker> = None;
    let mut last_nudge: f64 = 0.0;

    let mut editing = false;
    let mut edit_buf = String::new();
    let mut last_backspace: f64 = 0.0;
    let mut bad_edit_at: f64 = f64::NEG_INFINITY;

    let mut mode = ColorMode::Rgba;
    let mut dd_open = false;
    let mut on_top = false;
    let mut hints_vis = load_hints();

    let mut swatches: [Option<(u8, u8, u8, u8)>; 8] = load_swatches();

    let mut styled = screen::apply_win11_style(TITLE_CAPTION_BGR);
    let mut framed = false;
    let mut iconed = screen::set_window_icons();
    let lab_img = Texture2D::from_rgba8(LAB_TEX_W, LAB_TEX_H, &lab_diagram_bytes(LAB_TEX_W, LAB_TEX_H));
    lab_img.set_filter(FilterMode::Linear);

    let hue_tex = hue_bar_texture(24, 260, 12);
    let sat_tex = sat_overlay_texture(260);
    let val_tex = val_overlay_texture(260);
    let mut hsl_sq = Texture2D::from_rgba8(260, 260, &hsl_square_bytes(picker.hue));
    hsl_sq.set_filter(FilterMode::Linear);
    let mut hsl_sq_hue = picker.hue;
    let gray_sq = Texture2D::from_rgba8(260, 260, &gray_square_bytes());
    gray_sq.set_filter(FilterMode::Linear);

    let plane_sq = Texture2D::from_rgba8(260, 260, &vec![0u8; 260 * 260 * 4]);
    plane_sq.set_filter(FilterMode::Linear);
    let mut plane_key = (u8::MAX, f32::MAX, f32::MAX);
    let mut plane_pos_mode = ColorMode::Rgba;
    let mut plane_pos_key = (u8::MAX, u8::MAX, u8::MAX, u8::MAX);
    let gray_v_bar = gray_v_bar_texture(24, 260, 12);
    let checker_alpha = checker_pill_texture(24, 260, 6, 12);
    let checker_preview = checker_texture(144.0 as u16, 72.0 as u16, 8.0 as u16);
    let checker_slider = checker_pill_texture(144, 12, 4, 6);
    let pill_pick = pill_texture(100.0 as u16, 20.0 as u16, 6.0 as u16);
    let pill_edit = pill_texture(48.0 as u16, 18.0 as u16, 6.0 as u16);
    let pill_dd = pill_texture(52.0 as u16, 18.0 as u16, 6.0 as u16);
    let pill_top = pill_texture(50.0 as u16, 18.0 as u16, 6.0 as u16);
    let pill_input = pill_texture(148.0 as u16, 18.0 as u16, 6.0 as u16);
    let pill_hints = pill_texture(70.0 as u16, 16.0 as u16, 6.0 as u16);
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
    let ag_w = 22.0 as u16;
    let ag_h = 258.0 as u16;
    let alpha_grad = Texture2D::from_rgba8(ag_w, ag_h, &vec![0u8; ag_w as usize * ag_h as usize * 4]);
    alpha_grad.set_filter(FilterMode::Linear);
    let mut chans = mode_sliders(mode, &picker);
    let mut last_key = [i64::MIN; 39];
    screen::set_high_res_timer();
    let refresh_hz = screen::monitor_refresh_hz().max(30) as f64;
    loop {
        let frame_start = get_time();
        clear_background(BG);

        if !styled {
            styled = screen::apply_win11_style(TITLE_CAPTION_BGR);
        }
        if !framed {
            framed = screen::enable_custom_frame(
                TITLE_BAR_H as i32,
                WINDOW_H as i32,
                (TITLE_BTN_W * 3.0) as i32,
            );
        }
        if !iconed {
            iconed = screen::set_window_icons();
        }
        let (sw, sh) = (screen_width(), screen_height());
        screen::frame_tick(TITLE_BAR_H as i32, sh as i32);
        let scale = (sw / DESIGN_W).min(sh / DESIGN_H);
        let off = vec2(
            (sw - DESIGN_W * scale) * 0.5,
            (sh - DESIGN_H * scale) * 0.5,
        );
        set_camera(&Camera2D {
            target: vec2(DESIGN_W * 0.5, DESIGN_H * 0.5),
            zoom: vec2(2.0 * scale / sw, 2.0 * scale / sh),
            ..Default::default()
        });
        let (mx, my) = mouse_position();
        let mx = (mx - off.x) / scale;
        let my = (my - off.y) / scale;
        let m = if screen::mouse_in_window()
            || is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Right)
        {
            Vec2::new(mx, my)
        } else {
            Vec2::new(-1.0e6, -1.0e6)
        };

        let sv = Rect::new(20.0, TITLE_BAR_H + 20.0, 260.0, 260.0);
        // LAB block: largest diagram-aspect rect centered in the square
        let lab_rect = fit_rect(sv, LAB_TEX_W as f32, LAB_TEX_H as f32);
        let hue_bar = Rect::new(296.0, TITLE_BAR_H + 20.0, 24.0, 260.0);
        let alpha_bar = Rect::new(336.0, TITLE_BAR_H + 20.0, 24.0, 260.0);
        let preview = Rect::new(376.0, TITLE_BAR_H + 20.0, 144.0, 72.0);
        let bars = [
            Rect::new(376.0, TITLE_BAR_H + 118.0, 144.0, 12.0),
            Rect::new(376.0, TITLE_BAR_H + 150.0, 144.0, 12.0),
            Rect::new(376.0, TITLE_BAR_H + 182.0, 144.0, 12.0),
            Rect::new(376.0, TITLE_BAR_H + 214.0, 144.0, 12.0),
        ];

        // ------- screen-pick mode ----------(alr)
        let was_picking = picking;
        if picking {
            let (mut gx, mut gy) = screen::cursor();
            let t = get_time();
            if t - last_nudge > 0.05 {
                let (nx, ny) = (gx, gy);
                if is_key_down(KeyCode::Left) { gx -= 1; }
                if is_key_down(KeyCode::Right) { gx += 1; }
                if is_key_down(KeyCode::Up) { gy -= 1; }
                if is_key_down(KeyCode::Down) { gy += 1; }
                if (gx, gy) != (nx, ny) {
                    screen::set_cursor(gx, gy);
                    last_nudge = t;
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
                macroquad::miniquad::window::clipboard_set(
                    &format!("#{:02X}{:02X}{:02X}", pr, pg, pb),
                );
                copied_at = get_time();
                picking = false;
                screen::stop_pick();
                screen::overlay_hide();
                macroquad::miniquad::window::set_mouse_cursor(
                    macroquad::miniquad::CursorIcon::Default,
                );
            } else if is_key_pressed(KeyCode::Escape) {
                if let Some(p) = saved_picker {
                    picker = p;
                }
                picking = false;
                screen::stop_pick();
                screen::overlay_hide();
                macroquad::miniquad::window::set_mouse_cursor(
                    macroquad::miniquad::CursorIcon::Default,
                ); } }

        // -------- text edit mode ----------
        if editing {
            while let Some(ch) = get_char_pressed() {
                if ch.is_ascii_alphanumeric()
                    || matches!(ch, ',' | ' ' | '.' | '#' | '%' | '-')
                {
                    edit_buf.push(ch);
                }
            }
            if is_key_down(KeyCode::Backspace) && get_time() - last_backspace > 0.12 {
                edit_buf.pop();
                last_backspace = get_time();
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                match parse_mode(mode, &edit_buf) {
                    Some((rgb, a)) => {
                        picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                        if let Some(a) = a { picker.alpha = a; }editing = false; }
                    None => {
                        bad_edit_at = get_time();
                    } } } else if is_key_pressed(KeyCode::Escape) {
                editing = false; } }

        if !was_picking && !editing && !dd_open
            && is_mouse_button_pressed(MouseButton::Left) && active.is_none()
        {
            active = if hit(sv, m, 0.0) { Some(Widget::SatVal)
            } else if hit(hue_bar, m, 8.0) { Some(Widget::Hue)
            } else if hit(alpha_bar, m, 8.0) { Some(Widget::Alpha)
            } else if hit(bars[0], m, 10.0) { Some(Widget::Chan(0))
            } else if hit(bars[1], m, 10.0) { Some(Widget::Chan(1))
            } else if hit(bars[2], m, 10.0) { Some(Widget::Chan(2))
            } else if hit(bars[3], m, 10.0) { Some(Widget::Chan(3)) } else {
                None
            }; }

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

                    let gv = 1.0 - clamp01((m.y - hue_bar.y) / hue_bar.h);
                    picker.sync_rgb_f(gv, gv, gv);
                }
                Some(Widget::Hue) => {
                    picker.hue = hue_from_t(clamp01((m.y - hue_bar.y) / hue_bar.h));

                    let (r, g, b) = picker.rgb_f32();
                    picker.keep_hsv = true;
                    picker.sync_rgb_f(r, g, b);
                }
                Some(Widget::Alpha) => {
                    picker.alpha = clamp01(1.0 - (m.y - alpha_bar.y) / alpha_bar.h)
                }
                Some(Widget::Chan(i)) => {
                    let t = clamp01((m.x - bars[i].x) / bars[i].w);
                    set_channel(mode, i, t, &mut picker);
                }
                None => {}
            }
        } else {
            active = None;
        }

        let (r, g, b) = picker.rgb();
        let (rf, gf, bf) = picker.rgb_f32();
        let a8 = q8(picker.alpha);

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
            alpha_grad.update(&Image { width: ag_w, height: ag_h,
                bytes: alpha_grad_bytes(r, g, b) });
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
        if mode == ColorMode::Gray {
            draw_texture_ex(&gray_sq, sv.x, sv.y, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(sv.w, sv.h)), ..Default::default() });
        } else if diagram_mode {
            // LAB/xyZ/xyY: the cie 1931 chromaticity diagram + reference lines
            draw_texture_ex(&lab_img, lab_rect.x, lab_rect.y, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(lab_rect.w, lab_rect.h)), ..Default::default() });
            draw_lab_reference(lab_rect);
        } else if plane_mode {
            draw_texture_ex(&plane_sq, sv.x, sv.y, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(sv.w, sv.h)), ..Default::default() });
            if mode == ColorMode::Yuv {
                draw_uv_reference(sv, 0.436, "U", "V");
            } else if mode == ColorMode::Ypbpr {
                draw_uv_reference(sv, 0.5, "Pb", "Pr");
            }
        } else if hsl_mode {
            draw_texture_ex(&hsl_sq, sv.x, sv.y, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(sv.w, sv.h)), ..Default::default() });
        } else {
            draw_rectangle(sv.x, sv.y, sv.w, sv.h, Color::from_rgba(hr, hg, hb, 255));
            draw_texture_ex(&sat_tex, sv.x, sv.y, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(sv.w, sv.h)), ..Default::default() });
            draw_texture_ex(&val_tex, sv.x, sv.y, WHITE,
                            DrawTextureParams { dest_size: Some(vec2(sv.w, sv.h)), ..Default::default() });
        }
        draw_rectangle_lines(sv.x, sv.y, sv.w, sv.h, 1.0, BORDER);

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
            let rr = sv.w * 0.5 * 0.8;
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
        draw_circle(cx, cy, 8.0, WHITE);
        draw_circle(cx, cy, 6.0, Color::from_rgba(r, g, b, 255));
        draw_circle_lines(cx, cy, 8.0, 1.5, BLACK);

        draw_stadium_rim(hue_bar.x, hue_bar.y, hue_bar.w, hue_bar.h, 12.0, BG);
        if mode == ColorMode::Gray {
            draw_texture_ex(&gray_v_bar, hue_bar.x + 1.0, hue_bar.y + 1.0, WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(hue_bar.w - 2.0, hue_bar.h - 2.0)),
                                ..Default::default()
                            });
            let gy = hue_bar.y + 12.0 + (1.0 - luma8(rf, gf, bf) as f32 / 255.0) * (hue_bar.h - 24.0);
            draw_circle(hue_bar.x + hue_bar.w * 0.5, gy, 9.0, WHITE);
            draw_circle(hue_bar.x + hue_bar.w * 0.5, gy, 7.0, Color::from_rgba(r, g, b, 255));
            draw_circle_lines(hue_bar.x + hue_bar.w * 0.5, gy, 9.0, 1.5, BLACK);
        } else {
            draw_texture_ex(&hue_tex, hue_bar.x + 1.0, hue_bar.y + 1.0, WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(hue_bar.w - 2.0, hue_bar.h - 2.0)),
                                ..Default::default()
                            });
            let (hx, hy) = (
                hue_bar.x + hue_bar.w * 0.5,
                hue_bar.y + 12.0 + picker.hue / 360.0 * (hue_bar.h - 24.0),
            );
            draw_circle(hx, hy, 9.0, WHITE);
            draw_circle(hx, hy, 7.0, Color::from_rgba(hr, hg, hb, 255));
            draw_circle_lines(hx, hy, 9.0, 1.5, BLACK);
        }

        draw_stadium_rim(alpha_bar.x, alpha_bar.y, alpha_bar.w, alpha_bar.h, 12.0, BG);
        draw_texture_ex(&checker_alpha, alpha_bar.x + 1.0, alpha_bar.y + 1.0, WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(alpha_bar.w - 2.0, alpha_bar.h - 2.0)),
                            ..Default::default()
                        });
        draw_texture(&alpha_grad, alpha_bar.x + 1.0, alpha_bar.y + 1.0, WHITE);
        let (ax, ay) = (
            alpha_bar.x + alpha_bar.w * 0.5,
            alpha_bar.y + 12.0 + (1.0 - picker.alpha) * (alpha_bar.h - 24.0),
        );
        draw_circle(ax, ay, 9.0, WHITE);
        draw_circle(ax, ay, 7.0, picker.color());
        draw_circle_lines(ax, ay, 9.0, 1.5, BLACK);

        // ---------- preview ----------
        draw_texture(&checker_preview, preview.x, preview.y, WHITE);
        draw_rectangle(preview.x, preview.y, preview.w, preview.h, picker.color());
        draw_rectangle_lines(preview.x, preview.y, preview.w, preview.h, 1.0, BORDER);

        for i in 0..4 {
            if chans[i].label.is_empty() {
                continue; // unused slot (INDEX has only I + A)
            }
            let rr = (bars[i].h * 0.5).min(6.0);
            let tl = rr / bars[i].w;
            let end_l = sample_grad(&chans[i].bytes, tl);
            let end_r = sample_grad(&chans[i].bytes, 1.0 - tl);
            let checker = if chans[i].label == "A" {
                Some(&checker_slider)
            } else {
                None
            };
            draw_slider(
                bars[i], chans[i].label, &chans[i].value, chans[i].t,
                &chan_tex[i], chans[i].handle, end_l, end_r, checker,
            );
        }

        // ---------- click-to-copy (copies values only well u can change it if u want) ----
        let mode_text = format_mode(mode, &picker);
        let values_text = mode_values(mode, &picker);
        let rd = measure_text(&mode_text, None, 26.0 as u16, 1.0);
        let over = !was_picking && !editing && !dd_open
            && mx >= 16.0 && mx <= 24.0 + rd.width
            && my >= TITLE_BAR_H + 294.0 && my <= TITLE_BAR_H + 328.0;
        let clicked = over && is_mouse_button_pressed(MouseButton::Left) && active.is_none();

        if clicked { macroquad::miniquad::window::clipboard_set(&values_text);copied_at = get_time(); }
        let copied = get_time() - copied_at < 1.0;
        if copied { draw_text("copied", 30.0 + rd.width, TITLE_BAR_H + 324.0, 15.0, LIGHTGRAY); }
        draw_text(&mode_text, 20.0, TITLE_BAR_H + 318.0, 26.0,
                  if copied { Color::from_rgba(251, 255, 118, 160) }
                  else if over { Color::from_rgba(251, 255, 118, 255) }
                  else { UI_TEXT });
        if over || copied { draw_rectangle(20.0, TITLE_BAR_H + 323.0, rd.width, 2.0, LIGHTGRAY); }
        let hex_text = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let hd = measure_text(&hex_text, None, 26.0 as u16, 1.0);
        draw_text(&hex_text, DESIGN_W - 20.0 - hd.width, TITLE_BAR_H + 318.0, 26.0, UI_TEXT);
        if !picking {
            let edit_btn = Rect::new(20.0, TITLE_BAR_H + 336.0, 48.0, 18.0);
            let edit_over = hit(edit_btn, m, 0.0);
            draw_texture(&pill_edit, edit_btn.x, edit_btn.y,
                         if edit_over { Color::from_rgba(60, 64, 74, 255) } else { Color::from_rgba(45, 48, 56, 255) });
            let et = measure_text(if editing { "OK" } else { "EDIT" }, None, 13.0 as u16, 1.0);
            draw_text(if editing { "OK" } else { "EDIT" },
                      edit_btn.x + (edit_btn.w - et.width) * 0.5, edit_btn.y + 13.5, 13.0, UI_TEXT);
            if edit_over && is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
                dd_open = false;
                if editing {
                    match parse_mode(mode, &edit_buf) {
                        Some((rgb, a)) => {
                            picker.sync_rgb_f(rgb.0, rgb.1, rgb.2);
                            if let Some(a) = a { picker.alpha = a; }
                            editing = false;
                        }
                        None => bad_edit_at = get_time(),
                    }
                } else {
                    editing = true;
                    edit_buf = values_text.clone();
                    last_backspace = 0.0;
                }
            }


            let dd_btn = Rect::new(72.0, TITLE_BAR_H + 336.0, 52.0, 18.0);
            let dd_over = !editing && hit(dd_btn, m, 0.0);
            draw_texture(&pill_dd, dd_btn.x, dd_btn.y,
                         if dd_over || dd_open { Color::from_rgba(60, 64, 74, 255) } else { Color::from_rgba(45, 48, 56, 255) });
            let dn = mode_name(mode);
            let dt = measure_text(dn, None, 12.0 as u16, 1.0);
            draw_text(dn, dd_btn.x + (dd_btn.w - dt.width) * 0.5, dd_btn.y + 13.5, 12.0, UI_TEXT);
            draw_text(if dd_open { "^" } else { "v" },
                      dd_btn.x + dd_btn.w - 8.0, dd_btn.y + 13.0, 10.0, HINT);
            if dd_over && is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
                dd_open = !dd_open;
            }

            if dd_open {
                let ih = 16.0;
                let top = dd_btn.y - MODES.len() as f32 * ih;
                draw_rectangle(dd_btn.x, top, 64.0, MODES.len() as f32 * ih,
                               Color::from_rgba(24, 26, 24, 255));
                draw_rectangle_lines(dd_btn.x, top, 64.0, MODES.len() as f32 * ih, 1.0, BORDER);
                let mut chose = None;
                for (i, md) in MODES.iter().enumerate() {
                    let ry = top + i as f32 * ih;
                    let iov = m.x >= dd_btn.x && m.x <= dd_btn.x + 64.0
                        && m.y >= ry && m.y <= ry + ih;
                    if iov {
                        draw_rectangle(dd_btn.x + 1.0, ry + 1.0, 62.0, ih - 2.0,
                                       Color::from_rgba(60, 64, 74, 255));
                    }
                    let name = mode_name(*md);
                    let nt = measure_text(name, None, 12.0 as u16, 1.0);
                    draw_text(name, dd_btn.x + (64.0 - nt.width) * 0.5, ry + 12.5, 12.0, UI_TEXT);
                    if iov && is_mouse_button_pressed(MouseButton::Left) {
                        chose = Some(*md);
                    }
                }
                match chose {
                    Some(md) => {
                        mode = md;
                        dd_open = false;
                    }
                    None => {
                        let in_dd = m.x >= dd_btn.x - 2.0 && m.x <= dd_btn.x + 66.0 && m.y >= top - 2.0 && m.y <= dd_btn.y + dd_btn.h;
                        if !in_dd && is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
                            dd_open = false;
                        }
                    }
                }
            }

            // ----- ON TOP toggle -----
            let top_btn = Rect::new(128.0, TITLE_BAR_H + 336.0, 50.0, 18.0);
            let top_over = !editing && !dd_open && hit(top_btn, m, 0.0);
            draw_texture(&pill_top, top_btn.x, top_btn.y,
                         if on_top {
                             WHITE
                         } else if top_over {
                             Color::from_rgba(60, 64, 74, 255)
                         } else {
                             Color::from_rgba(45, 48, 56, 255)
                         });
            let tt = measure_text("ON TOP", None, 10.0 as u16, 1.0);
            draw_text("ON TOP",
                      top_btn.x + (top_btn.w - tt.width) * 0.5, top_btn.y + 12.5, 10.0,
                      if on_top { Color::from_rgba(110, 112, 108, 255) } else { UI_TEXT });
            if top_over && is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
                on_top = !on_top;
                screen::set_topmost(on_top);
            }

            // ----- edit input box -----
            if editing {
                let box_r = Rect::new(182.0, TITLE_BAR_H + 336.0, 148.0, 18.0);
                draw_texture(&pill_input, box_r.x, box_r.y, Color::from_rgba(24, 26, 24, 255));
                let bd = measure_text(&edit_buf, None, 15.0 as u16, 1.0);
                draw_text(&edit_buf, box_r.x + 8.0, box_r.y + 13.5, 15.0, UI_TEXT);
                if (get_time() * 2.0).fract() < 0.5 {
                    draw_rectangle(box_r.x + 9.0 + bd.width, box_r.y + 4.0, 1.0, 11.0, UI_TEXT);
                }
                if get_time() - bad_edit_at < 1.0 {
                    draw_text("invalid", box_r.x + box_r.w - 52.0, box_r.y + 13.5, 13.0,
                              Color::from_rgba(255, 120, 90, 255)); }
                draw_text("Enter: apply | Esc: cancel", box_r.x, TITLE_BAR_H + 324.0, 11.0, HINT); } }


        let ui_free = !picking && !was_picking && !editing && !dd_open;
        for i in 0..8usize {
            let col = i % 4;
            let row = i / 4;
            let swx = 336.0 + col as f32 * 21.0;
            let swy = TITLE_BAR_H + if row == 0 { 315.0 } else { 336.0 };
            let (ccx, ccy) = (swx + 8.0, swy + 8.0);
            let over = ui_free
                && m.x >= swx - 2.0 && m.x <= swx + 18.0
                && m.y >= swy - 2.0 && m.y <= swy + 18.0;

            match swatches[i] {
                Some((sr, sg, sb, _sa)) => {
                    draw_circle(ccx, ccy, 8.0, Color::from_rgba(sr, sg, sb, 255)); }
                None => { draw_circle(ccx, ccy, 8.0, Color::from_rgba(45, 48, 56, 255)); } }
            draw_circle_lines(ccx, ccy, 8.0, 1.0, if over { WHITE } else { BORDER });

            if over && is_mouse_button_pressed(MouseButton::Left) && active.is_none() { swatches[i] = Some((r, g, b, a8));
                save_swatches(&swatches); }
            if over && is_mouse_button_pressed(MouseButton::Right) {
                if let Some((sr, sg, sb, sa)) = swatches[i] {
                    picker.sync_hsv(sr, sg, sb);picker.alpha = sa as f32 / 255.0; } } }

        // ---------- hints (click to hide) ---------
        let hint_defs: [(&str, f32, f32, f32); 4] = [
            ("C - print to console (useless)", 376.0, TITLE_BAR_H + 254.0, 14.0),
            ("click rgba to copy", 376.0, TITLE_BAR_H + 274.0, 14.0),
            ("click a hint to hide", 376.0, TITLE_BAR_H + 294.0, 14.0),
            ("L-click: save | R-click: apply", 336.0, TITLE_BAR_H + 359.0, 10.0),
        ];
        for i in 0..4 {
            if !hints_vis[i] { continue; }
            let (txt, hxp, hyp, hfs) = hint_defs[i];
            let ht = measure_text(txt, None, hfs as u16, 1.0);
            draw_text(txt, hxp, hyp, hfs, HINT);
            let hov = ui_free
                && m.x >= hxp - 2.0 && m.x <= hxp + ht.width + 2.0
                && m.y >= hyp - hfs && m.y <= hyp + 4.0;
            if hov {
                draw_rectangle(hxp, hyp + 3.0, ht.width, 1.0, Color::new(0.55, 0.57, 0.62, 0.6));
                if is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
                    hints_vis[i] = false;
                    save_hints(&hints_vis); } }
        }if hints_vis.iter().any(|v| !*v) {
            let hb = Rect::new(376.0, TITLE_BAR_H + 232.0, 70.0, 16.0);
            let hov = ui_free && hit(hb, m, 0.0);
            draw_texture(&pill_hints, hb.x, hb.y,
                         if hov { Color::from_rgba(60, 64, 74, 255) } else { Color::from_rgba(45, 48, 56, 255) });
            let st = measure_text("show hints", None, 10.0 as u16, 1.0);
            draw_text("show hints", hb.x + (hb.w - st.width) * 0.5, hb.y + 12.0, 10.0, UI_TEXT);
            if hov && is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
                hints_vis = [true; 4];
                save_hints(&hints_vis); } }
        // ---------- screen pick button ----------
        let pick_btn = Rect::new(420.0, TITLE_BAR_H + 334.0, 100.0, 20.0);
        let pick_over = !picking && !was_picking && !editing && !dd_open && hit(pick_btn, m, 0.0);
        draw_texture(&pill_pick, pick_btn.x, pick_btn.y,
                     if pick_over { Color::from_rgba(60, 64, 74, 255) } else { Color::from_rgba(45, 48, 56, 255) });
        let pt = measure_text("PICK SCREEN", None, 14.0 as u16, 1.0);
        draw_text("PICK SCREEN", pick_btn.x + (pick_btn.w - pt.width) * 0.5, pick_btn.y + 15.0, 14.0, UI_TEXT);
        if pick_over && is_mouse_button_pressed(MouseButton::Left) && active.is_none() {
            picking = true;
            saved_picker = Some(picker);
            screen::start_pick();
            screen::overlay_show();
            macroquad::miniquad::window::set_mouse_cursor(
                macroquad::miniquad::CursorIcon::Crosshair, ); }

        if is_key_pressed(KeyCode::C) && !editing { println!("rgba({}, {}, {}, {})  #{:02X}{:02X}{:02X}{:02X}", r, g, b, a8, r, g, b, a8); }

        set_default_camera();
        draw_rectangle(0.0, 0.0, sw, TITLE_BAR_H, BG);
        draw_text(TITLE_TEXT, 12.0, TITLE_BAR_H - 9.0, 15.0, UI_TEXT);


        let (raw_mx, raw_my) = mouse_position();
        let raw_m = if screen::mouse_in_window() {
            Vec2::new(raw_mx, raw_my) } else {
            Vec2::new(-1.0e6, -1.0e6) };
        let btn_ok = !picking && !was_picking && !editing && !dd_open;
        let maxed = screen::is_maximized();
        let btns = [(sw - 3.0 * TITLE_BTN_W, 0u8), (sw - 2.0 * TITLE_BTN_W, 1u8), (sw - TITLE_BTN_W, 2u8), ];
        for (bx, kind) in btns {
            let rect = Rect::new(bx, 0.0, TITLE_BTN_W, TITLE_BAR_H);
            let hov = btn_ok && hit(rect, raw_m, 0.0);
            let (bg, icon) = if hov {
                if kind == 2 {
                    (Color::from_rgba(196, 43, 28, 255), UI_TEXT) } else {
                    (Color::from_rgba(255, 255, 255, 26), UI_TEXT) } } else {
                (Color::from_rgba(0, 0, 0, 0), UI_TEXT) };
            if hov { draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg); }
            let mx2 = rect.x + rect.w * 0.5;
            let my2 = TITLE_BAR_H * 0.5;
            match kind {
                0 => draw_rectangle(mx2 - 5.0, my2, 10.0, 1.0, icon),
                1 => {
                    if maxed {
                        draw_rectangle_lines(mx2 - 1.0, my2 - 5.0, 8.0, 8.0, 1.0, icon);
                        draw_rectangle(mx2 - 5.0, my2 - 2.0, 8.0, 8.0, TITLE_BG);
                        draw_rectangle_lines(mx2 - 5.0, my2 - 2.0, 8.0, 8.0, 1.0, icon); } else {
                        draw_rectangle_lines(mx2 - 4.5, my2 - 4.5, 9.0, 9.0, 1.0, icon); } }
                _ => {
                    draw_line(mx2 - 4.0, my2 - 4.0, mx2 + 4.0, my2 + 4.0, 1.0, icon);
                    draw_line(mx2 + 4.0, my2 - 4.0, mx2 - 4.0, my2 + 4.0, 1.0, icon); } }
            if hov && is_mouse_button_pressed(MouseButton::Left) {
                match kind {
                    0 => screen::minimize_window(),
                    1 => screen::toggle_maximize_window(),
                    _ => screen::close_window(), } } }
        draw_text("v1.0.0",screen_width() / 100.,screen_height() -5.,10.,Color::from_rgba(0,0,0,102));
        next_frame().await;
        let budget = 1.0 / refresh_hz;
        let spent = get_time() - frame_start;
        if spent < budget {
            std::thread::sleep(std::time::Duration::from_secs_f64(budget - spent)); } } }
