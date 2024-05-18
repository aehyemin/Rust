use anyhow::{Context, Result};
use clap::Parser;

// 파일에서 찾을 문자열 탐색과 포함하는 라인 표시
#[derive(Parser)]
struct Cli {
    // 찾는 패턴, 문자열
    pattern: String,
    // 읽을 파일
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
