    let create_zip = |snippets: &[CodeSnippet]| -> Result<String, JsValue> {
        let mut buffer = Vec::new();
        {
            let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buffer));
            let options = FileOptions::default();
            for (i, snippet) in snippets.iter().enumerate() {
                let file_name = format!("snippet_{}_{}.txt", i, snippet.language);
                zip.start_file(file_name, options)?;
                zip.write_all(snippet.content.as_bytes())?;
            }
            zip.finish()?;
        }
        let blob_parts = Array::new();
        blob_parts.push(&JsValue::from(js_sys::Uint8Array::from(buffer.as_slice())));
        let mut blob_property_bag = BlobPropertyBag::new();
        blob_property_bag.type_("application/zip");
        let blob = Blob::new_with_buffer_source_sequence_and_options(&blob_parts, &blob_property_bag)?;
        let url = Url::create_object_url_with_blob(&blob)?;
        Ok(url)
    };
