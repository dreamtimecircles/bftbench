(function() {
    var type_impls = Object.fromEntries([["tonic",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRef%3C%5Bu8%5D%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#156-160\">Source</a><a href=\"#impl-AsRef%3C%5Bu8%5D%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#157-159\">Source</a><a href=\"#method.as_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref\" class=\"fn\">as_ref</a>(&amp;self) -&gt; &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>] <a href=\"#\" class=\"tooltip\" data-notable-ty=\"&amp;[u8]\">ⓘ</a></h4></section></summary><div class='docblock'>Converts this type into a shared reference of the (usually inferred) input type.</div></details></div></details>","AsRef<[u8]>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRef%3Cstr%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#150-154\">Source</a><a href=\"#impl-AsRef%3Cstr%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#151-153\">Source</a><a href=\"#method.as_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref\" class=\"fn\">as_ref</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Converts this type into a shared reference of the (usually inferred) input type.</div></details></div></details>","AsRef<str>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Borrow%3Cstr%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#162-166\">Source</a><a href=\"#impl-Borrow%3Cstr%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.borrow\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#163-165\">Source</a><a href=\"#method.borrow\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow\" class=\"fn\">borrow</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Immutably borrows from an owned value. <a href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow\">Read more</a></div></details></div></details>","Borrow<str>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#impl-Clone-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#168-172\">Source</a><a href=\"#impl-Debug-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#169-171\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, fmt: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#174-178\">Source</a><a href=\"#impl-Display-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#175-177\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, fmt: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26MetadataKey%3CVE%3E%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#187-191\">Source</a><a href=\"#impl-From%3C%26MetadataKey%3CVE%3E%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#188-190\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(src: &amp;'a <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;) -&gt; <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&'a MetadataKey<VE>>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FromStr-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#142-148\">Source</a><a href=\"#impl-FromStr-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html\" title=\"trait core::str::traits::FromStr\">FromStr</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Err\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#143\">Source</a><a href=\"#associatedtype.Err\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err\" class=\"associatedtype\">Err</a> = <a class=\"struct\" href=\"tonic/metadata/errors/struct.InvalidMetadataKey.html\" title=\"struct tonic::metadata::errors::InvalidMetadataKey\">InvalidMetadataKey</a></h4></section></summary><div class='docblock'>The associated error which can be returned from parsing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_str\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#145-147\">Source</a><a href=\"#method.from_str\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str\" class=\"fn\">from_str</a>(s: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"tonic/metadata/errors/struct.InvalidMetadataKey.html\" title=\"struct tonic::metadata::errors::InvalidMetadataKey\">InvalidMetadataKey</a>&gt;</h4></section></summary><div class='docblock'>Parses a string <code>s</code> to return a value of this type. <a href=\"https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str\">Read more</a></div></details></div></details>","FromStr","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#impl-Hash-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __H</a>)</h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237\">Source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;[Self], state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#37-140\">Source</a><a href=\"#impl-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_bytes\" class=\"method\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#41-55\">Source</a><h4 class=\"code-header\">pub fn <a href=\"tonic/metadata/struct.MetadataKey.html#tymethod.from_bytes\" class=\"fn\">from_bytes</a>(src: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"tonic/metadata/errors/struct.InvalidMetadataKey.html\" title=\"struct tonic::metadata::errors::InvalidMetadataKey\">InvalidMetadataKey</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Converts a slice of bytes to a <code>MetadataKey</code>.</p>\n<p>This function normalizes the input.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_static\" class=\"method\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#102-112\">Source</a><h4 class=\"code-header\">pub fn <a href=\"tonic/metadata/struct.MetadataKey.html#tymethod.from_static\" class=\"fn\">from_static</a>(src: &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Converts a static string to a <code>MetadataKey</code>.</p>\n<p>This function panics when the static string is a invalid metadata key.</p>\n<p>This function requires the static string to only contain lowercase\ncharacters, numerals and symbols, as per the HTTP/2.0 specification\nand header names internal representation within this library.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"comment\">// Parsing a metadata key\n</span><span class=\"kw\">let </span>CUSTOM_KEY: <span class=\"kw-2\">&amp;</span><span class=\"lifetime\">'static </span>str = <span class=\"string\">\"custom-key\"</span>;\n\n<span class=\"kw\">let </span>a = AsciiMetadataKey::from_bytes(<span class=\"string\">b\"custom-key\"</span>).unwrap();\n<span class=\"kw\">let </span>b = AsciiMetadataKey::from_static(CUSTOM_KEY);\n<span class=\"macro\">assert_eq!</span>(a, b);</code></pre></div>\n\n<div class=\"example-wrap should_panic\"><a href=\"#\" class=\"tooltip\" title=\"This example panics\">ⓘ</a><pre class=\"rust rust-example-rendered\"><code><span class=\"comment\">// Parsing a metadata key that contains invalid symbols(s):\n</span>AsciiMetadataKey::from_static(<span class=\"string\">\"content{}{}length\"</span>); <span class=\"comment\">// This line panics!</span></code></pre></div>\n\n<div class=\"example-wrap should_panic\"><a href=\"#\" class=\"tooltip\" title=\"This example panics\">ⓘ</a><pre class=\"rust rust-example-rendered\"><code><span class=\"comment\">// Parsing a metadata key that contains invalid uppercase characters.\n</span><span class=\"kw\">let </span>a = AsciiMetadataKey::from_static(<span class=\"string\">\"foobar\"</span>);\n<span class=\"kw\">let </span>b = AsciiMetadataKey::from_static(<span class=\"string\">\"FOOBAR\"</span>); <span class=\"comment\">// This line panics!</span></code></pre></div>\n\n<div class=\"example-wrap should_panic\"><a href=\"#\" class=\"tooltip\" title=\"This example panics\">ⓘ</a><pre class=\"rust rust-example-rendered\"><code><span class=\"comment\">// Parsing a -bin metadata key as an Ascii key.\n</span><span class=\"kw\">let </span>b = AsciiMetadataKey::from_static(<span class=\"string\">\"hello-bin\"</span>); <span class=\"comment\">// This line panics!</span></code></pre></div>\n\n<div class=\"example-wrap should_panic\"><a href=\"#\" class=\"tooltip\" title=\"This example panics\">ⓘ</a><pre class=\"rust rust-example-rendered\"><code><span class=\"comment\">// Parsing a non-bin metadata key as an Binary key.\n</span><span class=\"kw\">let </span>b = BinaryMetadataKey::from_static(<span class=\"string\">\"hello\"</span>); <span class=\"comment\">// This line panics!</span></code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_str\" class=\"method\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#118-120\">Source</a><h4 class=\"code-header\">pub fn <a href=\"tonic/metadata/struct.MetadataKey.html#tymethod.as_str\" class=\"fn\">as_str</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a></h4></section></summary><div class=\"docblock\"><p>Returns a <code>str</code> representation of the metadata key.</p>\n<p>The returned string will always be lower case.</p>\n</div></details></div></details>",0,"tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq%3C%26MetadataKey%3CVE%3E%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#200-205\">Source</a><a href=\"#impl-PartialEq%3C%26MetadataKey%3CVE%3E%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;&amp;'a <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#202-204\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;&amp;'a <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq<&'a MetadataKey<VE>>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq%3C%26str%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#254-261\">Source</a><a href=\"#impl-PartialEq%3C%26str%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#258-260\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Performs a case-insensitive comparison of the string against the header\nname</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq<&'a str>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq%3Cstr%3E-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#214-232\">Source</a><a href=\"#impl-PartialEq%3Cstr%3E-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#229-231\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Performs a case-insensitive comparison of the string against the header\nname</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>content_length = AsciiMetadataKey::from_static(<span class=\"string\">\"content-length\"</span>);\n\n<span class=\"macro\">assert_eq!</span>(content_length, <span class=\"string\">\"content-length\"</span>);\n<span class=\"macro\">assert_eq!</span>(content_length, <span class=\"string\">\"Content-Length\"</span>);\n<span class=\"macro\">assert_ne!</span>(content_length, <span class=\"string\">\"content length\"</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq<str>","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#impl-PartialEq-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<section id=\"impl-Eq-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#impl-Eq-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section>","Eq","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"],["<section id=\"impl-StructuralPartialEq-for-MetadataKey%3CVE%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tonic/metadata/key.rs.html#17\">Source</a><a href=\"#impl-StructuralPartialEq-for-MetadataKey%3CVE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;VE: ValueEncoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"tonic/metadata/struct.MetadataKey.html\" title=\"struct tonic::metadata::MetadataKey\">MetadataKey</a>&lt;VE&gt;</h3></section>","StructuralPartialEq","tonic::metadata::key::AsciiMetadataKey","tonic::metadata::key::BinaryMetadataKey"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[36553]}