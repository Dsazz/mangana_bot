use async_process::{Command, Stdio};
use futures::io::BufReader as AsyncBufReader;
use futures::prelude::*;
use regex::{Regex, RegexBuilder};

pub async fn get_request(url: &str) -> Result<String, anyhow::Error> {
    let mut command = Command::new("run");
    let command = command.arg("html-extractor");
    let command = command.arg(url);

    let mut child = command
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let mut lines = AsyncBufReader::new(child.stdout.take().unwrap())
        .lines();

    let mut output: String = "".to_string();
    while let Some(line) = lines.next().await {
        output.push_str(line.unwrap().as_str());
    }

    if has_error(output.as_str()) {
        bail!(output.to_string())
    }

    Ok(output)
}

lazy_static! {
    static ref CLI_OUTPUT_EXCEEDED_THE_LIMIT_ERROR: Regex = RegexBuilder::new("request error")
    .multi_line(true)
    .build()
    .unwrap();
}
fn has_error(output: &str) -> bool {
    CLI_OUTPUT_EXCEEDED_THE_LIMIT_ERROR.is_match(output)
}