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
use tokio::fs;
use std::hash::Hash;
use std::collections::BTreeMap;
use std::path::Path;
use xxhash_rust::xxh3::xxh3_64;

use crate::SvDocument;

/// Current version of the index file format.
const INDEX_VERSION: &str = "0.1.0";

/// Write the index to disk every this many seconds if it was updated since the last time we wrote
/// it.
const REFRESH_INDEX_TIMER: u32 = 60;

/// This is the actual index that is written to disk. Note that the index, like clangd's, is per
/// project.
/// This is just serialised to disk with serde and flatbuffers (via sqlite), so put whatever in here.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Clone)]
pub struct Index {
    /// Version of the index file
    pub version: String,

    /// Mapping between the absolute path of each document in the project and its parsed document
    /// tree. Used to calculate completions for each document.
    pub document_trees: BTreeMap<String, SvDocument>,

    /// Mapping between the absolute path of each document in the project and the xxh3 hash of its contents.
    /// Used to determine if the index needs to be refreshed when the LSP starts up or not.
    pub document_hashes: BTreeMap<String, u64>,
}

/// This is the tool for managing the index cache of all files
#[derive(Debug, Default)]
pub struct IndexManager {
    /// Currently loaded index. Either empty if no index exists yet or partially filled.
    pub index: Index,
    
    /// Path to the index file
    pub index_path: String,
    
    /// Copy of the last Index struct that was written to disk
    last_written_index: Index
}

impl IndexManager {
    fn default(path: &str) -> IndexManager {
        let index = Index {
            version: INDEX_VERSION.to_string(),
            document_trees: BTreeMap::new(),
            document_hashes: BTreeMap::new(),
        };
        IndexManager { index: index.clone(), index_path: path.to_string(), last_written_index: index.clone() }
    }
    
    /// Forces the current index to be flushed to disk.
    pub fn flush(&mut self) {
        // update last written index with current index
        self.last_written_index = self.index.clone();
    }
    
    /// Flushes the index to disk if and only if it was updated since the last time the index was
    /// written to disk.
    pub fn maybe_flush(&mut self) {
        if self.index == self.last_written_index {
            debug!("No need to flush current index - already written to disk");
            return
        }
        
        self.flush();
    }

    /// Requests that the given symbols at the given file path are introduced into the index.
    /// May not necessarily be done if the path and document text is exactly the same as the one
    /// that already exists in the index.
    /// This means that insert() can safely be called many times without significant performance
    /// loss.
    pub fn insert(&mut self, path: &str, document: &str, document_tree: &SvDocument) {
        let hash = xxh3_64(document.as_bytes());
        let existing = self.index.document_hashes.get(path);
        if existing.is_some() && *existing.unwrap() == hash {
            // this same document with the exact same hash already exists in the index - assume
            // that the document tree is valid, so no need to update
            debug!(
                "No need to insert document at {} with hash {} - already exists",
                path, hash
            );
            return;
        }
        
        self.index.document_hashes.insert(path.to_string(), hash);
        self.index.document_trees.insert(path.to_string(), document_tree.clone());
        debug!("Inserted document at path {} with hash {} into index", path, hash);
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
        let binding = index_path.canonicalize().unwrap();
        let absolute_path = binding.to_str().unwrap();

        debug!("index_path: {:?}, absolute_path: {}", index_path, absolute_path);

        if index_path.exists() {
            debug!("Index appears to exist, going to try and load it");

            // read the document to a byte array
            let bytes = match std::fs::read(index_path) {
                Ok(d) => d,
                Err(e) => {
                    error!("Failed to load slingshot_index.dat: {}", e);
                    return IndexManager::default(absolute_path);
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
                    return IndexManager::default(absolute_path);
                }
            };
            debug!("Instantiated flexbuffers reader, will now deserialise");

            return match Index::deserialize(reader) {
                Ok(index) => IndexManager { index: index.clone(), index_path: absolute_path.to_string(), last_written_index: index.clone() },
                Err(e) => {
                    error!("Failed to deserialise index: {}", e);
                    return IndexManager::default(absolute_path);
                }
            };
        } else {
            debug!("Index does not yet exist, will be created on next write");
            // TODO request population of index
            IndexManager::default(absolute_path)
        }
    }
}
