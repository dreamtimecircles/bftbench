searchState.loadedDescShard("yaml_rust2", 0, "YAML 1.2 implementation in pure Rust.\nYAML serialization helpers.\nHome to the YAML Parser.\nHome to the YAML Scanner.\nYAML objects manipulation utilities.\nAn error when emitting YAML.\nA convenience alias for emitter functions that may fail …\nContains the error value\nA formatting error.\nContains the success value\nThe YAML serializer.\nSet ‘compact inline notation’ on or off, as described …\nDump Yaml to an output stream.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDetermine if this emitter is using ‘compact inline …\nDetermine if this emitter will emit multiline strings when …\nRender strings containing multiple lines in literal style.\nCreate a new emitter serializing into <code>writer</code>.\nA YAML Alias.\nThe YAML end document directive (<code>...</code>).\nThe YAML start document directive (<code>---</code>).\nContains the error value\nAn event generated by the YAML parser.\nTrait to be implemented in order to use the low-level …\nThe end of a YAML mapping (object, hash).\nThe start of a YAML mapping (object, hash).\nTrait to be implemented for using the low-level parsing …\nReserved for internal use.\nContains the success value\nA convenience alias for a <code>Result</code> of a parser event.\nA YAML parser.\nValue, style, anchor id, tag\nThe end of a YAML sequence (array).\nThe start of a YAML sequence (array).\nLast event that will be generated by the parser. Signals …\nEvent generated at the very beginning of parsing.\nA YAML tag.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nHandle of the tag (<code>!</code> included).\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWhether to keep tags across multiple documents when …\nLoad the YAML from the stream in <code>self</code>, pushing events into …\nCreate a new instance of a parser from the given input of …\nCreate a new instance of a parser from a &amp;str.\nTry to load the next event and return it, consuming it …\nHandler called for each YAML event that is emitted by the …\nHandler called for each event that occurs.\nTry to load the next event and return it, but do not …\nThe suffix of the tag.\nA reference to an anchor.\nA YAML anchor (<code>&amp;</code>/<code>*</code>).\nEnd of the corresponding <code>BlockSequenceStart</code> or …\nAn entry in a block sequence (c.f.: …\nThe start of a sequence mapping.\nThe start of a sequence block.\nChomping, how final line breaks and trailing empty lines …\nThe final line break is preserved, but trailing empty …\nThe end of a YAML document (<code>...</code>).\nThe start of a YAML document (<code>---</code>).\nA YAML double quoted scalar.\nContains the error value\nAn entry in a flow sequence (c.f.: …\nEnd of an inline mapping.\nStart of an inline mapping (<code>{ a: b, c: d }</code>).\nEnd of an inline array.\nStart of an inline array (<code>[ a, b ]</code>).\nA YAML folded block (<code>&gt;</code> block).\nThe final line break and trailing empty lines are included.\nA key in a mapping.\nA YAML literal block (<code>|</code> block).\nA location in a yaml document.\nContains the success value\nA YAML plain scalar.\nA regular YAML scalar.\nAn error that occurred while scanning.\nA convenience alias for scanner functions that may fail …\nThe YAML scanner.\nA YAML single quoted scalar.\nThe end of the stream, EOF.\nThe start of the stream. Sent first, before even …\nThe final line break and any trailing empty lines are …\nThe encoding of the input. Currently, only UTF-8 is …\nThe style as which the scalar was written in the YAML …\nA YAML tag (starting with bangs <code>!</code>).\nA YAML tag directive (e.g.: <code>!!str</code>, <code>!foo!bar</code>, …).\nA scanner token.\nThe contents of a scanner token.\nUTF-8 encoding.\nA value in a mapping.\nA YAML version directive.\nReturn the column of the marker in the source.\nFetch tokens from the token stream.\nFetch the next token in the stream.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a copy of the last error that was encountered, if any.\nReturn the index (in bytes) of the marker in the source.\nReturn the information string describing the error that …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturn the line of the marker in the source.\nGet the current position in the input stream.\nReturn the marker pointing to the error in the source.\nCreate a new error from a location and an error string.\nCreates the YAML tokenizer.\nCreate a new error from a location and an error string.\nReturn the next token in the stream.\nReturn whether the <code>TokenType::StreamEnd</code> event has been …\nReturn whether the <code>TokenType::StreamStart</code> event has been …\nAlias, not fully supported yet.\nThe type contained in the <code>Yaml::Array</code> variant. This …\nYAML array, can be accessed as a <code>Vec</code>.\nAccessing a nonexistent node via the Index trait returns …\nYAML bool, e.g. <code>true</code> or <code>false</code>.\nCall the user-supplied function upon decoding malformation.\nA decoding error (e.g.: Invalid UTF-8).\nThe type contained in the <code>Yaml::Hash</code> variant. This …\nYAML hash, can be accessed as a <code>LinkedHashMap</code>.\nAn I/O error.\nIgnore the offending bytes, remove them from the output.\nYAML int is stored as i64.\nAn error that happened when loading a YAML document.\nYAML null, e.g. <code>null</code> or <code>~</code>.\nFloat types are stored as String and parsed on demand. …\nReplace them with the Unicode REPLACEMENT CHARACTER.\nAn error within the scanner. This indicates a malformed …\nError out.\nYAML scalar.\nThe behavior <code>YamlDecoder</code> must have when an decoding error …\nThe signature of the function to call when using …\nA YAML node is stored as this <code>Yaml</code> enumeration, which …\n<code>YamlDecoder</code> is a <code>YamlLoader</code> builder that allows you to …\nAn iterator over a <code>Yaml</code> node.\nMain structure for quickly parsing YAML.\nGet a copy of the inner object in the YAML enum if it is a …\nReturn the <code>f64</code> value contained in this YAML node.\nGet a reference to the inner object in the YAML enum if it …\nGet a copy of the inner object in the YAML enum if it is a …\nGet a mutable reference to the inner object in the YAML …\nGet a mutable reference to the inner object in the YAML …\nGet a reference to the inner object in the YAML enum if it …\nGet a reference to the inner object in the YAML enum if it …\nSee <code>or</code> for behavior. This performs the same operations, …\nRun the decode operation with the source and trap the …\nReturn a reference to the parsed Yaml documents.\nSet the behavior of the decoder when the encoding is …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert a string to a <code>Yaml</code> node.\nPerform indexing if <code>self</code> is a mapping.\nPerform indexing if <code>self</code> is a sequence or a mapping.\nPerform indexing if <code>self</code> is a sequence or a mapping.\nPerform indexing if <code>self</code> is a mapping.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGet the inner object in the YAML enum if it is a <code>$t</code>.\nReturn the <code>f64</code> value contained in this YAML node.\nGet the inner object in the YAML enum if it is a <code>$t</code>.\nGet the inner object in the YAML enum if it is a <code>$t</code>.\nExtract the <code>Array</code> from <code>self</code> and iterate over it.\nGet the inner object in the YAML enum if it is a <code>$t</code>.\nGet the inner object in the YAML enum if it is a <code>$t</code>.\nReturn whether <code>self</code> is a <code>Yaml::Array</code> node.\nReturn whether <code>self</code> is a <code>Yaml::BadValue</code> node.\nReturn whether <code>self</code> is a <code>Yaml::Hash</code> node.\nReturn whether <code>self</code> is a <code>Yaml::Null</code> node.\nLoad the contents of the given iterator as a set of YAML …\nLoad the contents from the specified Parser as a set of …\nLoad the given string as a set of YAML documents.\nIf a value is null or otherwise bad (see variants), …\nCreate a <code>YamlDecoder</code> decoding the given source.")