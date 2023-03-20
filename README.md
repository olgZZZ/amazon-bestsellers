# Scraper

## The program scans information from the pages of a given website

### 1. Goes into the company.
Eg. First company on this list is:
https://find-and-update.company-information.service.gov.uk/company/09646135

- Extracts Company Name
- Extracts Company Address

### 2. Extracts People (Director) Information
https://find-and-update.company-information.service.gov.uk/company/09646135/officers
- Their Full Names
- Roles
- Dates of Births
- Date Appointed

### 3. Extracts a link to the PDF from filing history for:
ONLY files for Total exemption full accounts
- Date
- File
eg. shorturl.at/amKR4

### 4. From the PDF Financial Statements extract the following data:
into a standardised google sheet format so that each item goes into a column.

the aim is that we create a spreadsheet which has:
Company Name
Company Number
Company Address
Number Directors (Count number people with director roles)
AGE of youngest Director (MIN age from director sheet)

Profit/Loss 2022
Tangible Assets 2022
Stocks 2022
Cash at bank and in Hand 2022
Creditors: Failing in 1 year 2022

Profit/Loss 2021
Tangible Assets 2021
Stocks 2021
Cash at bank and in Hand 2021
Creditors: Failing in 1 year 2021

Profit/Loss 2020
Tangible Assets 2020
Stocks 2020
Cash at bank and in Hand 2020
Creditors: Failing in 1 year 2020

Profit/Loss 2019
Tangible Assets 2019
Stocks 2019
Cash at bank and in Hand 2019
Creditors: Failing in 1 year 2019

Profit/Loss 2018
Tangible Assets 2018
Stocks 2018
Cash at bank and in Hand 2018
Creditors: Failing in 1 year 2018
