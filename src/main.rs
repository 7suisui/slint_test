slint::include_modules!();

fn main() {
    let app_win = AppWindow::new().expect("Failed to create AppWindow");
    app_win.run().unwrap();
}

