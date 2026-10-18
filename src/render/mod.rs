mod canvas;
mod colors;
mod context_menu;
mod font;
mod frame;
mod segment;

use crate::app::AppView;
use crate::model::CountdownStatus;

pub fn draw(buffer: &mut [u32], view: &AppView) {
    canvas::clear(buffer);
    frame::draw(buffer);

    font::draw_text_centered(buffer, "HORARIO LOCAL", 84, 2, colors::GREEN_DIM);

    segment::draw_large_time(
        buffer,
        view.clock.hour,
        view.clock.minute,
        view.clock.second,
        106,
        172,
    );

    let short_year = view.clock.year.rem_euclid(100) as u32;
    let date = format!(
        "DATA  {:02}/{:02}/{:02}",
        view.clock.day, view.clock.month, short_year
    );
    font::draw_text_centered(buffer, &date, 334, 4, colors::GREEN_BRIGHT);
    font::draw_text_centered(buffer, status_label(view.clock.countdown.status), 382, 2, colors::GREEN_DIM);
    segment::draw_countdown(buffer, view.clock.countdown.remaining_seconds, 410);
    font::draw_text_centered(buffer, "Acessar menu com botão direito", 514, 2, colors::GREEN_DIM);

    if view.ui.menu.visible {
        context_menu::draw(
            buffer,
            view.ui.menu,
            view.ui.fullscreen_hover,
            view.ui.restore_hover,
            view.ui.close_hover,
        );
    }
}

fn status_label(status: CountdownStatus) -> &'static str {
    match status {
        CountdownStatus::WaitingForStart => "AGUARDA 08:00  |  CONTAGEM 08:00 - 17:00",
        CountdownStatus::Active => "CONTAGEM ATE 17:00",
        CountdownStatus::Finished => "ENCERRADA  |  NOVO CICLO AS 08:00",
    }
}

