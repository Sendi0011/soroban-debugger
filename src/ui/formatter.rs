use colored::Colorize;

/// Pretty printing utilities for debugger output
pub struct Formatter;

impl Formatter {
    /// Format a value for display
    pub fn format_value(value: &str) -> String {
        // TODO: Add better formatting for different types
        value.to_string()
    }

    /// Format storage key-value pair
    pub fn format_storage_entry(key: &str, value: &str) -> String {
        format!("{} = {}", key, value)
    }

    /// Format a function call
    pub fn format_function_call(name: &str, args: Option<&str>) -> String {
        if let Some(args) = args {
            format!("{}({})", name, args)
        } else {
            format!("{}()", name)
        }
    }

    /// Format budget information
    pub fn format_budget(cpu: u64, cpu_limit: u64, mem: u64, mem_limit: u64) -> String {
        format!(
            "CPU: {}/{} ({:.1}%) | Memory: {}/{} bytes ({:.1}%)",
            cpu,
            cpu_limit,
            (cpu as f64 / cpu_limit as f64) * 100.0,
            mem,
            mem_limit,
            (mem as f64 / mem_limit as f64) * 100.0
        )
    }

    /// Format an informational message in blue.
    pub fn info(message: impl AsRef<str>) -> String {
        message.as_ref().blue().to_string()
    }

    /// Format a success message in green.
    pub fn success(message: impl AsRef<str>) -> String {
        message.as_ref().green().to_string()
    }

    /// Format a warning message in yellow.
    pub fn warning(message: impl AsRef<str>) -> String {
        message.as_ref().yellow().to_string()
    }

    /// Format an error message in red.
    pub fn error(message: impl AsRef<str>) -> String {
        message.as_ref().red().to_string()
    }

    /// Configure whether ANSI colors are enabled.
    /// Honors NO_COLOR automatically via `colored` and allows explicit override.
    pub fn configure_colors(enable: bool) {
        colored::control::set_override(enable);
    }

    /// Auto-configure color output based on environment.
    /// If NO_COLOR is set, colors are disabled. Otherwise default terminal
    /// detection is preserved for compatibility with non-color terminals.
    pub fn configure_colors_from_env() {
        let no_color = std::env::var_os("NO_COLOR").is_some();
        if no_color {
            colored::control::set_override(false);
        }
    }
}
