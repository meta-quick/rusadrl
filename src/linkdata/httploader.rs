use json_ld::syntax::Parse;
use json_ld::LoadError;
use json_ld::{Loader, RemoteDocument, syntax::Value};
use json_ld::iref::{Iri, IriBuf};
use reqwest::{Client, Proxy};
use tokio::time::error::Elapsed;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HttpLoader {
    client: Client,
    cache: Arc<std::sync::Mutex<HashMap<IriBuf, RemoteDocument<IriBuf, Value>>>>,
}

impl HttpLoader {
    pub fn new(proxy: Option<reqwest::Proxy>) -> Self {
        match proxy {
            Some(proxy) => {
                HttpLoader {
                    client: Client::builder().proxy(proxy).build().unwrap(),
                    cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
                }
            },
            None => {
                HttpLoader {
                    client: Client::new(),
                    cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
                }
                
            }
        }
    }
}

impl Loader for HttpLoader {
    #[allow(async_fn_in_trait)]
    async fn load(&self, url:  &Iri) -> Result<RemoteDocument<IriBuf>, LoadError>{
        let iri = IriBuf::new(url.to_string()).unwrap();

        {
            // Try to get the document from the cache
            let cache = self.cache.lock().unwrap();
            if let Some(doc) = cache.get(&iri) {
                return Ok(doc.clone());
            }
        }

        // Fetch the document using HTTP
        let response = self.client.get(iri.as_str()).send().await;

     

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Parse the content into JSON-LD
                    let content = resp.text().await.map_err(|e| LoadError::new(iri.clone(), e) )?;
                    let mut result = Value::parse_str(&content);

                    if let Err(_) = &result {
                        result = Value::parse_str("{\"@context\": \"https://schema.org/\"}");
                    }
                    let  value = result.unwrap().0;

                    // Create the RemoteDocument
                    let aplication_json: mime::Mime = "application/ld+json".parse().unwrap();
                    let document = RemoteDocument::new(Some(iri.clone()), Some(aplication_json), value);

                    // Cache the document
                    {
                        let mut cache = self.cache.lock().unwrap();
                        cache.insert(iri.clone(), document.clone());
                    }

                    Ok(document)
                } else {
                    Err(LoadError::new(IriBuf::new(url.to_string()).unwrap(), resp.error_for_status().unwrap_err()))
                }
            }
            Err(e) => Err(LoadError::new(IriBuf::new(url.to_string()).unwrap(), e)),
        }
    }
}

mod test {
    use reqwest::{Client, Proxy};


    #[tokio::test]
    async fn test_request(){
        let proxy = Proxy::https("192.168.12.57:9981").unwrap();

        let url = "https://schema.org/";
        let client = Client::builder().proxy(proxy).build().unwrap();

        let response = client.get(url).send().await;
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Parse the content into JSON-LD
                    let content = resp.text().await.unwrap();
                    println!("{content}");
                }else {
                    println!("err");
                }
            }
            Err(e) => {
                println!("{e:#?}");
            }
        }
    }
}