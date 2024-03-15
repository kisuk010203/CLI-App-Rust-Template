use std::process::Command;
use assert_fs::prelude::*;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cli-app")?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("lorem ipsum\ndolor sit amet\naloremalorem")?;

    let mut cmd = Command::cargo_bin("cli-app")?;
    cmd.arg("lorem").arg(file.path());
    cmd.assert().success().stdout("lorem ipsum\naloremalorem\n");

    Ok(())
}