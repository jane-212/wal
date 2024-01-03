use reqwest::{Client, ClientBuilder};
use scraper::Html;
use scraper::Selector;
use serde::Serialize;

use crate::error::IResult;

pub struct Porn {
    client: Client,
    selectors: Selectors,
}

impl Porn {
    const BASE_URL: &str = "https://91porny.com";

    pub fn new() -> IResult<Self> {
        let client = ClientBuilder::new().build()?;
        let selectors = Selectors::new()?;

        Ok(Self { client, selectors })
    }

    pub async fn hot(&self, page: u32) -> IResult<Vec<Video>> {
        assert!(
            page > 0 && page < 4,
            "page of hot should be bigger than 0 and smaller than 4"
        );

        let url = format!("{}/video/category/hot-list/{}", Self::BASE_URL, page);
        let videos = self.get_videos_by_url(&url).await?;

        Ok(videos)
    }

    async fn get_videos_by_url(&self, url: &str) -> IResult<Vec<Video>> {
        let res = self.client.get(url).send().await?.text().await?;
        let doc = Html::parse_document(&res);

        let mut videos = Vec::new();
        for item in doc.select(&self.selectors.item) {
            let Some(href) = item
                .select(&self.selectors.href)
                .next()
                .and_then(|a| a.attr("href"))
            else {
                continue;
            };
            let href = format!("{}{}", Self::BASE_URL, href);

            let Some(cover) = item
                .select(&self.selectors.cover)
                .next()
                .and_then(|img| img.attr("style"))
            else {
                continue;
            };
            let cover = &cover[23..cover.len()-2];
            let cover = format!("https:{}", cover);

            let Some(title) = item
                .select(&self.selectors.title)
                .next()
                .and_then(|img| img.attr("title"))
            else {
                continue;
            };

            let Some(small) = item.select(&self.selectors.cost).next() else {
                continue;
            };
            let cost = small.inner_html();

            videos.push(Video {
                href,
                cover,
                title: title.to_string(),
                cost,
            })
        }

        Ok(videos)
    }
}

#[derive(Debug, Serialize)]
pub struct Video {
    href: String,
    cover: String,
    title: String,
    cost: String,
}

struct Selectors {
    item: Selector,
    href: Selector,
    cover: Selector,
    title: Selector,
    cost: Selector,
}

impl Selectors {
    fn new() -> IResult<Self> {
        Ok(Self {
            item: Selector::parse(
                "main#main>div#videoListPage>div.row>div.col-60>div.row>div.colVideoList>div.video-elem",
            )?,
            href: Selector::parse("a.display")?,
            cover: Selector::parse("a.display>div.img")?,
            title: Selector::parse("a.display>div.img")?,
            cost: Selector::parse("a.display>small.layer")?,
        })
    }
}
