#[test]
fn set_password() {
    let service = "ktr_service";
    let account = "ktr_account";
    let password_in = "ktr_password";
    keytar::set_password(service, account, password_in).unwrap();

    let password_out = keytar::get_password(service, account).unwrap();
    assert_eq!(true, password_out.success);
    assert_eq!(password_in, password_out.password);

    keytar::delete_password(service, account).unwrap();
}

#[test]
fn get_not_existent_password() {
    let password_out = keytar::get_password("doesn't", "exist").unwrap();
    assert_eq!(false, password_out.success);
}

#[test]
fn delete_password() {
    let service = "ktr_del_service";
    let account = "ktr_del_account";
    let password_in = "ktr_del_password";
    keytar::set_password(service, account, password_in).unwrap();

    keytar::delete_password(service, account).unwrap();

    let password_out = keytar::get_password(service, account).unwrap();
    assert_eq!(false, password_out.success);
}

#[test]
fn find_password() {
    let service = "ktr_find_service";
    let account = "ktr_find_account";
    let password_in = "ktr_find_password";
    keytar::set_password(service, account, password_in).unwrap();

    let password_out = keytar::find_password(service).unwrap();
    assert_eq!(true, password_out.success);
    assert_eq!(password_in, password_out.password);

    keytar::delete_password(service, account).unwrap();
}
