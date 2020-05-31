#[test]
fn set_password() {
    let service = "ktr_service".to_owned();
    let account = "ktr_account".to_owned();
    let password_in = "ktr_password".to_owned();
    keytar::set_password(service.clone(), account.clone(), password_in.clone()).unwrap();

    let password_out = keytar::get_password(service.clone(), account.clone()).unwrap();
    assert_eq!(true, password_out.success);
    assert_eq!(password_in, password_out.password);

    keytar::delete_password(service.clone(), account.clone()).unwrap();
}

#[test]
fn get_not_existent_password() {
    let password_out = keytar::get_password("doesn't".to_owned(), "exist".to_owned()).unwrap();
    assert_eq!(false, password_out.success);
}

#[test]
fn delete_password() {
    let service = "ktr_del_service".to_owned();
    let account = "ktr_del_account".to_owned();
    let password_in = "ktr_del_password".to_owned();
    keytar::set_password(service.clone(), account.clone(), password_in.clone()).unwrap();

    keytar::delete_password(service.clone(), account.clone()).unwrap();

    let password_out = keytar::get_password(service, account).unwrap();
    assert_eq!(false, password_out.success);
}

#[test]
fn find_password() {
    let service = "ktr_find_service".to_owned();
    let account = "ktr_find_account".to_owned();
    let password_in = "ktr_find_password".to_owned();
    keytar::set_password(service.clone(), account.clone(), password_in.clone()).unwrap();

    let password_out = keytar::find_password(service.clone()).unwrap();
    assert_eq!(true, password_out.success);
    assert_eq!(password_in, password_out.password);

    keytar::delete_password(service.clone(), account.clone()).unwrap();
}
