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
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


use iref::iri::{Path};
use json_ld::syntax::Parse;
use json_ld::LoadError;
use json_ld::{Loader, RemoteDocument, syntax::Value};
use json_ld::iref::{Iri, IriBuf};
use reqwest::{Client};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use json_ld_core::fs::Error;
use linked_data::json_syntax;

pub struct HttpLoader {
    client: Client,
    cache: Arc<std::sync::Mutex<HashMap<IriBuf, RemoteDocument<IriBuf, Value>>>>,
    mounts: HashMap<IriBuf, std::path::PathBuf>,
}

impl HttpLoader {
    pub fn new(proxy: Option<reqwest::Proxy>) -> Self {
        match proxy {
            Some(proxy) => {
                HttpLoader {
                    client: Client::builder().proxy(proxy).gzip(true).build().unwrap(),
                    cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
                    mounts: HashMap::new(),
                }
            },
            None => {
                HttpLoader {
                    client: Client::builder().gzip(true).build().unwrap(),
                    cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
                    mounts: HashMap::new(),
                }
                
            }
        }
    }

    #[inline(always)]
    pub fn mount(&mut self, iri: IriBuf, path: std::path::PathBuf) {
        self.mounts.insert(iri, path);
    }

    pub fn unmount(&mut self, iri: &IriBuf) {
        self.mounts.remove(iri);
    }

    pub fn unmount_all(&mut self) {
        self.mounts.clear();
    }

    pub fn intercept(&self, iri: &IriBuf) -> Option<(String,std::path::PathBuf)> {
        let iri = iri.to_string();
        for (prefix, path) in self.mounts.iter() {
            if iri.starts_with(prefix.as_str()) {
                // Remove the prefix from the IRI, keep the rest
                let rest = iri.split_at(prefix.as_str().len()).1.to_string();

                return Some((rest,path.clone()));
            }
        }
        None
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

        // Check if the IRI is intercepted
        if let Some((rest,path)) = self.intercept(&iri) {
            let path = path.join(rest);

            let content = std::fs::read_to_string(path).map_err(|e| LoadError::new(iri.clone(), e) );
            if let Ok(content) = content {
                let result  = json_syntax::Value::parse_str(&content);
                if let Ok((doc,_)) = result {
                    let document = RemoteDocument::new(Some(url.to_owned()), Some("application/ld+json".parse().unwrap()), doc);
                    // Cache the document
                    {
                        let mut cache = self.cache.lock().unwrap();
                        cache.insert(iri.clone(), document.clone());
                    }
                    return Ok(document);
                }
            }
        }

        // Fetch the document using HTTP
        let response = self.client.get(iri.as_str()).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Parse the content into JSON-LD
                    let content = resp.text().await.map_err(|e| LoadError::new(iri.clone(), e) )?;

                    let (doc, _) = json_syntax::Value::parse_str(&content)
                        .map_err(|e| LoadError::new(url.to_owned(), Error::Parse(e)))?;

                    let document = RemoteDocument::new(Some(url.to_owned()), Some("application/ld+json".parse().unwrap()), doc);

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

#[cfg(test)]
mod test {
    use reqwest::{Client, Proxy};


    #[tokio::test]
    async fn test_request(){
        let proxy = Proxy::https("http://192.168.12.51:9981").unwrap();

        let url = "https://schema.org/";
        // let url = "https://www.baidu.com/";
        let client = Client::builder().proxy(proxy).gzip(true).build().unwrap();

        let response = client.get(url).send().await;
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("Headers: {:?}", resp.headers());
                    let content = resp.text().await.unwrap();
                    println!("Body: {:#?}", content);
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