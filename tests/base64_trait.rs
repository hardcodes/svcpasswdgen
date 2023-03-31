use svcpasswdgen::base64_trait::Base64StringConversions;

const PLAINTEXT: &str = r#"PASS!"§$%&/()=?ß\´`+*~'#-_.:,;<>|WORD"#;
const B64: &str = r#"UEFTUyEiwqckJSYvKCk9P8OfXMK0YCsqficjLV8uOiw7PD58V09SRA=="#;
const PLAINTEXT_URLSAFE: &str = r#"subjects?_d=1"#;
const B64_URLSAFE: &str = r#"c3ViamVjdHM_X2Q9MQ=="#;

#[test]
fn base64_default() {
    let base64 = PLAINTEXT.to_string().to_base64_encoded();
    assert_eq!(base64, B64, "not the expected base64 encoded value!");
}

#[test]
fn base64_urlsafe() {
    let base64_urlsafe = PLAINTEXT_URLSAFE.to_string().to_base64_urlsafe_encoded();
    assert_eq!(
        base64_urlsafe, B64_URLSAFE,
        "not the expected url safe base64 encoded value!"
    );
}
