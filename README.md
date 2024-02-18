# OrderInsight

**OrderInsight** is a Rust application for analyzing restaurant data to identify guest loyalty and preferences.

## Restaurant Data Analysis with Rust

This Rust program analyzes restaurant data stored in an Excel file to identify guest loyalty and preferences. It reads the data, processes it, and provides insights based on customer orders.

## Installation

To run this program, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from [the official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone this repository:

Navigate to the project directory:

cd restaurant-data-analysis-rust

## Usage
Place your Excel file containing restaurant data in the root directory of the project and name it guest_data.xlsx.
Run the program using Cargo, the Rust package manager:
cargo run

The program will read the data from the Excel file, process it, and display insights such as guest loyalty and food preferences.

## Dependencies
This project uses the following external dependencies:

calamine - For reading data from Excel files.
These dependencies are managed using Cargo, Rust's package manager.

## Approach
Use a Rust library to read data from the Excel file. calamine is a popular library for this purpose.
Parse the data and organize it in a suitable data structure.
Analyze the data to identify repeat orders by the same mobile number.
Generate insights on guest loyalty and preferences based on the analysis.
Optionally, build predictive models to predict future orders or customer behavior.
