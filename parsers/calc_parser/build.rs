fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .emit_comments(true)
        .emit_rerun_directives(true)
        .process_current_dir()
        .unwrap();
}
