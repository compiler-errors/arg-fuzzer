use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Result;
use rand::{prelude::SliceRandom, thread_rng, Rng};

const EIGHT: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> Result<()> {
    let handles = (0..10)
        .map(|_| {
            std::thread::spawn::<_, Result<_>>(|| loop {
                generate_test_case()?;
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

fn generate_test_case() -> Result<()> {
    let mut rng = thread_rng();

    let mut args = vec![];

    // Some probability ratio to either generate a random string of numbers
    // or generate the args by taking [1, 2, 3, .., 8] and removing some args.
    // This gives us good samples with/without replacement.
    if rng.gen_ratio(3, 4) {
        args = EIGHT.to_vec();

        for _ in 0..rng.gen_range(0..8) {
            args.remove(rng.gen_range(0..args.len()));
        }

        // Shuffle the args, sometimes
        if rng.gen_ratio(1, 4) {
            args.shuffle(&mut rng);
        }
    } else {
        for _ in 0..rng.gen_range(1..=8) {
            args.push(rng.gen_range(1..=8));
        }
    }

    let mut params = vec![];
    if rng.gen_ratio(1, 2) {
        params = EIGHT.to_vec();

        for _ in 0..rng.gen_range(0..8) {
            params.remove(rng.gen_range(0..params.len()));
        }

        // Shuffle the args, sometimes
        if rng.gen_ratio(1, 4) {
            params.shuffle(&mut rng);
        }
    } else {
        for _ in 0..rng.gen_range(1..=8) {
            params.push(rng.gen_range(1..=8));
        }
    }

    let i = COUNT.fetch_add(1, Ordering::AcqRel);

    let contents = format!(
        r#"
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo{i}({expected}) {{}}
        
        fn test{i}() {{ foo{i}({provided}); }}
    "#,
        expected = args
            .into_iter()
            .map(|i| format!("_: S{i}"))
            .collect::<Vec<_>>()
            .join(", "),
        provided = params
            .into_iter()
            .map(|i| format!("S{i}"))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let file = std::env::temp_dir().join(format!("fuzz{}.rs", i));

    std::fs::write(&file, contents)?;

    let nightly = execute("+nightly", &file)?;
    let stage1 = execute("+stage1", &file)?;

    let file_name = file.display();

    match (nightly, stage1) {
        (Output::Ice, Output::NoIce) => {
            println!("ICE: {file_name}");
        }
        (Output::Ice, Output::Ice) => {
            println!("BOTH ICE: {file_name}");
            // Stop for investigation
            std::process::exit(1);
        }
        (Output::NoIce, Output::Ice) => {
            println!("Only +stage1 ICEs: {file_name}");
            // Stop for investigation
            std::process::exit(1);
        }
        _ => {
            print!(".");
            std::io::stdout().flush()?;
            std::fs::remove_file(&file)?;
        }
    }

    Ok(())
}

fn execute(stage: &str, file: &Path) -> Result<Output> {
    let status = Command::new("rustc")
        .arg(stage)
        .arg("--crate-type=lib")
        .arg(file)
        .current_dir(std::env::temp_dir())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    Ok(if let Some(code) = status.code() {
        if code == 101 {
            Output::Ice
        } else if code == 1 || code == 0 {
            Output::NoIce
        } else {
            anyhow::bail!("what the hell is error code {code}");
        }
    } else {
        Output::NoIce
    })
}

enum Output {
    Ice,
    NoIce,
}
