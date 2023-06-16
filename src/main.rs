use arboard::Clipboard;
use cursive::{
    event::Event,
    theme::{BaseColor::*, Color::*, PaletteColor::*, *},
    traits::*,
    views::{LinearLayout, OnEventView, Panel, ResizedView, TextArea, TextView},
};
use prql_compiler::Options;
use std::sync::{Arc, Mutex};

fn main() {
    let clipboard = Arc::new(Mutex::new(Clipboard::new().unwrap()));
    let mut palette = Palette::default();
    let borders = BorderStyle::Simple;

    palette[Background] = TerminalDefault;
    palette[View] = TerminalDefault;
    palette[Primary] = TerminalDefault;
    palette[Secondary] = Dark(Blue);
    palette[TitlePrimary] = Dark(Red);
    palette[HighlightText] = Dark(White);
    palette[Highlight] = Dark(Red);
    palette[HighlightInactive] = Dark(Blue);
    let theme = Theme {
        shadow: false,
        palette,
        borders,
    };

    let mut siv = cursive::default();

    siv.set_theme(theme);

    let compiled_output = Panel::new(TextView::new("Compiled Output").full_height())
        .title("Compiled Output")
        .with_name("compiled_output");
    let text_editor = Panel::new(TextArea::new().full_height())
        .title("Text Editor")
        .with_name("text_editor");

    let linear_layout = LinearLayout::horizontal()
        .child(text_editor)
        .child(compiled_output);

    let inner_clipboard = Arc::clone(&clipboard);
    let linear_layout = OnEventView::new(linear_layout)
        .on_event(Event::CtrlChar('s'), |v| {
            let text_editor = v
                .find_name::<Panel<ResizedView<TextArea>>>("text_editor")
                .unwrap();
            let prql_code = text_editor.get_inner().get_inner().get_content();
            let compiled_code = prql_compiler::compile(prql_code, &Options::default())
                .unwrap_or_else(|error| format!("Error: {}", error));
            let mut compiled_output = v
                .find_name::<Panel<ResizedView<TextView>>>("compiled_output")
                .unwrap();
            compiled_output
                .get_inner_mut()
                .get_inner_mut()
                .set_content(compiled_code);
        })
        .on_event(Event::CtrlChar('l'), move |v| {
            let compiled_output = v
                .find_name::<Panel<ResizedView<TextView>>>("compiled_output")
                .unwrap();
            let compiled_code = compiled_output.get_inner().get_inner().get_content();
            inner_clipboard
                .lock()
                .unwrap()
                .set_text(compiled_code.source())
                .unwrap();
        });

    siv.add_layer(linear_layout);

    siv.run();
}
