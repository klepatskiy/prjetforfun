use crate::app::command::create_short_url::{CreateShortUrlCommand, CreateShortUrlRepository};
use crate::app::query::get_full_url::{GetFullUrlQuery, GetFullUrlRepository};

pub struct Container<R, I>
where
    R: CreateShortUrlRepository,
    I: GetFullUrlRepository,
{
    pub shorten_command: CreateShortUrlCommand<R>,
    pub full_url_query: GetFullUrlQuery<I>,
}

impl<R, I> Container<R, I>
where
    R: CreateShortUrlRepository,
    I: GetFullUrlRepository,
{
    pub fn new(repository_command: R, repository_query: I) -> Self {
        let shorten_command = CreateShortUrlCommand::new(repository_command);
        let full_url_query = GetFullUrlQuery::new(repository_query);

        Container {
            shorten_command,
            full_url_query,
        }
    }
}
