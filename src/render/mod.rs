mod animation;
mod canvas;
mod celebration;
mod colors;
mod context_menu;
mod font;
mod frame;
mod segment;

use crate::app::AppView;
use crate::model::{ClockSnapshot, CountdownStatus};

const OFF_CYCLE_PHRASES: [&str; 20] = [
    "Acabou, acabou, acabouuuuu...",
    "Descansar, né? Que o cabra não é de ferro.",
    "É hora de dar tchau.",
    "Bom descanso!",
    "Partiu descanso, meus consagrados!",
    "Hoje acabou o expediente, só alegria!",
    "Fechou o expediente, pai!",
    "O guerreiro merece um descanso.",
    "Missão cumprida, tropa!",
    "Pode baixar a guarda, campeão.",
    "A firma fechou, meus queridos!",
    "Trabalhar amanhã? Só amanhã!",
    "Quem é rei nunca perde a majestade.",
    "Agora é só alegria!",
    "Fui de base, até amanhã!",
    "Desligando os trabalhos...",
    "Pode tirar o crachá, chefe.",
    "O expediente pediu arrego.",
    "Acabou o sofrimento, Brasil!",
    "Zerou o game, bora descansar!",
];

pub fn draw(buffer: &mut [u32], view: &AppView) {
    let palette = colors::for_theme(view.ui.theme);

    if let Some(celebration) = view.celebration {
        celebration::draw(buffer, celebration.progress, palette);
        return;
    }

    let segment_shadow = colors::segment_shadow(palette, view.ui.shadow_level);
    canvas::clear(buffer, palette.background);
    frame::draw(buffer, palette);

    font::draw_text_centered(
        buffer,
        &friday_countdown_label(view.clock.friday_countdown.remaining_seconds),
        46,
        1,
        palette.muted,
    );
    font::draw_text_centered(buffer, "HORÁRIO LOCAL", 84, 2, palette.muted);

    segment::draw_large_time(
        buffer,
        view.clock.hour,
        view.clock.minute,
        view.clock.second,
        106,
        172,
        palette,
        segment_shadow,
    );

    let short_year = view.clock.year.rem_euclid(100) as u32;
    let date = format!(
        "DATA  {:02}/{:02}/{:02}",
        view.clock.day, view.clock.month, short_year
    );
    font::draw_text_centered(buffer, &date, 334, 4, palette.highlight);
    font::draw_text_centered(
        buffer,
        status_label(view.clock.countdown.status),
        382,
        2,
        palette.muted,
    );
    if let Some(animation_view) = view.end_of_cycle_animation {
        animation::draw(buffer, animation_view, palette);
    } else {
        segment::draw_countdown(
            buffer,
            view.clock.countdown.remaining_seconds,
            view.clock.countdown.status,
            410,
            palette,
            segment_shadow,
        );
    }
    font::draw_text_centered(buffer, footer_label(view.clock), 514, 2, palette.muted);

    if view.ui.menu.visible {
        context_menu::draw(buffer, view.ui, palette);
    }
}

fn status_label(status: CountdownStatus) -> &'static str {
    match status {
        CountdownStatus::WaitingForStart => "ESPERANDO NOVO CICLO",
        CountdownStatus::Active => "FALTAM",
        CountdownStatus::Finished => "ESPERANDO NOVO CICLO",
    }
}

fn friday_countdown_label(total_seconds: u32) -> String {
    let days = total_seconds / (24 * 3600);
    let hours = (total_seconds % (24 * 3600)) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!(
        "PROXIMA SEXTA 17:00  {:02}D {:02}:{:02}:{:02}",
        days, hours, minutes, seconds
    )
}

fn footer_label(clock: ClockSnapshot) -> &'static str {
    if clock.countdown.status == CountdownStatus::Active {
        return "PARA AS 17:00";
    }

    let status_seed = match clock.countdown.status {
        CountdownStatus::WaitingForStart => 0,
        CountdownStatus::Active => 1,
        CountdownStatus::Finished => 2,
    };
    let seed = (clock.year as u64)
        .wrapping_mul(31)
        .wrapping_add(clock.month as u64)
        .wrapping_mul(31)
        .wrapping_add(clock.day as u64)
        .wrapping_mul(3)
        .wrapping_add(status_seed);

    OFF_CYCLE_PHRASES[(seed % OFF_CYCLE_PHRASES.len() as u64) as usize]
}
