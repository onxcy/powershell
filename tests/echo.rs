use powershell::PowerShell;

#[test]
fn test_echo() {
    let test_str = "hello world!";
    let o = PowerShell::new()
        .add_variable("a", test_str)
        .invoke("echo $a");
    let str = std::str::from_utf8(&o.stdout).unwrap().trim();
    assert_eq!(str, test_str);
}
