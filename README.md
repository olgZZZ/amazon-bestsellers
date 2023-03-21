# Scraper

## The program scans information from the pages of a given website

This application implements site scraping functionality to scan a list of companies and extract information such as company name, address, full name of the director, birthday, date of appointment, etc. All the collected data is saved in a CSV file format.

The application utilizes the "thirtyfour" library, which is a Rust-based Selenium/WebDriver library for automating UI testing on websites. Its features include creating new browser sessions directly via WebDriver (e.g., chromedriver), finding elements using common selectors such as Id, Class, CSS, Tag, XPath, and sending keys to elements, including key combinations.

The application also employs the "Tokio" library, which is a runtime for writing reliable, asynchronous, and slim applications with the Rust programming language. In addition, it uses the "Serde" library, which is a framework for efficiently and generically serializing and deserializing Rust data structures.
