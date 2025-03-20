(function() {
    var type_impls = Object.fromEntries([["ordered_multimap",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BuildHasher-for-RandomState\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#81\">Source</a></span><a href=\"#impl-BuildHasher-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Hasher\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#82\">Source</a><a href=\"#associatedtype.Hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#associatedtype.Hasher\" class=\"associatedtype\">Hasher</a> = <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.DefaultHasher.html\" title=\"struct std::hash::random::DefaultHasher\">DefaultHasher</a></h4></section></summary><div class='docblock'>Type of the hasher that will be created.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.build_hasher\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#85\">Source</a><a href=\"#method.build_hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#tymethod.build_hasher\" class=\"fn\">build_hasher</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.DefaultHasher.html\" title=\"struct std::hash::random::DefaultHasher\">DefaultHasher</a></h4></section></summary><div class='docblock'>Creates a new hasher. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#tymethod.build_hasher\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_one\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.71.0\">1.71.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#694-697\">Source</a></span><a href=\"#method.hash_one\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#method.hash_one\" class=\"fn\">hash_one</a>&lt;T&gt;(&amp;self, x: T) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a><div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#associatedtype.Hasher\" title=\"type core::hash::BuildHasher::Hasher\">Hasher</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Calculates the hash of a single value. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#method.hash_one\">Read more</a></div></details></div></details>","BuildHasher","ordered_multimap::list_ordered_multimap::RandomState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-RandomState\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#35\">Source</a></span><a href=\"#impl-Clone-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#35\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","ordered_multimap::list_ordered_multimap::RandomState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-RandomState\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.16.0\">1.16.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#157\">Source</a></span><a href=\"#impl-Debug-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#158\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","ordered_multimap::list_ordered_multimap::RandomState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-RandomState\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#148\">Source</a></span><a href=\"#impl-Default-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#151\">Source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h4></section></summary><div class=\"docblock\"><p>Constructs a new <code>RandomState</code>.</p>\n</div></details></div></details>","Default","ordered_multimap::list_ordered_multimap::RandomState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RandomState\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#41\">Source</a><a href=\"#impl-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/std/hash/random.rs.html#56\">Source</a></span><h4 class=\"code-header\">pub fn <a href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html\" title=\"struct std::hash::random::RandomState\">RandomState</a></h4></section></summary><div class=\"docblock\"><p>Constructs a new <code>RandomState</code> that is initialized with random keys.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::hash::RandomState;\n\n<span class=\"kw\">let </span>s = RandomState::new();</code></pre></div>\n</div></details></div></details>",0,"ordered_multimap::list_ordered_multimap::RandomState"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[11759]}