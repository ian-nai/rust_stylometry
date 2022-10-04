# rust_stylometry
A package to perform stylometry operations on given texts, written in Rust. The package includes implementations of Mendenhall's graphing of word lengths, the Kilgariff chi-squared algorithm, and an algorithm to find words that occur in only one text out of two being compared (hapax legomena).

## How to Use
The main functions in the package can be called like so:

```
use stylometry::mendenhall_file;
// or, if graphing a string rather than reading from a file: use stylometry::mendenhall_string;

mendenhall("path/to/file.txt"); // this will output a .png of a line graph of the text's word lengths
```
```
use stylometry::scatterplot;
// or, if graphing a string rather than reading from a file: use stylometry::mendenhall_string;

scatterplot("path/to/file.txt"); // this will output a .png showing a scatterplot of the text's word lengths, 
// similar to the line graph in the mendenhall function
```

```
use stylometry::kilgariff;

kilgariff("path/to/file1_known_author.txt", "path/to/file2_disputed_author.txt"); // this will print the result of the Kilgariff 
// chi-squared formula for texts you're comparing
```

```
use stylometry::hapax;

hapax("path/to/file1.txt", "path/to/file2.txt"); // this will return a vec of words only found in file1 
// (and which only occur once in that file)
```
```
use stylometry::hapax_single;

hapax("path/to/file1.txt"); // this will return a vec of words only occurring once in file1

```
You can also get a vec containing tuples of word lengths and how often they occur (e.g., (4, 15) would show that four-letter words occur fifteen times) using the following function:
```
use stylometry::unique_and_total;

unique_and_total("path/to/file1.txt");
```
