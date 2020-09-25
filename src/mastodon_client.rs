use std::borrow::Cow;

use crate::{
    entities::prelude::*,
    errors::Result,
    http_send::{HttpSend, HttpSender},
    media_builder::MediaBuilder,
    page::Page,
    requests::{
        AddFilterRequest, AddPushRequest, StatusesRequest, UpdateCredsRequest, UpdatePushRequest,
    },
    status_builder::NewStatus,
};

/// Represents the set of methods that a Mastodon Client can do, so that
/// implementations might be swapped out for testing
#[allow(unused)]
pub trait MastodonClient<H: HttpSend = HttpSender> {
    /// Type that wraps streaming API streams
    type Stream: Iterator<Item = Event>;

    /// GET /api/v1/favourites
    fn favourites(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/blocks
    fn blocks(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/domain_blocks
    fn domain_blocks(&self) -> Result<Page<String, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/follow_requests
    fn follow_requests(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/home
    fn get_home_timeline(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/custom_emojis
    fn get_emojis(&self) -> Result<Page<Emoji, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/mutes
    fn mutes(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/notifications
    fn notifications(&self) -> Result<Page<Notification, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/reports
    fn reports(&self) -> Result<Page<Report, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/followers
    fn followers(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/following
    fn following(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/reblogged_by
    fn reblogged_by(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/favourited_by
    fn favourited_by(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/domain_blocks
    fn unblock_domain(&self, domain: String) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/instance
    fn instance(&self) -> Result<Instance> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/verify_credentials
    fn verify_credentials(&self) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/reports
    fn report(&self, account_id: &str, status_ids: Vec<&str>, comment: String) -> Result<Report> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/domain_blocks
    fn block_domain(&self, domain: String) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/follow_requests/authorize
    fn authorize_follow_request(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/follow_requests/reject
    fn reject_follow_request(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/search
    fn search<'a>(&self, q: &'a str, resolve: bool) -> Result<SearchResult> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v2/search
    fn search_v2<'a>(&self, q: &'a str, resolve: bool) -> Result<SearchResultV2> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/follows
    fn follows(&self, uri: Cow<'static, str>) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/media
    fn media(&self, media_builder: MediaBuilder) -> Result<Attachment> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/notifications/clear
    fn clear_notifications(&self) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/notifications/dismiss
    fn dismiss_notification(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id
    fn get_account(&self, id: &str) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/:id/follow
    fn follow(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/:id/unfollow
    fn unfollow(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/block
    fn block(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/unblock
    fn unblock(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/mute
    fn mute(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/unmute
    fn unmute(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/notifications/:id
    fn get_notification(&self, id: &str) -> Result<Notification> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id
    fn get_status(&self, id: &str) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/context
    fn get_context(&self, id: &str) -> Result<Context> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/card
    fn get_card(&self, id: &str) -> Result<Card> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/reblog
    fn reblog(&self, id: &str) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/unreblog
    fn unreblog(&self, id: &str) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/favourite
    fn favourite(&self, id: &str) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/unfavourite
    fn unfavourite(&self, id: &str) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/statuses/:id
    fn delete_status(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// PATCH /api/v1/accounts/update_credentials
    fn update_credentials(&self, builder: &mut UpdateCredsRequest) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses
    fn new_status(&self, status: NewStatus) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/public?local=true
    fn get_local_timeline(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/public?local=false
    fn get_federated_timeline(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/tag/:hashtag
    fn get_hashtag_timeline(&self, hashtag: &str, local: bool) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/statuses
    fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status, H>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/relationships
    fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/search?q=:query&limit=:limit&following=:following
    fn search_accounts(
        &self,
        query: &str,
        limit: Option<u64>,
        following: bool,
    ) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/push/subscription
    fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
        unimplemented!("This method was not implemented");
    }
    /// PUT /api/v1/push/subscription
    fn update_push_data(&self, request: &UpdatePushRequest) -> Result<Subscription> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/push/subscription
    fn get_push_subscription(&self) -> Result<Subscription> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/push/subscription
    fn delete_push_subscription(&self) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/filters
    fn get_filters(&self) -> Result<Vec<Filter>> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/filters
    fn add_filter(&self, request: &mut AddFilterRequest) -> Result<Filter> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/filters/:id
    fn get_filter(&self, id: &str) -> Result<Filter> {
        unimplemented!("This method was not implemented");
    }
    /// PUT /api/v1/filters/:id
    fn update_filter(&self, id: &str, request: &mut AddFilterRequest) -> Result<Filter> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/filters/:id
    fn delete_filter(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/suggestions
    fn get_follow_suggestions(&self) -> Result<Vec<Account>> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/suggestions/:account_id
    fn delete_from_suggestions(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/endorsements
    fn get_endorsements(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/:id/pin
    fn endorse_user(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/:id/unpin
    fn unendorse_user(&self, id: &str) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// Shortcut for: `let me = client.verify_credentials(); client.followers()`
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// # use elefren::prelude::*;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// # let client = Mastodon::from(data);
    /// let follows_me = client.follows_me()?;
    /// #   Ok(())
    /// # }
    fn follows_me(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// Shortcut for
    /// `let me = client.verify_credentials(); client.following(&me.id)`
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use std::error::Error;
    /// # use elefren::prelude::*;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// # let client = Mastodon::from(data);
    /// let follows_me = client.followed_by_me()?;
    /// #   Ok(())
    /// # }
    fn followed_by_me(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }

    /// Returns events that are relevant to the authorized user, i.e. home
    /// timeline and notifications
    fn streaming_user(&self) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }

    /// Returns all public statuses
    fn streaming_public(&self) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }

    /// Returns all local statuses
    fn streaming_local(&self) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }

    /// Returns all public statuses for a particular hashtag
    fn streaming_public_hashtag(&self, hashtag: &str) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }

    /// Returns all local statuses for a particular hashtag
    fn streaming_local_hashtag(&self, hashtag: &str) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }

    /// Returns statuses for a list
    fn streaming_list(&self, list_id: &str) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }

    /// Returns all direct messages
    fn streaming_direct(&self) -> Result<Self::Stream> {
        unimplemented!("This method was not implemented");
    }
}

/// Trait that represents clients that can make unauthenticated calls to a
/// mastodon instance
#[allow(unused)]
pub trait MastodonUnauthenticated<H: HttpSend> {
    /// GET /api/v1/statuses/:id
    fn get_status(&self, id: &str) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/context
    fn get_context(&self, id: &str) -> Result<Context> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/card
    fn get_card(&self, id: &str) -> Result<Card> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/reblogged_by
    fn reblogged_by(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/favourited_by
    fn favourited_by(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
}
