#[test]
fn test_echo() {
    let output = powershell::execute("echo 'Hello world!'");
    let output = std::str::from_utf8(&output.stdout).unwrap().trim();
    assert_eq!(output, "Hello world!");
}
