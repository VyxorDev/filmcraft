use egui::{Event, Modifiers, PointerButton, Rect, pos2, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use filmcraft_engine::Session;
use filmcraft_ui_egui::{
    FilmcraftApp,
    panels::monitor::{self, Which},
    theme::{self, ThemeKind},
};
use serde_json::json;

fn harness() -> Harness<'static, FilmcraftApp> {
    let mut session = Session::default();
    session.execute("file.openDemoProject", json!({})).unwrap();
    let mut app = FilmcraftApp::new(session);
    app.ui.program.transport_hidden.retain(|command| command != "playback.loop");

    let mut h = Harness::builder().with_size(vec2(250.0, 340.0)).build_ui_state(
        move |ui, app: &mut FilmcraftApp| {
            app.set_theme(ui.ctx(), ThemeKind::Light);
            app.auto.begin_frame();
            let frame = ui.max_rect();
            ui.painter().rect_filled(frame, 0.0, app.tokens.panel_bg);
            let content = Rect::from_min_max(pos2(frame.min.x, frame.min.y + app.tokens.tab_h), frame.max);
            monitor::show(app, ui, content, Which::Program);
        },
        app,
    );
    theme::install(&h.ctx, &h.state().tokens);
    h.run_steps(4);
    h
}

fn element(h: &Harness<'_, FilmcraftApp>, id: &str) -> Rect {
    let element = h.state().auto.elements.iter().find(|element| element.id == id).unwrap_or_else(|| panic!("missing {id}"));
    let [x, y, w, h] = element.rect;
    Rect::from_min_size(pos2(x, y), vec2(w, h))
}

fn click(h: &mut Harness<'_, FilmcraftApp>, id: &str) {
    let pos = element(h, id).center();
    h.event(Event::PointerMoved(pos));
    for pressed in [true, false] {
        h.event(Event::PointerButton { pos, button: PointerButton::Primary, pressed, modifiers: Modifiers::NONE });
        h.step();
    }
    h.run_steps(3);
}

#[test]
fn compact_program_overflow_marks_loop_selected_when_enabled() {
    let mut h = harness();

    click(&mut h, "program.transport.more");
    assert_eq!(h.get_by_label("Loop").accesskit_node().toggled(), Some(egui::accesskit::Toggled::False));

    click(&mut h, "program.transport.playback.loop");
    assert!(h.state().playback.looping);

    click(&mut h, "program.transport.more");
    assert_eq!(h.get_by_label("Loop").accesskit_node().toggled(), Some(egui::accesskit::Toggled::True));
}
