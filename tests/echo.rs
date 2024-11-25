use powershell::PowerShell;

#[test]
fn test() {
    let mut ps = PowerShell::new();
    let out = ps.exec("echo '123'");
    assert_eq!(out, "123");
    let out = ps.exec("echo 'abc'");
    assert_eq!(out, "abc");
    drop(ps);
}
