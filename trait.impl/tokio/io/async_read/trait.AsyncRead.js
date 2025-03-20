(function() {
    var implementors = Object.fromEntries([["hyper_util",[["impl&lt;T&gt; <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"hyper_util/rt/tokio/struct.TokioIo.html\" title=\"struct hyper_util::rt::tokio::TokioIo\">TokioIo</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"hyper/rt/io/trait.Read.html\" title=\"trait hyper::rt::io::Read\">Read</a>,</div>"]]],["tokio",[]],["tokio_util",[["impl&lt;L, R&gt; <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"enum\" href=\"tokio_util/either/enum.Either.html\" title=\"enum tokio_util::either::Either\">Either</a>&lt;L, R&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a>,\n    R: <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a>,</div>"],["impl&lt;R: <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>])&gt; <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_util/io/struct.InspectReader.html\" title=\"struct tokio_util::io::InspectReader\">InspectReader</a>&lt;R, F&gt;"],["impl&lt;S, B, E&gt; <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_util/io/struct.StreamReader.html\" title=\"struct tokio_util::io::StreamReader\">StreamReader</a>&lt;S, B&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;B, E&gt;&gt;,\n    B: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,</div>"],["impl&lt;S: <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a>&gt; <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_util/io/struct.SinkWriter.html\" title=\"struct tokio_util::io::SinkWriter\">SinkWriter</a>&lt;S&gt;"],["impl&lt;W: <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a>, F&gt; <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_util/io/struct.InspectWriter.html\" title=\"struct tokio_util::io::InspectWriter\">InspectWriter</a>&lt;W, F&gt;"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[450,13,3207]}