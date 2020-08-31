mod ptree;

#[test]
fn it_works() {
    let mut pt = ptree::Ptree::new();
    pt.add("naman");
    pt.add("nanan");

    assert!(pt.find("naman"));
    assert!(!pt.find("sakshi"));
}
