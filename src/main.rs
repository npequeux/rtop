mod cli;
mod config;
mod error;
mod export;
mod monitor;
mod ui;
mod utils;
mod graphics;
mod theme;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{error, info};
use tracing_subscriber;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup logging
    if cli.verbose > 0 {
        let level = match cli.verbose {
            1 => tracing::Level::INFO,
            2 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        };
        tracing_subscriber::fmt()
            .with_max_level(level)
            .init();
    }

    // Handle subcommands
    if let Some(command) = cli.command {
        return handle_command(command);
    }

    // Generate default config if requested
    if cli.generate_config {
        Config::create_default_config()?;
        let config_path = Config::config_path()?;
        println!("Created default config at: {}", config_path.display());
        return Ok(());
    }

    // Load configuration
    let config = if let Some(config_path) = cli.config {
        let contents = std::fs::read_to_string(&config_path)?;
        toml::from_str(&contents)?
    } else {
        Config::load().unwrap_or_default()
    };

    // Handle export mode
    if let Some(export_path) = cli.export {
        info!("Exporting metrics to: {}", export_path.display());
        return export_and_exit(&export_path, &cli.format, &config);
    }

    // Setup signal handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // Calculate run duration
    let run_duration = if let Some(duration_str) = &cli.duration {
        Some(Cli::parse_duration(duration_str)?)
    } else {
        None
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = ui::App::new(config.clone());
    
    // Apply CLI overrides
    if cli.minimal {
        app.set_minimal_mode(true);
    }
    if cli.no_color {
        app.set_color_mode(false);
    }

    // Main loop
    let start_time = Instant::now();
    let result = run_app(&mut terminal, &mut app, running, run_duration, start_time);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        error!("Error: {:?}", err);
        eprintln!("Error: {:?}", err);
    }

    info!("rtop exited successfully");
    Ok(())
}

fn handle_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::ShowConfig => {
            let config = Config::load().unwrap_or_default();
            let config_str = toml::to_string_pretty(&config)?;
            println!("{}", config_str);
        }
        Commands::InitConfig => {
            Config::create_default_config()?;
            let config_path = Config::config_path()?;
            println!("Created default config at: {}", config_path.display());
        }
        Commands::Export { output, format } => {
            let config = Config::load().unwrap_or_default();
            return export_and_exit(&output, &format, &config);
        }
    }
    Ok(())
}

fn export_and_exit(
    path: &std::path::Path,
    format: &str,
    config: &Config,
) -> anyhow::Result<()> {
    let mut app = ui::App::new(config.clone());
    app.update();
    
    // Give monitors time to collect data
    std::thread::sleep(Duration::from_millis(500));
    app.update();

    let metrics = app.collect_metrics();

    match format {
        "json" => metrics.export_json(path)?,
        "csv" => metrics.export_csv(path)?,
        _ => anyhow::bail!("Unsupported format: {}", format),
    }

    println!("Exported metrics to: {}", path.display());
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut ui::App,
    running: Arc<AtomicBool>,
    run_duration: Option<Duration>,
    start_time: Instant,
) -> io::Result<()> {
    loop {
        // Check if we should exit
        if !running.load(Ordering::SeqCst) {
            info!("Received interrupt signal, exiting...");
            break;
        }

        // Check run duration
        if let Some(duration) = run_duration {
            if start_time.elapsed() >= duration {
                info!("Run duration reached, exiting...");
                break;
            }
        }

        // Update monitors
        app.update();

        // Draw UI
        terminal.draw(|f| app.draw(f))?;

        // Handle input
        if app.handle_input()? {
            break;
        }
    }
    Ok(())
}
