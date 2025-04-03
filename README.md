# Transportation Routing

A repository for tools, documentation, and code related to transportation routing with Overture Maps Foundation data.

## Description

This repository serves as a workspace for developing tools and documentation that help translate Overture Transportation Schema to existing routing engine formats. Currently, we're focusing on integrating with Valhalla routing engine. This is a space to collect code, notebooks, and thoughts until we find better places for them. The repository includes a book on Valhalla's Mjolnir graph tile builder to help understand how to integrate Overture data with Valhalla's routing capabilities.

## Getting Started

### Dependencies

* Valhalla routing engine
* Overture Maps Foundation data
* mdBook (for building and viewing the documentation books)
* Rust toolchain (for running code examples)

### Installing

* Clone this repository
* Follow the installation instructions for each specific tool or example in their respective directories
* For viewing the books, install mdBook: `cargo install mdbook`

### Executing program

* To build and view the Valhalla Mjolnir book:
```
cd docs/books/valhalla-mjolnir
mdbook build
mdbook serve
```
* Open your browser to http://localhost:3000 to view the book

## Tasks

* Develop tools to convert Overture Transportation Schema to Valhalla's input format
* Create documentation on the conversion process and data mapping
* Build example notebooks demonstrating the integration workflow
* Implement validation tools to ensure data quality during conversion
* Develop visualization tools to inspect routing results
* Create test cases with sample Overture data
* Document performance considerations and optimization techniques
* Explore integration with other routing engines beyond Valhalla

## Authors

Contributors names and contact info

ex. Dominique Pizzie  
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

* 0.2
    * Various bug fixes and optimizations
    * See [commit change]() or See [release history]()
* 0.1
    * Initial Release


