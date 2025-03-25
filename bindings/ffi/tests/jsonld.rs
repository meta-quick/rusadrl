use std::ffi::CString;
use adrlcffi::{create_odrl_world, ffi, to_iri};
use adrlcffi::ffi::*;
use rusadrl::model::policy::OdrlRequest;

#[test]
fn test_basic_jsonld() {
    //load and read jsonld file
    let json = std::fs::read_to_string("tests/jsonld/basic.jsonld").unwrap();

    //covert json to *const c_char
    let json = CString::new(json).unwrap();
    let handle = create_odrl_world(json.as_c_str().as_ptr());

    //eval policy
    let mut req = OdrlRequest::default();
    req.set_action(to_iri("http://www.w3.org/ns/odrl/2/use"));
    req.set_assignee(to_iri("https://datasate.ids/usercollection/liumazi"));
    req.set_assigner(to_iri("https://datasate.ids/users/gaosg"));
    req.set_target(to_iri("https://datasate.ids/llm/dataset/0001"));
    let result = ffi::Engine::policy_evaluate(handle,req);
    println!("result: {:?}", result);
}

