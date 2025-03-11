// Copyright 2024 meduo <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Error;
use iref::IriBuf;
use json_ld::Expand;
use json_ld::ExpandedDocument;
use json_ld::RemoteDocument;
use json_ld::{syntax::{Parse, Value}};
use reqwest::{Proxy};
use super::httploader;


#[derive(Default)]
pub struct JsonLdHttpParser{
    pub proxy: Option<Proxy>,
}

impl JsonLdHttpParser {
    pub fn new(proxy: Option<Proxy>) -> Self {
        Self {
           proxy,
        }
    }

    pub async  fn parse(&mut self, iri: String, val: String) -> Result<ExpandedDocument<IriBuf>, Error> {
        let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(IriBuf::new(iri).unwrap()),
            // Optional content type.
            Some("application/ld+json".parse().unwrap()),
            // The actual content.
            Value::parse_str(&val).expect("unable to parse document").0,
        );


        let mut loader = httploader::HttpLoader::new(self.proxy.clone());
        let result =  input.expand(&mut loader).await;
        match result {
            Ok(document) => {
               Ok(document) 
            },
            Err(e) => {
                Err(anyhow::anyhow!("{}", e))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_odrl_policy(){
        let mut parse = JsonLdHttpParser::new(None);
        let val = r#"
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
         "#;

        let iri = "https://datasafe.com".to_string();

        let document = parse.parse(iri, val.to_string()).await;

        match document {
            Ok(document) => {
                for object in document {
                    if let Some(id) = object.id() {
                        let telephone = object.as_node().unwrap()
                        .get_any(&IriBuf::new("http://www.w3.org/ns/odrl/2/Asset".to_owned()).unwrap()).unwrap();
                
                        println!("id: {id}");
                        println!("telephone: {telephone:#?}");
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod test_jsonld {
    use std::fmt::Debug;

    use iref::IriBuf;
    use json_ld::{context_processing::ProcessedRef, object::Any, syntax::{Parse, Value}, Compact, Iri, RemoteDocument, RemoteDocumentReference};
    use reqwest::Proxy;

    use crate::linkdata::httploader;

    #[tokio::test]
    async fn test_no_loader() {
        use json_ld::{syntax::{Parse, Value}, JsonLdProcessor, RemoteDocument};
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
    async fn test_fs_load() {
        use static_iref::iri;
        use json_ld::{JsonLdProcessor, RemoteDocumentReference};

        let input = RemoteDocumentReference::iri(iri!("https://example.com/sample.jsonld").to_owned());

        // Use `FsLoader` to redirect any URL starting with `https://example.com/` to
        // the local `example` directory. No HTTP query.
        let mut loader = json_ld::FsLoader::default();
        loader.mount(iri!("https://example.com/").to_owned(), "examples");

        let expanded = input.expand(&mut loader)
            .await
            .expect("expansion failed");
        for object in expanded {
            if let Some(id) = object.id() {
                let name = object.as_node().unwrap()
                    .get_any(&IriBuf::new("http://xmlns.com/foaf/0.1/name".to_owned()).unwrap()).unwrap()
                    .as_str().unwrap();
                println!("id: {id}");

                println!("name: {name}");
            }
        }
    }

    #[tokio::test]
    async fn test_json_ld_mount() {
        use static_iref::iri;
        use json_ld::{JsonLdProcessor, RemoteDocumentReference};
        
        let input: RemoteDocumentReference = RemoteDocumentReference::iri(iri!("http://192.168.12.7:8000/contexts/person.jsonld").to_owned());
        
        // let mut loader = httploader::HttpLoader::new(Some(Proxy::https("192.168.12.51:9981").unwrap()));
        let mut loader = httploader::HttpLoader::new(None);
        // loader.mount(iri!("https://json-ld.org/").to_owned(), "examples");

        //print current working directory
        let current_dir = std::env::current_dir().unwrap();
        println!("current dir: {}", current_dir.display());
        
        let expanded = input.expand(&mut loader)
          .await;

        match expanded {
            Ok(expanded) => {
                for object in expanded{
                    println!("{:?}", object.as_node().unwrap().properties());
                }
            },
            Err(err) => {
                println!("error: {err:#?}");
            }
        }
    }

    #[tokio::test]
    async fn test_json_http_with_context() {
        use json_ld::{syntax::{Parse, Value}, JsonLdProcessor, RemoteDocument};
        use json_ld::iref::IriBuf;

        // Create a "remote" document by parsing a file manually.
        let input = RemoteDocument::new(
            // We use `IriBuf` as IRI type.
            Some(IriBuf::new("https://datasafe.com/persion.jsonld".to_owned()).unwrap()),

            // Optional content type.
            Some("application/ld+json".parse().unwrap()),

            // Parse the file.
            Value::parse_str(r#"
            {
              "@context": "https://json-ld.org/contexts/person.jsonld",
              "@id": "http://dbpedia.org/resource/John_Lennon",
              "name": "John Lennon",
              "born": "1940-10-09",
              "spouse": "http://dbpedia.org/resource/Cynthia_Lennon"
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

                let born = object.as_node().unwrap()
                    .get_any(&IriBuf::new("http://schema.org/birthDate".to_owned()).unwrap()).unwrap()
                    .as_str().unwrap();

                let spouse = object.as_node().unwrap()
                    .get_any(&IriBuf::new("http://schema.org/spouse".to_owned()).unwrap()).unwrap()
                    .as_str().unwrap();

                println!("id: {id}");
                println!("name: {name}");
                println!("born: {born}");
                println!("spouse: {spouse}");
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
        use json_ld::{syntax::{Parse, Value}, JsonLdProcessor, RemoteDocument};
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
    async fn test_http_loader_compact() {
        use json_ld::{syntax::{Parse, Value}, JsonLdProcessor, RemoteDocument};
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

        let mut loader = httploader::HttpLoader::new(Some(Proxy::https("192.168.12.51:9981").unwrap()));
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
    }

    #[tokio::test]
    async fn test_json_ld_document() {
        use json_ld::{syntax::{Parse, Value}, JsonLdProcessor, RemoteDocument};
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

#[cfg(test)]
mod test_sophia {
    use sophia::api::source::Source;
    use sophia::jsonld::{JsonLdOptions, JsonLdQuadSource};
    use sophia::jsonld::loader::HttpLoader;
    use sophia::jsonld::loader_factory::{DefaultLoaderFactory};

    #[test]
    fn test_sophia_jsonld() {
        use sophia::jsonld::JsonLdParser;
        use sophia::api::prelude::*;
        use sophia::inmem::graph::LightGraph;

        let example = r#"{
              "@context": "https://json-ld.org/contexts/person.jsonld",
              "@id": "http://dbpedia.org/resource/John_Lennon",
              "name": "John Lennon",
              "born": "1940-10-09",
              "spouse": "http://dbpedia.org/resource/Cynthia_Lennon"
        }"#;

        // Parse the JSON-LD into a graph
        let parser: JsonLdParser<DefaultLoaderFactory<HttpLoader>> = JsonLdParser::<DefaultLoaderFactory<HttpLoader>>::
            new_with_options(JsonLdOptions::default());
        let quad_source =  parser.parse_str(example);

        match quad_source {
            JsonLdQuadSource::Quads(quads) => {
                println!("----------------------------");
                for quad in quads {
                   let triples = quad.into_triple();
                   let sub = triples.clone().to_s();
                   let pred = triples.clone().to_p();
                   let obj = triples.clone().to_o();
                   println!(" sub: {sub:#?}");
                   println!(" pred: {pred:#?}");
                   println!(" obj: {obj:#?}");
                }
            },
            JsonLdQuadSource::Err(err) => {
                println!("error: {err:#?}");
            }
        }
    }
}