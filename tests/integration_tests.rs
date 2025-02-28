use bs_solctra_rs::simulation::add_one;

#[test]
fn it_adds_one() {
    let res = add_one(1);
    assert_eq!(res, 2);
}
