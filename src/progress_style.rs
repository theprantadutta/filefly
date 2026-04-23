use colorgrad::{Gradient, GradientBuilder, LinearGradient};
use indicatif::{ProgressState, ProgressStyle};

const SUBCELL_CHARS: [char; 9] = [' ', '\u{258F}', '\u{258E}', '\u{258D}', '\u{258C}', '\u{258B}', '\u{258A}', '\u{2589}', '\u{2588}'];
const TICK_CHARS: &str = "\u{280B}\u{2819}\u{2839}\u{2838}\u{283C}\u{2834}\u{2826}\u{2827}\u{2807}\u{280F} ";
const BAR_WIDTH: usize = 28;
const NAME_WIDTH: usize = 36;

const COPY_ICON: &str = "\u{21AA}";
const DELETE_ICON: &str = "\u{2716}";

fn fit_name(name: &str) -> String {
    let count = name.chars().count();
    if count > NAME_WIDTH {
        let truncated: String = name.chars().take(NAME_WIDTH - 1).collect();
        format!("{truncated}\u{2026}")
    } else {
        let pad = NAME_WIDTH - count;
        format!("{name}{}", " ".repeat(pad))
    }
}

pub fn copy_prefix(name: &str) -> String {
    format!("{} {}", COPY_ICON, fit_name(name))
}

pub fn delete_prefix(name: &str) -> String {
    format!("{} {}", DELETE_ICON, fit_name(name))
}

fn build_gradient(stops: &[&str]) -> LinearGradient {
    GradientBuilder::new()
        .html_colors(stops)
        .build::<LinearGradient>()
        .expect("valid gradient stops")
}

fn copy_gradient() -> LinearGradient {
    build_gradient(&["#22D3EE", "#A78BFA", "#F472B6"])
}

fn delete_gradient() -> LinearGradient {
    build_gradient(&["#FB923C", "#F43F5E", "#DB2777"])
}

fn render_bar(state: &ProgressState, w: &mut dyn std::fmt::Write, grad: &LinearGradient) {
    let fraction = state.fraction().clamp(0.0, 1.0);
    let filled_exact = fraction * BAR_WIDTH as f32;
    let filled_full = filled_exact.floor() as usize;
    let partial_frac = filled_exact - filled_full as f32;

    for i in 0..filled_full.min(BAR_WIDTH) {
        let t = (i as f32 + 0.5) / BAR_WIDTH as f32;
        let [r, g, b, _] = grad.at(t).to_rgba8();
        let _ = write!(w, "\x1b[38;2;{};{};{}m\u{2588}", r, g, b);
    }

    if filled_full < BAR_WIDTH {
        let idx = (partial_frac * (SUBCELL_CHARS.len() - 1) as f32).round() as usize;
        if idx > 0 {
            let t = (filled_full as f32 + 0.5) / BAR_WIDTH as f32;
            let [r, g, b, _] = grad.at(t).to_rgba8();
            let _ = write!(w, "\x1b[38;2;{};{};{}m{}", r, g, b, SUBCELL_CHARS[idx]);
            for _ in (filled_full + 1)..BAR_WIDTH {
                let _ = write!(w, "\x1b[38;5;238m\u{2591}");
            }
        } else {
            for _ in filled_full..BAR_WIDTH {
                let _ = write!(w, "\x1b[38;5;238m\u{2591}");
            }
        }
    }

    let _ = write!(w, "\x1b[0m");
}

fn style_with_gradient(gradient: LinearGradient) -> ProgressStyle {
    let template = "{spinner:.magenta} \x1b[38;2;167;139;250m{prefix}\x1b[0m {gradient_bar} \x1b[1m{percent:>3}%\x1b[0m \x1b[2m\u{2502}\x1b[0m {decimal_bytes:>10}/{decimal_total_bytes:<10} \x1b[2m\u{2502}\x1b[0m {decimal_bytes_per_sec:>11} \x1b[2m\u{2502}\x1b[0m eta {eta:>4}";

    ProgressStyle::with_template(template)
        .expect("valid template")
        .with_key("gradient_bar", move |state: &ProgressState, w: &mut dyn std::fmt::Write| {
            render_bar(state, w, &gradient);
        })
        .tick_chars(TICK_CHARS)
}

pub fn copy_style() -> ProgressStyle {
    style_with_gradient(copy_gradient())
}

pub fn delete_style() -> ProgressStyle {
    style_with_gradient(delete_gradient())
}
