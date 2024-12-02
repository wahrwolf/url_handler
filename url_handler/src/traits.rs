use anyhow::Result;
use url::Url;

pub trait StringRecordHandler {
    fn fetch_record_from_url(&self, url: &Url) -> Result<Option<String>>;
    fn push_record_to_url(&self, url: &Url, record: &String) -> Result<()>;
}
