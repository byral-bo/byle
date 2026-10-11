// Byle UI Integration Tests
// Tests that .byx files compile correctly and generate expected JS

use byc_ui::backend::UiBackend;
use byc_ui::backend::web::WebBackend;
use byc_ui::resolver::build_multifile_program;
use std::fs;
use std::path::Path;

fn test_byx_file(path: &str, expected_patterns: &[&str]) {
    let source =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    // Parse + resolve in one step
    let program = build_multifile_program(&source, Path::new(path))
        .unwrap_or_else(|e| panic!("Failed to build program for {}: {}", path, e));

    // Generate web output
    let backend = WebBackend::new();
    let output = backend.generate(&program);

    assert!(!output.files.is_empty(), "No output files for {}", path);
    let js = &output.files[0].content;

    // Check expected patterns
    for pattern in expected_patterns {
        assert!(
            js.contains(pattern),
            "Expected pattern '{}' not found in generated JS for {}\nJS snippet (first 500 chars):\n{}",
            pattern,
            path,
            &js[..js.len().min(500)]
        );
    }
}

#[test]
fn test_binding() {
    test_byx_file(
        "tests/ui/binding.byx",
        &["state.set('name'", "state.get('name'"],
    );
}

#[test]
fn test_form_model() {
    test_byx_file("tests/ui/form_model.byx", &["model", "field", "submit"]);
}

#[test]
fn test_events() {
    test_byx_file(
        "tests/ui/events.byx",
        &[
            "click",
            "mouseenter",
            "mouseleave",
            "touchstart",
            "touchend",
            "keydown",
            "keyup",
            "focus",
            "blur",
        ],
    );
}

#[test]
fn test_lifecycle() {
    test_byx_file(
        "tests/ui/lifecycle.byx",
        &[
            "on_created",
            "on_mounted",
            "on_updated",
            "on_unmounted",
            "state.onAnyChange",
        ],
    );
}

#[test]
fn test_conditional() {
    test_byx_file("tests/ui/conditional.byx", &["if (", "Visible", "Hidden"]);
}

#[test]
fn test_for_loop() {
    test_byx_file("tests/ui/for_loop.byx", &["for (const", "items"]);
}

#[test]
fn test_styles() {
    test_byx_file(
        "tests/ui/styles.byx",
        &[
            "'Primary'",
            "'Secondary'",
            "'Title'",
            "background",
            "borderRadius",
            "fontSize",
            "applyStyle",
        ],
    );
}

#[test]
fn test_image() {
    test_byx_file("tests/ui/image.byx", &["lazy", "https://picsum.photos/200"]);
}

#[test]
fn test_virtual_list() {
    test_byx_file(
        "tests/ui/virtual_list.byx",
        &["createVirtualList", "items.length"],
    );
}

#[test]
fn test_inputs() {
    test_byx_file("tests/ui/inputs.byx", &["Binding.twoWay", "twoWay"]);
}

#[test]
fn test_transitions() {
    test_byx_file("tests/ui/transitions.byx", &["transition", "hovered"]);
}

#[test]
fn test_file_picker() {
    test_byx_file(
        "tests/ui/file_picker.byx",
        &["type = 'file'", "accept", "arrayBuffer"],
    );
}

#[test]
fn test_routing() {
    test_byx_file(
        "tests/ui/routing.byx",
        &["createRouter", "route", "__BYLE_ROUTES"],
    );
}

#[test]
fn test_animations() {
    test_byx_file(
        "tests/ui/animations.byx",
        &["FadeIn", "keyframes", "duration"],
    );
}

/// Test that ALL .byx files in tests/ui/ can at least compile
#[test]
fn test_all_byx_files_compile() {
    let dir = fs::read_dir("tests/ui").expect("tests/ui/ directory not found");

    let mut count = 0;
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "byx") {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", path, e));

            // Parse + resolve
            let program = build_multifile_program(&source, &path)
                .unwrap_or_else(|e| panic!("Failed to build program for {:?}: {}", path, e));

            // Generate
            let backend = WebBackend::new();
            let output = backend.generate(&program);

            assert!(!output.files.is_empty(), "No output for {:?}", path);
            assert!(output.html_shell.is_some(), "No HTML shell for {:?}", path);

            count += 1;
        }
    }

    assert!(
        count >= 10,
        "Expected at least 10 .byx test files, found {}",
        count
    );
}
