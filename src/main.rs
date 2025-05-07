use std::process::ExitCode;

use std::io;

fn sub() -> Result<(), io::Error> {
    rs_zip2meta::stdin2zipitems2metadata2jsons2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
