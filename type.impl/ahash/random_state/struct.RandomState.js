(function() {var type_impls = {
"hashbrown":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RandomState\" class=\"impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#171\">source</a><a href=\"#impl-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_random_source\" class=\"method\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#182\">source</a><h4 class=\"code-header\">pub fn <a href=\"ahash/random_state/struct.RandomState.html#tymethod.set_random_source\" class=\"fn\">set_random_source</a>(\n    source: impl RandomSource + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Provides an optional way to manually supply a source of randomness for Hasher keys.</p>\n<p>The provided [RandomSource] will be used to be used as a source of randomness by <a href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a> to generate new states.\nIf this method is not invoked the standard source of randomness is used as described in the Readme.</p>\n<p>The source of randomness can only be set once, and must be set before the first RandomState is created.\nIf the source has already been specified <code>Err</code> is returned with a <code>bool</code> indicating if the set failed because\nmethod was previously invoked (true) or if the default source is already being used (false).</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#201\">source</a><h4 class=\"code-header\">pub fn <a href=\"ahash/random_state/struct.RandomState.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h4></section></summary><div class=\"docblock\"><p>Use randomly generated keys</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.generate_with\" class=\"method\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#210\">source</a><h4 class=\"code-header\">pub fn <a href=\"ahash/random_state/struct.RandomState.html#tymethod.generate_with\" class=\"fn\">generate_with</a>(k0: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, k1: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, k2: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, k3: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>) -&gt; <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h4></section></summary><div class=\"docblock\"><p>Allows for supplying seeds, but each time it is called the resulting state will be different.\nThis is done using a static counter, so it can safely be used with a fixed keys.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_seed\" class=\"method\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#244\">source</a><h4 class=\"code-header\">pub fn <a href=\"ahash/random_state/struct.RandomState.html#tymethod.with_seed\" class=\"fn\">with_seed</a>(key: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h4></section></summary><div class=\"docblock\"><p>Allows for explicitly setting a seed to used.</p>\n<p>Note: This method does not require the provided seed to be strong.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_seeds\" class=\"method\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#254\">source</a><h4 class=\"code-header\">pub const fn <a href=\"ahash/random_state/struct.RandomState.html#tymethod.with_seeds\" class=\"fn\">with_seeds</a>(k0: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, k1: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, k2: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, k3: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>) -&gt; <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h4></section></summary><div class=\"docblock\"><p>Allows for explicitly setting the seeds to used.</p>\n<p>Note: This method is robust against 0s being passed for one or more of the parameters\nor the same value being passed for more than one parameter.</p>\n</div></details></div></details>",0,"hashbrown::map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-RandomState\" class=\"impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#259\">source</a><a href=\"#impl-Default-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#261\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","hashbrown::map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BuildHasher-for-RandomState\" class=\"impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#266\">source</a><a href=\"#impl-BuildHasher-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> for <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.build_hasher\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#298\">source</a><a href=\"#method.build_hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#tymethod.build_hasher\" class=\"fn\">build_hasher</a>(&amp;self) -&gt; <a class=\"struct\" href=\"ahash/fallback_hash/struct.AHasher.html\" title=\"struct ahash::fallback_hash::AHasher\">AHasher</a></h4></section></summary><div class=\"docblock\"><p>Constructs a new <a href=\"ahash/fallback_hash/struct.AHasher.html\" title=\"struct ahash::fallback_hash::AHasher\">AHasher</a> with keys based on this <a href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a> object.\nThis means that two different <a href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a>s will will generate\n<a href=\"ahash/fallback_hash/struct.AHasher.html\" title=\"struct ahash::fallback_hash::AHasher\">AHasher</a>s that will return different hashcodes, but <a href=\"std::hash::Hasher\">Hasher</a>s created from the same <a href=\"std::hash::BuildHasher\">BuildHasher</a>\nwill generate the same hashes for the same input data.</p>\n<h5 id=\"examples\"><a href=\"#examples\">Examples</a></h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>ahash::{AHasher, RandomState};\n<span class=\"kw\">use </span>std::hash::{Hasher, BuildHasher};\n\n<span class=\"kw\">let </span>build_hasher = RandomState::new();\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>hasher_1 = build_hasher.build_hasher();\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>hasher_2 = build_hasher.build_hasher();\n\nhasher_1.write_u32(<span class=\"number\">1234</span>);\nhasher_2.write_u32(<span class=\"number\">1234</span>);\n\n<span class=\"macro\">assert_eq!</span>(hasher_1.finish(), hasher_2.finish());\n\n<span class=\"kw\">let </span>other_build_hasher = RandomState::new();\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>different_hasher = other_build_hasher.build_hasher();\ndifferent_hasher.write_u32(<span class=\"number\">1234</span>);\n<span class=\"macro\">assert_ne!</span>(different_hasher.finish(), hasher_1.finish());</code></pre></div>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Hasher\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#associatedtype.Hasher\" class=\"associatedtype\">Hasher</a> = <a class=\"struct\" href=\"ahash/fallback_hash/struct.AHasher.html\" title=\"struct ahash::fallback_hash::AHasher\">AHasher</a></h4></section></summary><div class='docblock'>Type of the hasher that will be created.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_one\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.71.0\">1.71.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#704-707\">source</a></span><a href=\"#method.hash_one\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#method.hash_one\" class=\"fn\">hash_one</a>&lt;T&gt;(&amp;self, x: T) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a><span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#associatedtype.Hasher\" title=\"type core::hash::BuildHasher::Hasher\">Hasher</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</span></h4></section></summary><div class='docblock'>Calculates the hash of a single value. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html#method.hash_one\">Read more</a></div></details></div></details>","BuildHasher","hashbrown::map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-RandomState\" class=\"impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#165\">source</a><a href=\"#impl-Debug-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#166\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hashbrown::map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-RandomState\" class=\"impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#157\">source</a><a href=\"#impl-Clone-for-RandomState\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ahash/random_state.rs.html#157\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"ahash/random_state/struct.RandomState.html\" title=\"struct ahash::random_state::RandomState\">RandomState</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","hashbrown::map::DefaultHashBuilder"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()