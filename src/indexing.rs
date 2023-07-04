/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use bytesize::ByteSize;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use xxhash_rust::xxh3::xxh3_64;

use crate::SvToken;

// TODO: due to the way SV works, we basically really want to query what a partial symbol (e.g.
// mymodu...) will resolve into (in this case, "mymodule"). this most likely implies a trie?
// I say do away with sqlite and just write a trie to disk (and do re-writing in an async thread?
// like queue a db update to disk whenever we add new symbols)
// we could use trie_rs if it serialises with serde
// ...
// doesn't seem that trie_rs supports serde serialisation so maybe we write out the symbol table as
// a vector and build the trie ourselves?
// ...
// in the future we would like to record richer data about each token, not just its name and type,
// stuff like docstrings, etc.
// ...
// we probably want to keep a reference to each document's name and its xxh3 hash so we know if we
// need to invalidate the index if it was edited externally

/// Current version of the index file format.
const INDEX_VERSION: &str = "0.1.0";

/// This is the actual index that is written to disk. Note that the index, like clangd's, is per
/// project.
/// This is just serialised to disk with serde and flatbuffers (via sqlite), so put whatever in here.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Index {
    /// Version of the index file
    pub version: String,

    /// List of completion tokens. Stored as Vec, not HashSet, since serde doesn't seem to like
    /// that
    pub tokens: Vec<SvToken>,

    /// Mapping between the name of each document in the project and the xxh3 hash of its contents.
    /// Used to determine if the index needs to be refreshed when the LSP starts up or not.
    /// Note that we use BTreeMap, not HashMap, because we need to serialise it.
    pub documents: BTreeMap<String, u64>,
}

/// This is the tool for managing the index cache of all files
#[derive(Debug, Default)]
pub struct IndexManager {
    /// Currently loaded index. Either empty if no index exists yet or partially filled.
    pub index: Index,
}

impl IndexManager {
    fn default() -> IndexManager {
        let index = Index {
            version: INDEX_VERSION.to_string(),
            tokens: Vec::new(),
            documents: BTreeMap::new(),
        };
        return IndexManager { index };
    }

    /// Requests that the index is flushed to disk. This will only actually bother to do work if a
    /// new call to insert() was made.
    /// TODO we need to then handle what if documents are removed??
    pub async fn flush(&self) {
        todo!()
    }

    /// Requests that the given symbols at the given file path are introduced into the index.
    /// May not necessarily be done if the path and document text is exactly the same as the one
    /// that already exists in the index.
    /// This means that insert() can safely be called many times without significant performance
    /// loss.
    pub fn insert(&self, path: &str, document: &str, symbols: &Vec<SvToken>) {
        let hash = xxh3_64(document.as_bytes());
        let existing = self.index.documents.get(path);
        if existing.is_some() && *existing.unwrap() == hash {
            debug!(
                "No need to insert document at {} with hash {} - already exists",
                path, hash
            );
            return;
        }

        todo!()
    }

    /// Creates or loads the IndexManager for the particular project.
    /// Must be a path to a cache directory, e.g. /home/joe/project/.cache
    pub fn new(path: &Path) -> Self {
        debug!(
            "IndexManager initialising in folder {}",
            path.to_str().unwrap()
        );
        assert!(path.is_dir());

        let index_path = path.join("slingshot_index.dat");
        if index_path.exists() {
            debug!("Index exists, loading it");

            // read the document to a byte array
            let bytes = match std::fs::read(index_path) {
                Ok(d) => d,
                Err(e) => {
                    error!("Failed to load slingshot_index.dat: {}", e);
                    return IndexManager::default();
                }
            };
            debug!(
                "Index file is {}",
                ByteSize(bytes.len().try_into().unwrap())
            );

            // deserialise with flexbuffers
            let reader = match flexbuffers::Reader::get_root(bytes.as_slice()) {
                Ok(d) => d,
                Err(e) => {
                    error!("Failed to instantiate document reader: {}", e);
                    return IndexManager::default();
                }
            };
            debug!("Instantiated flexbuffers reader, will now deserialise");

            return match Index::deserialize(reader) {
                Ok(d) => IndexManager { index: d },
                Err(e) => {
                    error!("Failed to deserialise index: {}", e);
                    return IndexManager::default();
                }
            };
        } else {
            debug!("Index does not yet exist, will be created on next write");
            // TODO request population of index
            return IndexManager::default();
        }
    }
}
