extern crate fastobo;
// extern crate ureq;

fn main() {
    let response = ureq::get("http://purl.obolibrary.org/obo/hp.obo").call();
    let mut reader = std::io::BufReader::new(response.unwrap().into_reader());

    match fastobo::from_reader(&mut reader) {
        Ok(doc) => println!("Number of hpo entities: {}", doc.entities().len()),
        Err(e) => panic!("Could not parse the Mass-Spec Ontology: {}", e),
    }
}