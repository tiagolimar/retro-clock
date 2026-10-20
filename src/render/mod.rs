mod canvas;
mod colors;
mod context_menu;
mod font;
mod frame;
mod segment;

use crate::app::AppView;
use crate::model::CountdownStatus;

pub fn draw(buffer: &mut [u32], view: &AppView) {
    let palette = colors::for_theme(view.ui.theme);
    canvas::clear(buffer, palette.background);
    frame::draw(buffer, palette);

    font::draw_text_centered(buffer, "HORARIO LOCAL", 84, 2, palette.muted);

    segment::draw_large_time(
        buffer,
        view.clock.hour,
        view.clock.minute,
        view.clock.second,
        106,
        172,
        palette,
        view.ui.transparent_sticks_visible,
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
    segment::draw_countdown(
        buffer,
        view.clock.countdown.remaining_seconds,
        410,
        palette,
        view.ui.transparent_sticks_visible,
    );
    font::draw_text_centered(buffer, "PARA AS 17:00", 514, 2, palette.muted);

    if view.ui.menu.visible {
        context_menu::draw(buffer, view.ui, palette);
    }
}

fn status_label(status: CountdownStatus) -> &'static str {
    match status {
        CountdownStatus::WaitingForStart => "AGUARDA 08:00  |  CONTAGEM 08:00 - 17:00",
        CountdownStatus::Active => "FALTAM",
        CountdownStatus::Finished => "ENCERRADA  |  NOVO CICLO AS 08:00",
    }
}
