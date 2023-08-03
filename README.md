# PDFNetworkGraph

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffile_upload)](https://raeesmullah.github.io/PDFNetworkGraph/)

## About
PDFNetworkGraph aka PDFng is a tool that analyzes PDFs and generates a network graph allowing users to visualize the connections across their documents. The goal of PDFng is to produce a top-level visual that compares both the semantics and co-occurences of words across all provided PDFs. This prototype uses only co-occurences. 

## How To
Load PDFs using the Browse button.
Click on nodes to see their appearances listed in the console. 

## How It Works
PDFng is a [WebAssembly](https://webassembly.org/) application written in the [Rust programming language](https://www.rust-lang.org/) using the [Yew web framework](https://yew.rs/). This allows for high performance, and client-side compute. The text within PDFs is extracted using [pdfium-render](https://github.com/ajrcarey/pdfium-render), a Rust wrapper around PDFium (C++ PDF library from Google). This project links to the WASM build of PDFium provided [here](https://github.com/paulocoutinhox/pdfium-lib). After extraction, text is processed and <svg> elements are dynamically rendered to the DOM via Yew. 

## Future Plans
- Semantic analysis (plan to use the [Burn](https://burn-rs.github.io/) machine learning framework)
- GUI for maximum number of words from a single document (currently set to 11)
- Scrollable canvas (currently only fits 9 documents)
- Extract and analyze all PDF objects, not just text
