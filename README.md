# Web app to demonstrate the fuzzymatch Rust library

A simple web app built using rocket.rs to provide a REST API around
my [fuzzymatch](https://github.com/batuwa/fuzzymatch) library. A basic HTML/CSS/Jquery frontend provides a simple search box 
with functionality similar to google autocomplete.

![FuzzySearch](https://github.com/batuwa/fuzzysearch/blob/master/public/screenshot.png "FuzzySearch Screenshot") 

## Usage

The rocket framework needs nightly version of rust, so make sure you have that enabled in your project. 

The web app can be simply run with `cargo run` and going to `http://localhost:8000`
in your browser. 

## Sample Data

A list of Top 5000 TMDB (The Movie Database) is provided in the `data` folder. Replace that with the your list
and open that file from the `search.rs` module if you want. 
