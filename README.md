# PDFNetworkGraph

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffile_upload)](https://raeesmullah.github.io/PDFNetworkGraph/)

PDFNetworkGraph aka PDFng is a tool that analyzes PDFs and generates a network graph allowing users to visualize the connections across their documents. The goal of PDFng is to produce a top-level visual that compares both the semantics and co-occurences of words across all provided PDFs. This prototype uses only co-occurences. 

## How To
1. Click on demo or the link under About
2. Click Browse.. in the top-left corner
3. Select the PDFs for use
4. Wait for the visual to load (documents over 20 pages will take longer)
5. Click on the visual's nodes to list the node's appearances in the browser's console
6. Add more documents by repeating 2-4
7. Reset the application by refreshing the browser

## How It Works
PDFng is a [WebAssembly](https://webassembly.org/) application written in the [Rust programming language](https://www.rust-lang.org/) using the [Yew web framework](https://yew.rs/). This allows for high performance, and client-side compute. The text within PDFs is extracted using [pdfium-render](https://github.com/ajrcarey/pdfium-render), a Rust wrapper around PDFium (C++ PDF library from Google). This project links to the WASM build of PDFium provided [here](https://github.com/paulocoutinhox/pdfium-lib). After extraction, text is processed and <svg> elements are dynamically rendered to the DOM via Yew. 

## Future Plans
- Semantic analysis (plan to use the [Burn](https://burn-rs.github.io/) machine learning framework)
- GUI for maximum number of words from a single document (currently set to 11)
- Scrollable canvas (currently only fits 9 documents)
- Extract and analyze all PDF objects, not just text
