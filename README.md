# Scraper

## The program scans information from the pages of a given website

This application implements the functionality of site scraping. The following information about the company such as - company name, company address, full name of the director, his birthday, date of appointment, etc. All information is saved in a file in CSV format. 

The application uses the "thirtyfour" library. This is a Selenium / WebDriver library for Rust, for automated website UI testing. 
Features:
  - Create new browser session directly via WebDriver (e.g. chromedriver)
  - Find elements (via all common selectors e.g. Id, Class, CSS, Tag, XPath)
  - Send keys to elements, including key-combinations

Also using "Tokio" framework. A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language.
"Serde" is a framework for _ser_ializing and _de_serializing Rust data structures efficiently and generically.
