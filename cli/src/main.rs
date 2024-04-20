use clap::Parser;

// 파일의 패턴을 탐색하고 패턴이 포함된 라인을 표시함.
#[derive(Parser)]
struct Cli {
    //찾을 문자열
    pattern : String,
    //읽을 파일
    path : std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    
}
