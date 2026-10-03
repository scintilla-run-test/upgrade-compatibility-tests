use std::process::ExitCode;

fn main() -> ExitCode {
    if ores_clis_core::self_update::self_update_requested() {
        ores_clis_core::self_update::run_self_update_cli(
            ores_clis_core::self_update::SelfUpdateConfig::new(
                "ORESoftware",
                "ores-compose",
                "ores-compose",
                env!("CARGO_PKG_VERSION"),
            ),
        );
    }

    ExitCode::SUCCESS
}
