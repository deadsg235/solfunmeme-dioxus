//! This module is for cleaning and processing the founding documents.
//! It extracts code snippets, creates content-addressable versions,
//! counts tokens, and tests the code.

use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};


/// new
// This module is for cleaning and processing the founding documents.  
// It extracts code snippets, creates content-addressable versions,  
// counts tokens, and tests the code.  
  
use markdown::{to_mdast, ParseOptions};  
use markdown::mdast::{Node, Code, InlineCode};  

  
  
  
  
// Keep your existing save_summary and main functions unchanged
//// old



