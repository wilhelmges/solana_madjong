use macroquad::prelude::*;

const FG: Color = Color::new(1.0, 1.0, 1.0, 0.92);
const FG_DARK: Color = Color::new(0.0, 0.0, 0.0, 0.45);

fn px(cx: f32, cy: f32, s: f32, nx: f32, ny: f32) -> (f32, f32) {
    (cx + nx * s, cy + ny * s)
}

fn thick(s: f32) -> f32 {
    (s * 0.16).clamp(1.2, 3.2)
}

fn seg(cx: f32, cy: f32, s: f32, ax: f32, ay: f32, bx: f32, by: f32, color: Color) {
    let (x1, y1) = px(cx, cy, s, ax, ay);
    let (x2, y2) = px(cx, cy, s, bx, by);
    draw_line(x1, y1, x2, y2, thick(s), color);
}

fn arrow_seg(cx: f32, cy: f32, s: f32, ax: f32, ay: f32, bx: f32, by: f32, color: Color) {
    seg(cx, cy, s, ax, ay, bx, by, color);
    let (x1, y1) = px(cx, cy, s, ax, ay);
    let (x2, y2) = px(cx, cy, s, bx, by);
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = dx.hypot(dy);
    if len < 0.001 {
        return;
    }
    let w = thick(s) * 1.7;
    let ux = dx / len;
    let uy = dy / len;
    let bx2 = x2 - ux * w * 1.4;
    let by2 = y2 - uy * w * 1.4;
    let nx = -uy;
    let ny = ux;
    let (hx1, hy1) = (bx2 + nx * w * 0.9, by2 + ny * w * 0.9);
    let (hx2, hy2) = (bx2 - nx * w * 0.9, by2 - ny * w * 0.9);
    draw_triangle(Vec2::new(x2, y2), Vec2::new(hx1, hy1), Vec2::new(hx2, hy2), color);
}

fn disc(cx: f32, cy: f32, s: f32, x: f32, y: f32, r: f32, color: Color) {
    let (dx, dy) = px(cx, cy, s, x, y);
    draw_circle(dx, dy, r * s, color);
}

fn ring(cx: f32, cy: f32, s: f32, x: f32, y: f32, r: f32, color: Color) {
    let (dx, dy) = px(cx, cy, s, x, y);
    draw_circle_lines(dx, dy, r * s, thick(s), color);
}

fn box_f(cx: f32, cy: f32, s: f32, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    let (a, b) = px(cx, cy, s, x1, y1);
    let (c, d) = px(cx, cy, s, x2, y2);
    draw_rectangle(a.min(c), b.min(d), (a - c).abs(), (b - d).abs(), color);
}

fn box_l(cx: f32, cy: f32, s: f32, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    let (a, b) = px(cx, cy, s, x1, y1);
    let (c, d) = px(cx, cy, s, x2, y2);
    draw_rectangle_lines(
        a.min(c),
        b.min(d),
        (a - c).abs(),
        (b - d).abs(),
        thick(s),
        color,
    );
}

fn tri(cx: f32, cy: f32, s: f32, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), color: Color) {
    let (x1, y1) = px(cx, cy, s, p1.0, p1.1);
    let (x2, y2) = px(cx, cy, s, p2.0, p2.1);
    let (x3, y3) = px(cx, cy, s, p3.0, p3.1);
    draw_triangle(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), color);
}

fn ell(cx: f32, cy: f32, s: f32, x: f32, y: f32, rx: f32, ry: f32, color: Color) {
    let (dx, dy) = px(cx, cy, s, x, y);
    draw_ellipse(dx, dy, rx * s, ry * s, 0.0, color);
}

pub fn draw_tile_icon(name: &str, cx: f32, cy: f32, s: f32) {
    // shadow of icon
    let scx = cx + s * 0.06;
    let scy = cy + s * 0.08;
    draw_icon_inner(name, scx, scy, s, FG_DARK);
    draw_icon_inner(name, cx, cy, s, FG);
}

fn draw_icon_inner(name: &str, cx: f32, cy: f32, s: f32, color: Color) {
    match name {
        // ---------------- Tokens ----------------
        "SOL" => {
            ring(cx, cy, s, 0.0, 0.0, 0.72, color);
            seg(cx, cy, s, -0.68, 0.0, 0.68, 0.0, color);
        }
        "USDC" => {
            ring(cx, cy, s, 0.0, 0.0, 0.75, color);
            ring(cx, cy, s, 0.0, 0.0, 0.35, color);
        }
        "USDT" => {
            ring(cx, cy, s, 0.0, 0.0, 0.72, color);
            seg(cx, cy, s, -0.3, -0.48, -0.3, 0.48, color);
            seg(cx, cy, s, -0.55, -0.48, -0.05, -0.48, color);
        }
        "BONK" => {
            disc(cx, cy, s, -0.35, 0.12, 0.2, color);
            disc(cx, cy, s, 0.35, 0.12, 0.2, color);
            disc(cx, cy, s, -0.42, -0.24, 0.2, color);
            disc(cx, cy, s, 0.0, -0.35, 0.2, color);
            disc(cx, cy, s, 0.42, -0.24, 0.2, color);
            ell(cx, cy, s, 0.0, 0.34, 0.42, 0.33, color);
        }
        "WIF" => {
            box_f(cx, cy, s, -0.5, -0.35, 0.5, -0.05, color);
            box_f(cx, cy, s, -0.72, 0.28, 0.72, 0.45, color);
            box_f(cx, cy, s, 0.72, 0.28, 0.62, 0.12, color);
            box_f(cx, cy, s, -0.72, 0.28, -0.62, 0.12, color);
        }
        "JUP" => {
            ring(cx, cy, s, 0.0, 0.0, 0.72, color);
            seg(cx, cy, s, -0.55, -0.42, 0.4, 0.55, color);
            seg(cx, cy, s, -0.25, -0.55, 0.55, -0.2, color);
        }
        "RAY" => {
            disc(cx, cy, s, 0.0, 0.0, 0.34, color);
            for k in 0..8 {
                let a = k as f32 * std::f32::consts::TAU / 8.0;
                let (dx, dy) = (a.cos(), a.sin());
                seg(
                    cx,
                    cy,
                    s,
                    dx * 0.55,
                    dy * 0.55,
                    dx * 0.85,
                    dy * 0.85,
                    color,
                );
            }
        }
        "ORCA" => {
            ell(cx, cy, s, 0.0, 0.0, 0.72, 0.38, color);
            box_f(cx, cy, s, 0.15, -0.38, 0.7, -0.12, color);
            tri(cx, cy, s, (0.62, 0.05), (0.5, -0.4), (0.75, 0.22), color);
            disc(cx, cy, s, -0.42, -0.08, 0.12, color);
            box_f(cx, cy, s, -0.72, -0.05, 0.15, 0.05, color);
        }
        "PYTH" => {
            ell(cx, cy, s, 0.0, 0.0, 0.68, 0.42, color);
            disc(cx, cy, s, 0.12, 0.0, 0.2, color);
            disc(cx, cy, s, -0.4, -0.2, 0.08, color);
        }
        "JTO" => {
            box_f(cx, cy, s, -0.5, -0.5, 0.5, -0.12, color);
            box_f(cx, cy, s, -0.56, -0.06, 0.56, 0.28, color);
            box_f(cx, cy, s, -0.5, 0.34, 0.5, 0.62, color);
        }
        // ---------------- DeFi ----------------
        "Swap" => {
            arrow_seg(cx, cy, s, -0.6, 0.5, -0.6, -0.3, color);
            seg(cx, cy, s, -0.6, -0.3, 0.6, -0.3, color);
            arrow_seg(cx, cy, s, 0.6, -0.3, 0.6, 0.5, color);
        }
        "Stake" => {
            arrow_seg(cx, cy, s, 0.0, 0.55, 0.0, -0.35, color);
            box_f(cx, cy, s, -0.6, -0.45, 0.6, -0.28, color);
            box_f(cx, cy, s, -0.42, -0.28, 0.42, -0.05, color);
        }
        "Lend" => {
            tri(cx, cy, s, (-0.65, -0.45), (0.65, -0.45), (0.0, -0.7), color);
            box_f(cx, cy, s, -0.55, -0.4, 0.55, 0.6, color);
            box_f(cx, cy, s, -0.4, 0.6, -0.4, -0.35, color);
            box_f(cx, cy, s, -0.14, 0.6, -0.14, -0.35, color);
            box_f(cx, cy, s, 0.12, 0.6, 0.12, -0.35, color);
        }
        "Yield" => {
            seg(cx, cy, s, 0.0, 0.62, 0.0, -0.1, color);
            tri(cx, cy, s, (0.0, -0.1), (0.45, -0.35), (0.0, -0.6), color);
            tri(cx, cy, s, (0.0, -0.1), (-0.45, -0.35), (0.0, -0.6), color);
        }
        "Bridge" => {
            seg(cx, cy, s, -0.75, -0.1, -0.75, 0.6, color);
            seg(cx, cy, s, 0.75, -0.1, 0.75, 0.6, color);
            seg(cx, cy, s, -0.72, 0.05, 0.72, 0.05, color);
            seg(cx, cy, s, -0.5, 0.05, 0.5, -0.65, color);
            seg(cx, cy, s, 0.5, -0.65, -0.5, 0.05, color);
        }
        "DAO" => {
            ring(cx, cy, s, -0.45, -0.2, 0.22, color);
            ring(cx, cy, s, 0.45, -0.2, 0.22, color);
            ring(cx, cy, s, 0.0, 0.35, 0.22, color);
            seg(cx, cy, s, -0.28, -0.05, -0.25, 0.2, color);
            seg(cx, cy, s, 0.28, -0.05, 0.25, 0.2, color);
            seg(cx, cy, s, -0.5, -0.05, -0.45, -0.1, color);
        }
        "NFT" => {
            box_l(cx, cy, s, -0.65, -0.65, 0.65, 0.65, color);
            box_l(cx, cy, s, -0.35, -0.35, 0.35, 0.35, color);
            tri(cx, cy, s, (-0.25, 0.2), (0.0, -0.2), (0.2, 0.2), color);
        }
        "Mint" => {
            ring(cx, cy, s, 0.0, -0.1, 0.55, color);
            seg(cx, cy, s, -0.3, -0.1, -0.3, 0.1, color);
            seg(cx, cy, s, -0.2, -0.1, -0.2, 0.1, color);
            seg(cx, cy, s, -0.1, -0.1, -0.1, 0.1, color);
            seg(cx, cy, s, 0.0, -0.1, 0.0, 0.1, color);
            seg(cx, cy, s, -0.5, 0.1, 0.5, 0.1, color);
            seg(cx, cy, s, -0.5, -0.5, 0.0, -0.5, color);
        }
        "Pool" => {
            tri(cx, cy, s, (-0.35, 0.3), (0.35, 0.3), (0.0, -0.5), color);
            disc(cx, cy, s, 0.0, 0.28, 0.34, color);
        }
        "Farm" => {
            disc(cx, cy, s, 0.0, -0.45, 0.16, color);
            for rx in [-0.5f32, 0.0, 0.5] {
                tri(
                    cx,
                    cy,
                    s,
                    (rx - 0.12, 0.1),
                    (rx + 0.12, 0.1),
                    (rx, -0.12),
                    color,
                );
                tri(
                    cx,
                    cy,
                    s,
                    (rx - 0.12, 0.45),
                    (rx + 0.12, 0.45),
                    (rx, 0.2),
                    color,
                );
            }
        }
        // ---------------- Eco ----------------
        "Saga" => {
            box_l(cx, cy, s, -0.5, -0.7, 0.5, 0.55, color);
            box_l(cx, cy, s, -0.35, -0.5, 0.35, 0.3, color);
            disc(cx, cy, s, 0.0, 0.45, 0.07, color);
        }
        "Firedancer" => {
            tri(cx, cy, s, (0.0, -0.68), (0.42, 0.1), (0.0, 0.62), color);
            tri(cx, cy, s, (0.0, -0.68), (-0.34, -0.05), (0.0, 0.62), color);
            tri(cx, cy, s, (0.0, -0.1), (0.2, 0.3), (0.0, 0.45), color);
        }
        "Token2022" => {
            box_l(cx, cy, s, -0.5, -0.5, 0.2, 0.5, color);
            box_l(cx, cy, s, -0.15, -0.15, 0.5, 0.5, color);
            seg(cx, cy, s, 0.0, -0.3, 0.0, 0.3, color);
            seg(cx, cy, s, -0.3, 0.0, 0.3, 0.0, color);
        }
        "Program" => {
            seg(cx, cy, s, -0.7, 0.55, -0.15, 0.0, color);
            seg(cx, cy, s, -0.7, -0.55, -0.15, 0.0, color);
            seg(cx, cy, s, 0.7, 0.55, 0.15, 0.0, color);
            seg(cx, cy, s, 0.7, -0.55, 0.15, 0.0, color);
            seg(cx, cy, s, -0.05, -0.55, 0.12, -0.55, color);
            seg(cx, cy, s, -0.05, 0.55, 0.12, 0.55, color);
        }
        "Account" => {
            disc(cx, cy, s, 0.0, -0.32, 0.3, color);
            ell(cx, cy, s, 0.0, 0.55, 0.6, 0.4, color);
        }
        "Block" => {
            tri(cx, cy, s, (0.0, -0.55), (0.6, -0.28), (0.0, 0.0), color);
            box_f(cx, cy, s, -0.58, -0.26, 0.58, 0.6, color);
            tri(cx, cy, s, (-0.58, -0.26), (0.0, -0.55), (0.0, 0.0), color);
            seg(cx, cy, s, -0.58, -0.26, 0.58, -0.26, color);
            seg(cx, cy, s, 0.58, -0.26, 0.6, -0.28, color);
        }
        "Vote" => {
            box_l(cx, cy, s, -0.6, -0.65, 0.6, 0.5, color);
            seg(cx, cy, s, -0.42, -0.15, -0.05, 0.2, color);
            seg(cx, cy, s, -0.05, 0.2, 0.42, -0.3, color);
        }
        "Wallet" => {
            box_f(cx, cy, s, -0.68, -0.65, 0.68, 0.55, color);
            box_l(cx, cy, s, -0.5, -0.5, 0.5, 0.4, color);
            disc(cx, cy, s, 0.45, 0.0, 0.1, color);
        }
        // ---------------- Validators ----------------
        "Jito" => {
            tri(cx, cy, s, (0.0, -0.7), (0.25, -0.1), (0.0, -0.05), color);
            tri(cx, cy, s, (0.0, -0.05), (-0.25, -0.1), (0.12, 0.45), color);
        }
        "Marinade" => {
            seg(cx, cy, s, 0.0, -0.4, 0.0, 0.55, color);
            tri(cx, cy, s, (0.0, -0.4), (-0.55, 0.1), (0.0, 0.1), color);
            seg(cx, cy, s, -0.6, 0.55, 0.6, 0.55, color);
        }
        "Lido" => {
            box_f(cx, cy, s, -0.5, 0.0, 0.5, 0.6, color);
            box_f(cx, cy, s, -0.35, -0.45, 0.35, -0.1, color);
            seg(cx, cy, s, -0.62, -0.3, 0.62, -0.3, color);
            seg(cx, cy, s, 0.0, -0.45, 0.2, -0.62, color);
            disc(cx, cy, s, 0.28, -0.62, 0.09, color);
        }
        "Rocket" => {
            tri(cx, cy, s, (0.0, -0.7), (0.3, -0.2), (-0.3, -0.2), color);
            ring(cx, cy, s, 0.0, 0.05, 0.18, color);
            tri(cx, cy, s, (-0.3, -0.25), (-0.55, -0.15), (-0.3, 0.05), color);
            tri(cx, cy, s, (0.3, -0.25), (0.55, -0.15), (0.3, 0.05), color);
            tri(cx, cy, s, (0.0, 0.35), (0.2, 0.2), (-0.2, 0.2), color);
            tri(cx, cy, s, (0.0, 0.35), (0.3, 0.65), (-0.3, 0.65), color);
        }
        "Figment" => {
            tri(cx, cy, s, (0.0, -0.65), (0.55, 0.0), (0.0, -0.05), color);
            tri(cx, cy, s, (0.0, -0.05), (0.55, 0.0), (0.0, 0.6), color);
            tri(cx, cy, s, (0.0, -0.05), (-0.55, 0.0), (0.0, 0.6), color);
            seg(cx, cy, s, 0.0, -0.65, 0.0, 0.6, color);
        }
        "Anza" => {
            seg(cx, cy, s, -0.55, 0.62, -0.55, -0.6, color);
            tri(cx, cy, s, (-0.55, -0.6), (0.6, -0.3), (-0.55, 0.0), color);
            seg(cx, cy, s, -0.25, -0.1, 0.55, 0.62, color);
        }
        "Triton" => {
            seg(cx, cy, s, 0.0, -0.68, 0.0, 0.6, color);
            seg(cx, cy, s, -0.35, -0.45, -0.35, -0.05, color);
            seg(cx, cy, s, 0.35, -0.45, 0.35, -0.05, color);
            tri(cx, cy, s, (-0.35, -0.45), (-0.1, -0.3), (-0.35, -0.18), color);
            tri(cx, cy, s, (0.35, -0.45), (0.1, -0.3), (0.35, -0.18), color);
            tri(cx, cy, s, (0.0, -0.68), (0.14, -0.5), (-0.14, -0.5), color);
            seg(cx, cy, s, -0.5, 0.6, 0.5, 0.6, color);
        }
        "Helius" => {
            disc(cx, cy, s, 0.0, 0.0, 0.34, color);
            for k in 0..8 {
                let a = k as f32 * std::f32::consts::TAU / 8.0;
                let (c2, e2) = (a.cos(), a.sin());
                seg(
                    cx,
                    cy,
                    s,
                    c2 * 0.5,
                    e2 * 0.5,
                    c2 * 0.82,
                    e2 * 0.82,
                    color,
                );
            }
        }
        _ => {}
    }
}