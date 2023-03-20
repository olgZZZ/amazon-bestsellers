use std::time::Duration;

use serde::Serialize;
use thirtyfour::{prelude::*, components::{ElementResolver, Component}};

// #[derive(Debug, Clone, Component)]
// pub struct CompanyItemComponent {
//   base: WebElement,
//   #[by(class = "govuk-link", single)]
//   name: ElementResolver<WebElement>,
//   #[by(class = "govuk-list govuk-!-font-size-16")]
//   address: ElementResolver<WebElement>,
//   // #[by(css = "a.a-link-normal .a-size-small", single)]
//   // officers: ElementResolver<WebElement>
// }

// impl CompanyItemComponent {
//   pub async fn get_company_name(&self) -> WebDriverResult<String> {
//     Ok(self.name.resolve().await?.text().await?.split('\n').next().unwrap().to_string())
//   }

//   pub async fn get_company_address(&self) -> WebDriverResult<String> {
//     let address = self.base.find(By::Tag("li")).await?;
//     Ok(address.to_string())
//     // Ok(self.name.resolve().await?.text().await?)
//   }
// }

// #[derive(Serialize)]
// pub struct CompanyItem {
//   name: String,
//   address: String,
//   // officers: OfficerItem,
// }

// impl CompanyItem {
//   pub async fn from(value: CompanyItemComponent) -> Self {
//     Self {
//       name: value.get_company_name().await.unwrap(),
//       address: value.get_company_address().await.unwrap(),
//     }
//   }
// }

#[derive(Debug, Clone, Component)]
pub struct OfficerItemComponent {
  base: WebElement,
  #[by(class = "appointment-1", single)]
  name: ElementResolver<WebElement>,
  #[by(id = "officer-role-1", single)]
  role: ElementResolver<WebElement>,
  #[by(id = "officer-date-of-birth-1", single)]
  birthday: ElementResolver<WebElement>,
  #[by(id = "officer-appointed-on-1", single)]
  appointed: ElementResolver<WebElement>
}

impl OfficerItemComponent {
  pub async fn get_officer_name(&self) -> WebDriverResult<String> {
    Ok(self.name.resolve().await?.text().await?)
  }

  pub async fn get_officer_role(&self) -> WebDriverResult<String> {
    Ok(self.role.resolve().await?.text().await?)
  }
  
  pub async fn get_officer_birthday(&self) -> WebDriverResult<String> {
    Ok(self.birthday.resolve().await?.text().await?)
  }

  pub async fn get_officer_appointed(&self) -> WebDriverResult<String> {
    Ok(self.appointed.resolve().await?.text().await?)
  }
}

#[derive(Serialize)]
pub struct OfficerItem {
  name: String,
  role: String,
  birthday: String,
  appointed: String,
}

impl OfficerItem {
  pub async fn from(value: OfficerItemComponent) -> Self {
    Self {
      name: value.get_officer_name().await.unwrap(),
      role: value.get_officer_role().await.unwrap(),
      birthday: value.get_officer_birthday().await.unwrap(),
      appointed: value.get_officer_appointed().await.unwrap(),
    }
  }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
  let caps = DesiredCapabilities::firefox();
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;
  driver.goto("https://find-and-update.company-information.service.gov.uk/company/09646135/officers").await?;

  for _ in 0..2 {
    driver.execute_async(r#"
        window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
        arguments[0]();
      "#,
      vec![]
    ).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
  }

  driver.query(By::ClassName("push")).first().await?.scroll_into_view().await?;


  let query = driver.query(By::ClassName("govuk-tabs__panel"));

  let mut csv = csv::Writer::from_path("companies.csv")
    .expect("Failed to open output csv file");
  if let Ok(elems) = query.all_required().await {
    for (i, elem) in elems.into_iter().enumerate() {
      let item = OfficerItemComponent::from(elem);
      println!("{}: Officer item `{}`", i + 1, item.get_officer_name().await?);
      csv.serialize(OfficerItem::from(item).await).unwrap();
    }
  } else {
    eprintln!("Failed to find company items!");
  }

  csv.flush().unwrap();

  driver.quit().await?;
  Ok(())
}
