use std::time::Duration;

use serde::Serialize;
use thirtyfour::{prelude::*, components::{ElementResolver, Component}};

#[derive(Debug, Clone, Component)]
pub struct CompanyItemComponent {
  base: WebElement,
  #[by(css = "a.a-link-normal span div",  single)]
  title: ElementResolver<WebElement>,
  #[by(css = "a.a-link-normal i span", single)]
  rating: ElementResolver<WebElement>,
  #[by(css = "a.a-link-normal .a-size-small", single)]
  comments: ElementResolver<WebElement>
}

impl CompanyItemComponent {
  pub async fn get_company_name(&self) -> WebDriverResult<String> {
    self.title.resolve().await?.text().await
  }

  pub async fn get_company_address(&self) -> WebDriverResult<String> {
    Ok(self.rating.resolve().await?.inner_html().await?)
  }
  
  pub async fn get_company_officers(&self) -> WebDriverResult<String> {
    Ok(self.rating.resolve().await?.inner_html().await?)
  }
}

#[derive(Serialize)]
pub struct CompanyItem {
  name: String,
  address: String,
  officers: OfficerItem,
}

impl CompanyItem {
  pub async fn from(value: CompanyItemComponent) -> Self {
    Self {
      name: value.get_company_name().await.unwrap(),
      address: value.get_company_address().await.unwrap(),
      officers: value.get_company_officers().await.unwrap(),
    }
  }
}

#[derive(Serialize)]
pub struct OfficerItem {
  name: String,
  role: String,
  birthday: String,
  appointed: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
  let caps = DesiredCapabilities::firefox();
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;
  driver.goto("https://find-and-update.company-information.service.gov.uk/advanced-search/get-results?companyNameIncludes=&companyNameExcludes=&registeredOfficeAddress=&incorporationFromDay=01&incorporationFromMonth=01&incorporationFromYear=2005&incorporationToDay=01&incorporationToMonth=01&incorporationToYear=2017&status=active&sicCodes=77110&type=limited-partnership&type=ltd").await?;

  for _ in 0..2 {
    driver.execute_async(r#"
        window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
        arguments[0]();
      "#,
      vec![]
    ).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
  }

  driver.query(By::ClassName("a-pagination")).first().await?.scroll_into_view().await?;


  let query = driver.query(By::Id("gridItemRoot"));

  let mut csv = csv::Writer::from_path("bestsellers.csv")
    .expect("Failed to open output csv file");
  if let Ok(elems) = query.all_required().await {
    for (i, elem) in elems.into_iter().enumerate() {
      let item = CompanyItemComponent::from(elem);
      println!("{}: Shop item `{}`", i + 1, item.get_company_name().await?);
      csv.serialize(CompanyItem::from(item).await).unwrap();
    }
  } else {
    eprintln!("Failed to find shop items!");
  }

  csv.flush().unwrap();

  driver.quit().await?;
  Ok(())
}
