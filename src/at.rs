use std::fmt;
use std::process::Command;

#[derive(Debug)]
pub enum AtError {
    CommandError(String),
    ParseError(String),
}

impl fmt::Display for AtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AtError::CommandError(ref err) => write!(f, "Command error: {}", err),
            AtError::ParseError(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for AtError {}

/// Schedules a new job with the `at` command using commands from a file.
/// Returns the job ID on success.
pub fn schedule_jobs_from_file(time: &str, file_path: &str) -> Result<String, AtError> {
    // Schedule the script using `at`
    let output = Command::new("at")
        .arg("-f")
        .arg(file_path)
        .arg(time)
        .output()
        .map_err(|e| AtError::CommandError(e.to_string()))?;

    if !output.status.success() {
        return Err(AtError::CommandError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Get the job ID from `atq`
    let output = Command::new("atq")
        .output()
        .map_err(|e| AtError::CommandError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let last_line = stdout
        .lines()
        .last()
        .ok_or_else(|| AtError::ParseError("Failed to read atq output".to_string()))?;
    let job_id = last_line
        .split_whitespace()
        .next()
        .ok_or_else(|| AtError::ParseError("Failed to parse job ID".to_string()))?;

    Ok(job_id.to_string())
}

/// Removes a scheduled job using the job ID.
pub fn remove_job(job_id: &str) -> Result<(), AtError> {
    let output = Command::new("atrm")
        .arg(job_id.to_string())
        .output()
        .map_err(|e| AtError::CommandError(e.to_string()))?;

    if !output.status.success() {
        return Err(AtError::CommandError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}
