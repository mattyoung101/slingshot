/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use anyhow::Context;
use bytesize::ByteSize;
use dashmap::DashMap;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Mutex, fs};
use xxhash_rust::xxh3::xxh3_64;
use crate::SvDocument;

/// Current version of the index file format.
const INDEX_VERSION: &str = "0.1.0";

/// Only actually write a real update to disk every this many calls to maybe_flush
const INDEX_UPDATE_BATCH_SIZE: u32 = 2;

/// This is the actual index that is written to disk. Note that the index, like clangd's, is per
/// project.
/// This is just serialised to disk with serde and flatbuffers (via sqlite), so put whatever in here.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Index {
    /// Version of the index file
    pub version: String,

    /// Mapping between the absolute path of each document in the project and its parsed document
    /// tree. Used to calculate completions for each document.
    pub document_trees: DashMap<PathBuf, SvDocument>,

    /// Mapping between the absolute path of each document in the project and the xxh3 hash of its contents.
    /// Used to determine if the index needs to be refreshed when the LSP starts up or not.
    pub document_hashes: DashMap<PathBuf, u64>,
}

/// This is the tool for managing the index cache of all files
#[derive(Debug, Default)]
pub struct IndexManager {
    /// Currently loaded index. Either empty if no index exists yet or partially filled.
    pub index: Index,

    /// Path to the index file
    pub index_path: PathBuf,
    
    /// Current flush batch counter
    batch: Mutex<u32>
}

impl IndexManager {
    fn default(path: &PathBuf) -> IndexManager {
        let index = Index {
            version: INDEX_VERSION.to_string(),
            document_trees: DashMap::new(),
            document_hashes: DashMap::new(),
        };
        IndexManager {
            index,
            index_path: path.to_path_buf(),
            batch: 0.into()
        }
    }

    /// Writes the current index to be flushed to disk. TODO make this function async
    pub fn flush(&self) -> Result<(), anyhow::Error> {
        debug!("Flushing index now");
        let binding = self.index_path.to_path_buf();
        let parent = binding.parent().context("No parent")?;
        debug!("Parent dir: {:?}", parent);

        // create cache directory if it doesn't exist yet
        fs::create_dir_all(parent)?;

        let mut serialiser = flexbuffers::FlexbufferSerializer::new();
        self.index.serialize(&mut serialiser)?;
        fs::write(self.index_path.to_path_buf(), serialiser.take_buffer())?;
        
        Ok(())
    }
    
    /// To reduce disk thrashing, we batch updates, which is controlled by
    /// INDEX_UPDATE_BATCH_SIZE. When you call maybe_flush, it will only actually flush to disk
    /// every INDEX_UPDATE_BATCH_SIZE calls.
    pub fn maybe_flush(&self) -> Result<(), anyhow::Error> {
        let mut guard = self.batch.lock().unwrap();
        if *guard == INDEX_UPDATE_BATCH_SIZE {
            debug!("Flushing to disk!");
            *guard = 0;
            return self.flush();
        } else {
            debug!("Not flushing index yet, progress {}/{}", *guard, INDEX_UPDATE_BATCH_SIZE);
            *guard += 1;
            return Ok(())
        }
    }

    /// Requests that the given symbols at the given file path are introduced into the index.
    /// May not necessarily be done if the path and document text is exactly the same as the one
    /// that already exists in the index.
    /// This means that insert() can safely be called many times without significant performance
    /// loss.
    pub fn insert(&self, path: &PathBuf, document: &str, document_tree: &SvDocument) {
        let hash = xxh3_64(document.as_bytes());
        let existing = self.index.document_hashes.get(path);
        if existing.is_some() && *existing.unwrap() == hash {
            // this same document with the exact same hash already exists in the index - assume
            // that the document tree is valid, so no need to update
            debug!(
                "No need to insert document at {:?} with hash {:#X} - already exists",
                path, hash
            );
            return;
        }

        self.index.document_hashes.insert(path.to_path_buf(), hash);
        self.index
            .document_trees
            .insert(path.to_path_buf(), document_tree.clone());
        debug!(
            "Inserted document at path {:?} with hash {:#X} into index",
            path, hash
        );
    }

    fn attempt_deserialise(path: &PathBuf) -> Result<Index, anyhow::Error> {
        // read the document to a byte array
        let bytes = std::fs::read(path)?;
        debug!(
            "Index file is {}",
            ByteSize(bytes.len().try_into().unwrap())
        );

        // deserialise with flexbuffers
        let reader = flexbuffers::Reader::get_root(bytes.as_slice())?;
        debug!("Instantiated flexbuffers reader, will now deserialise");

        return Ok(Index::deserialize(reader)?);
    }

    /// Creates or loads the IndexManager for the particular project.
    /// Must be a full path to the Slingshot cache, e.g.
    /// /home/matt/workspace/epic_riscv_cpu/.cache/slingshot/slingshot_cache.dat
    pub fn new(path: &PathBuf) -> Self {
        info!(
            "IndexManager initialising in path {}",
            path.to_str().unwrap()
        );

        if path.exists() {
            info!("Index appears to exist, going to try and load it");
            
            match Self::attempt_deserialise(path) {
                Ok(index) => {
                    info!("Deserialised index successfully");
                    return IndexManager {
                        index,
                        index_path: path.to_path_buf(),
                        batch: 0.into()
                    }
                }
                Err(e) => {
                    error!("Failed to deserialise index: {:?}", e);
                    return IndexManager::default(&path.to_path_buf());
                }
            }
        } else {
            info!("Index does not yet exist, will be created on next write");
            // TODO request population of index
            IndexManager::default(&path.to_path_buf())
        }
    }
}
