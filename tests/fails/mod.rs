#[cfg(not(miri))]
#[test]
fn string_lifetimes() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/types/*.rs");
}
