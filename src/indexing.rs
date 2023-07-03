/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use log::{debug, error};
use serde::{Deserialize, Serialize};
use tokio_rusqlite::Connection;
use xxhash_rust::xxh3::xxh3_64;

use crate::completion::CompletionToken;

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

/// This is an indexed version of a file that contains all the symbol information. This is just
/// serialised to disk with serde and flatbuffers (via sqlite), so put whatever in here.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct IndexedFile {
    // hash info is stored separately (as part of the sqlite db)
    /// Path to the file on disk.
    path: String,

    /// List of completion tokens. Stored as Vec, not HashSet, since serde doesn't seem to like
    /// that
    tokens: Vec<CompletionToken>, // consider caching the entire AST and possibly diagnostics - only if necessary
}

pub struct Indexer {}

impl Indexer {
    /// Indexes a particular file and returns an IndexedFile
    fn index(&self, document: &str) -> IndexedFile {
        todo!()
    }
}

/// This is the tool for managing the index cache of all files
pub struct IndexManager {
    conn: Connection,
}

impl IndexManager {
    /// Queries the index for a particular document. Returns the deserialised document index
    /// entry if found.
    async fn find_document(&self, document: &str) -> Option<IndexedFile> {
        let hash = xxh3_64(document.as_bytes());

        let result = self
            .conn
            .call(move |conn| {
                conn.execute("SELECT * FROM slingshot_files WHERE hash = ?1;", (hash,))
            })
            .await;

        todo!()
    }

    /// Inserts a document into the index. TODO make this return a Result
    fn insert_document(&self, document: &str, completion: HashSet<CompletionToken>) {
        let hash = xxh3_64(document.as_bytes());
    }

    pub async fn new() -> Self {
        debug!("IndexManager is creating SQLite database");

        // TODO just using an in memory DB for now
        let conn = Connection::open_in_memory().await.unwrap();

        // TODO create DB table

        IndexManager { conn }
    }
}
