use bstr::ByteSlice;
use codevis::render;
use std::path::Path;
use std::sync::atomic::AtomicBool;

#[test]
fn various_renders() {
    let (paths, ignored) = codevis::unicode_content(
        Path::new("./src/"),
        &[],
        prodash::progress::Discard,
        &AtomicBool::default(),
    )
    .unwrap();
    assert_eq!(ignored, 0, "no ignore pattern configured");

    let theme = "Solarized (dark)";
    let mut opts = render::Options {
        column_width: 100,
        line_height: 1,
        target_aspect_ratio: 0.0,
        plain: false,
        highlight_truncated_lines: true,
        display_to_be_processed_file: false,
        fg_color: codevis::render::FgColor::Style,
        bg_color: codevis::render::BgColor::Style,
        color_modulation: 0.2,
        threads: 1,
        theme,
        dont_force_full_columns: true,
        ignore_files_without_syntax: true,
    };
    codevis::render(
        paths.clone(),
        prodash::progress::Discard,
        &AtomicBool::default(),
        opts,
    )
    .unwrap();

    opts.dont_force_full_columns = false;
    opts.ignore_files_without_syntax = false;
    opts.line_height = 2;
    opts.highlight_truncated_lines = false;
    opts.fg_color = codevis::render::FgColor::StyleAsciiBrightness;
    opts.bg_color = codevis::render::BgColor::HelixEditor;
    opts.plain = true;
    opts.threads = 3;
    opts.target_aspect_ratio = 16.0 / 9.0;

    codevis::render(
        paths.clone(),
        prodash::progress::Discard,
        &AtomicBool::default(),
        opts,
    )
    .unwrap();

    opts.line_height = 2;
    codevis::render(
        paths,
        prodash::progress::Discard,
        &AtomicBool::default(),
        opts,
    )
    .unwrap();
}

#[test]
fn multi_threading_produces_same_result_as_single_threaded_mode() {
    let (paths, ignored) = codevis::unicode_content(
        Path::new("./src/"),
        &[],
        prodash::progress::Discard,
        &AtomicBool::default(),
    )
    .unwrap();
    assert_eq!(ignored, 0, "no ignore pattern configured");

    let theme = "Solarized (dark)";
    let mut opts = render::Options {
        column_width: 100,
        line_height: 1,
        target_aspect_ratio: 0.0,
        highlight_truncated_lines: false,
        display_to_be_processed_file: true,
        plain: true,
        fg_color: codevis::render::FgColor::Style,
        bg_color: codevis::render::BgColor::Style,
        threads: 1,
        theme,
        color_modulation: 0.2,
        dont_force_full_columns: true,
        ignore_files_without_syntax: true,
    };
    let expected = codevis::render(
        paths.clone(),
        prodash::progress::Discard,
        &AtomicBool::default(),
        opts,
    )
    .unwrap();

    opts.threads = 2;
    let actual = codevis::render(
        paths.clone(),
        prodash::progress::Discard,
        &AtomicBool::default(),
        opts,
    )
    .unwrap();
    assert!(
        actual.as_bytes() == expected.as_bytes(),
        "multi-threaded version should be pixel-perfect"
    );
}
