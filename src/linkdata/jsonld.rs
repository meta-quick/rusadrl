
mod test {
    use std::fmt::Debug;

    use iref::IriBuf;
    use json_ld::{compaction, context_processing::ProcessedRef, object::{node::properties, Any}, print, rdf_types::vocabulary::no_vocabulary, syntax::{print::print_array, Parse, Value}, Compact, Flatten, Iri, RemoteDocument, RemoteDocumentReference};
    use reqwest::Proxy;

    use crate::linkdata::httploader;

    #[tokio::test]
    async fn it_works() {
        use json_ld::{JsonLdProcessor, RemoteDocument, syntax::{Value, Parse}};
        use json_ld::iref::IriBuf;

        // Create a "remote" document by parsing a file manually.
         let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(IriBuf::new("https://example.com/sample.jsonld".to_owned()).unwrap()),
        
            // Optional content type.
            Some("application/ld+json".parse().unwrap()),
            
            // Parse the file.
            Value::parse_str(r#"
            {
                "@context": {
                "name": "http://xmlns.com/foaf/0.1/name",
                "homepage": "http://xmlns.com/foaf/0.1/homepage"
                },
                "@id": "https://www.rust-lang.org",
                "name": "Rust Programming Language",
                "homepage": "https://www.rust-lang.org"
            }"#).expect("unable to parse file").0
        );

        let mut loader = json_ld::NoLoader;
        let expanded = input
            .expand(&mut loader)
            .await
            .expect("expansion failed");

        for object in expanded {
            if let Some(id) = object.id() {
                let name = object.as_node().unwrap()
                .get_any(&IriBuf::new("http://xmlns.com/foaf/0.1/name".to_owned()).unwrap()).unwrap()
                .as_str().unwrap();

                let homepage = object.as_node().unwrap()
                .get_any(&IriBuf::new("http://xmlns.com/foaf/0.1/homepage".to_owned()).unwrap()).unwrap()
                .as_str().unwrap();

                println!("id: {id}");
                println!("name: {name}");
                println!("homepage: {homepage}");
            }
        }
    }

    #[tokio::test]
    async fn test_iref() {
        use static_iref::{iri, iri_ref};
        use json_ld::Iri;
        use json_ld::iref::IriRef;

        const IRI: &'static Iri = iri!("https://www.rust-lang.org/foo/bar#frag");
        const IRI_REF: &'static IriRef = iri_ref!("/foo/bar#frag");

        assert_eq!(IRI.to_string(), "https://www.rust-lang.org/foo/bar#frag");
        assert_eq!(IRI_REF.to_string(), "/foo/bar#frag");
    }

    #[tokio::test]
    async fn test_json_ld_mount() {
        use static_iref::iri;
        use json_ld::{JsonLdProcessor, Options, RemoteDocumentReference};
        
        let input: RemoteDocumentReference = RemoteDocumentReference::iri(iri!("https://json-ld.org/contexts/person.jsonld").to_owned());
        
        let mut loader = httploader::HttpLoader::new(None);
        // loader.mount(iri!("https://json-ld.org/").to_owned(), "examples");

        //print current working directory
        let current_dir = std::env::current_dir().unwrap();
        println!("current dir: {}", current_dir.display());
        
        let expanded = input.expand(&mut loader)
          .await;

        match expanded {
            Ok(expanded) => {
                println!("expanded: {expanded:#?}");
            },
            Err(err) => {
                println!("error: {err:#?}");
            }
        }
    }

    #[tokio::test]
    async fn test_phantom_data() {
        use std::marker::PhantomData;

        #[derive(Debug, Default)]
        struct ZeroSize<T> {
            data: Box<T>,
            _phantom: PhantomData<T>, // 零大小的类型标记
        }

        impl<T: Default> ZeroSize<T> {
            fn new() -> Self {
                ZeroSize {
                    data: Box::new(T::default()),
                    _phantom: PhantomData,
                }
            }
        }
        let _zero_size: ZeroSize<i32> = ZeroSize::new();
        println!("{}",std::mem::size_of::<ZeroSize<i32>>());
        println!("{_zero_size:#?}");
    }


    #[tokio::test]
    async fn test_http_loader() {
        use json_ld::{JsonLdProcessor, RemoteDocument, syntax::{Value, Parse}};
        use json_ld::iref::IriBuf;

        // Create a "remote" document by parsing a file manually.
         let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(IriBuf::new("https://example.com/sample.jsonld".to_owned()).unwrap()),
        
            // Optional content type.
            Some("application/ld+json".parse().unwrap()),
            
            // Parse the file.
            Value::parse_str(r#"
            {
                "@context": {
                "name": "http://xmlns.com/foaf/0.1/name",
                "homepage": "http://xmlns.com/foaf/0.1/homepage"
                },
                "@id": "https://www.rust-lang.org",
                "name": "Rust Programming Language",
                "homepage": "https://www.rust-lang.org"
            }"#).expect("unable to parse file").0
        );

        let mut loader = httploader::HttpLoader::new(None);
        let expanded = input
            .expand(&mut loader)
            .await
            .expect("expansion failed");

        for object in expanded {
            if let Some(id) = object.id() {
                let name = object.as_node().unwrap()
                .get_any(&IriBuf::new("http://xmlns.com/foaf/0.1/name".to_owned()).unwrap()).unwrap()
                .as_str().unwrap();

                let homepage = object.as_node().unwrap()
                .get_any(&IriBuf::new("http://xmlns.com/foaf/0.1/homepage".to_owned()).unwrap()).unwrap()
                .as_str().unwrap();

                println!("id: {id}");
                println!("name: {name}");
                println!("homepage: {homepage}");
            }
        }
    }


    #[tokio::test]
    async fn test_http_xloader() {
        use json_ld::{JsonLdProcessor, RemoteDocument, syntax::{Value, Parse}};
        use json_ld::iref::IriBuf;

        // Create a "remote" document by parsing a file manually.
         let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(IriBuf::new("https://datasafe.io".to_owned()).unwrap()),
        
            // Optional content type.
            Some("application/ld+json".parse().unwrap()),
            
            // Parse the file.
            Value::parse_str(r#"
            {
                "@context": {
                    "name": "https://datasafe.io/1.1/name",
                    "jobTitle": "https://datasafe.io/1.1/jobTitle",
                    "telephone": "https://datasafe.io/1.1/telephone",
                    "url": "https://datasafe.io/1.1/url"
                },
                "@id": "https://datasafe.io/iri/1.1/Person",
                "@type": "Person",
                "name": "Jane Doe",
                "jobTitle": "Professor",
                "telephone": "(425) 123-4567",
                "url": "http://www.janedoe.com"
            }"#).expect("unable to parse file").0
        );

        let mut loader = httploader::HttpLoader::new(Some(Proxy::https("192.168.12.57:9981").unwrap()));
        let expanded = input
            .expand(&mut loader)
            .await;

        if let Err(e) = &expanded {
            println!("error: {e:#?}");
            return;
        }

        let unprocessed = json_ld_syntax::context::Context::null();
        let processed = json_ld_core::context::Context::new(
          Some(IriBuf::new("https://datasafe.io/1.1/".to_owned()).unwrap())
        );
        let context = ProcessedRef::new(&unprocessed, &processed);

        let compacted = &expanded.unwrap().compact(context, &mut loader).await;

        if let Err(e) = &compacted {
            println!("error: {e:#?}");
            return;
        }

        println!("compacted: {compacted:#?}");


        // let expanded = expanded.unwrap();
        // for object in expanded {
        //     if let Some(id) = object.id() {
        //         let name = object.as_node().unwrap()
        //         .get_any(&IriBuf::new("https://datasafe.io/1.1/name".to_owned()).unwrap()).unwrap()
        //         .as_str().unwrap();

        //         let job_title = object.as_node().unwrap()
        //         .get_any(&IriBuf::new("https://datasafe.io/1.1/jobTitle".to_owned()).unwrap()).unwrap()
        //         .as_str().unwrap();

        //         let telephone = object.as_node().unwrap()
        //         .get_any(&IriBuf::new("https://datasafe.io/1.1/telephone".to_owned()).unwrap()).unwrap()
        //         .as_str().unwrap();

        //         let url = object.as_node().unwrap()
        //         .get_any(&IriBuf::new("https://datasafe.io/1.1/url".to_owned()).unwrap()).unwrap()
        //         .as_str().unwrap();

        //         println!("id: {id}");
        //         println!("name: {name}");
        //         println!("jobTitle: {job_title}");
        //         println!("telephone: {telephone}");
        //         println!("url: {url}");
        //     }
        // }
    }

    #[tokio::test]
    async fn test_json_ld_document() {
        use json_ld::{JsonLdProcessor, RemoteDocument, syntax::{Value, Parse}};
        use json_ld::iref::IriBuf;
        use crate::linkdata::httploader;

        // Create a "remote" document by parsing a file manually.
         let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(IriBuf::new("https://datasafe.io/index".to_owned()).unwrap()),
        
            // Optional content type.
            Some("application/ld+json".parse().unwrap()),
            
            // Parse the file.
            Value::parse_str(r#"
            {
                "@context": [
                  "https://www.w3.org/ns/odrl.jsonld",
                  {
                    "title": "https://datasafe.io/ds/1.1/title",
                    "creator": "https://datasafe.io/ds/1.1/creator",
                    "dateCreated": "https://datasafe.io/ds/1.1/dateCreated"
                  }
                ],
                "type": "Policy",
                "uid": "urn:uuid:12345678-90ab-cdef-1234-567890abcdef",
                "permission": [
                    {
                    "action": "use",
                    "target": "https://example.com/media/video1.mp4",
                    "constraint": {
                        "leftOperand": "date",
                        "operator": "before",
                        "rightOperand": "2025-12-31"
                    }
                    }
                ],
                "obligation": [
                    {
                    "action": "credit",
                    "target": "https://example.com/creator/author1",
                    "constraint": {
                        "leftOperand": "license",
                        "operator": "equals",
                        "rightOperand": "CC-BY"
                    }
                    }
                ],
                "prohibition": [
                    {
                    "action": "copy",
                    "target": "https://example.com/media/video1.mp4"
                    }
                ],
                "Asset": [
                    {
                    "id": "https://example.com/media/video1.mp4",
                    "type": "Video",
                    "status": "active",
                    "title": "Example Video",
                    "creator": "Example Creator",
                    "dateCreated": "2024-01-01"
                    }
                ]
         }
         "#).expect("unable to parse file").0
        );

        let mut loader = httploader::HttpLoader::new(None);
        // let mut loader = json_ld::NoLoader::default();
        let expanded = input
            .expand(&mut loader)
            .await
            .expect("expansion failed");

        for object in expanded {
            if let Some(id) = object.id() {
                let telephone = object.as_node().unwrap()
                .get_any(&IriBuf::new("http://www.w3.org/ns/odrl/2/Asset".to_owned()).unwrap()).unwrap();
        
                println!("id: {id}");
                println!("telephone: {telephone:#?}");
            }
        }
    }

    #[test]
    fn test_rdf_turtle() -> Result<(), Box<dyn std::error::Error>> {
        use sophia::api::prelude::*;
        use sophia::api::ns::Namespace;
        use sophia::inmem::graph::LightGraph;
        use sophia::turtle::parser::turtle;
        use sophia::turtle::serializer::nt::NtSerializer;

        // loading a graph
        let example = r#"
        @prefix : <http://example.org/>.
        @prefix foaf: <http://xmlns.com/foaf/0.1/>.
        :alice foaf:name "Alice";
            foaf:mbox <mailto:alice@work.example> .
        :bob foaf:name "Bob".
        "#;
        let mut graph: LightGraph = turtle::parse_str(example).collect_triples()?;

        // mutating the graph
        let ex = Namespace::new("http://example.org/")?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
        graph.insert(
        ex.get("bob")?,
        foaf.get("knows")?,
        ex.get("alice")?,
        )?;

        let mut nt_stringifier = NtSerializer::new_stringifier();
        let example2 = nt_stringifier.serialize_graph(&graph)?.as_str();
        println!("The resulting graph:\n{}", example2);

        Ok::<(), Box<dyn std::error::Error>>(())
    }

    #[tokio::test]
    async fn test_rdf_json() {
        use serde_json::json;


        #[derive(linked_data::Serialize,Debug)]
        #[ld(prefix("ex" = "http://example.org/"))]
        struct Foo {
            #[ld("ex:name")]
            name: String,

            #[ld("ex:email")]
            email: String,
        }

        let foo = Foo {
            name: "Alice".to_string(),
            email: "alice@work.example".to_string(),
        };

       let unprocessed = json_ld_syntax::context::Context::null();
       let processed = json_ld_core::context::Context::new(
         Some(IriBuf::new("http://example.org/".to_owned()).unwrap())
       );
       let context = ProcessedRef::new(&unprocessed, &processed);
    

        match json_ld_serialization::serialize(&foo) {
            Ok(expanded) => {       
                let compacted_result = expanded.compact(context, &mut json_ld::NoLoader::default()).await;

                match compacted_result {
                    Ok(compacted_doc) => {
                        println!("Compacted Document: {:#?}",compacted_doc.to_string());
                    }
                    Err(err) => {
                        println!("Error during compaction: {err:#?}");
                    }
                }

                for object in expanded {
                    match object.id(){
                        Some(id) => {
                            println!("id: {id}");
                        } 
                        None => {
                            println!("{:?}",object);
                        }                       
                    }
                }
            },
            Err(err) => {
                println!("error: {err:#?}");
            }
        }
    }


}