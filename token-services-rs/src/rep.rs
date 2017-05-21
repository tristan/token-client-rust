use eth::{SecretKey};
use super::{signed_request,Method};

pub struct ReputationService<'a> {
    base_url: &'static str,
    signing_key: &'a SecretKey
}

impl<'a> ReputationService<'a> {
    pub fn new(base_url: &'static str, signing_key: &'a SecretKey) -> ReputationService<'a> {
        ReputationService {
            base_url: base_url,
            signing_key: signing_key
        }
    }

    pub fn submit_review(&self, reviewee: &str, rating: f32, review: &str) -> Result<(), (String)> {
        let result = signed_request(self.signing_key,
                                    Method::POST,
                                    self.base_url,
                                    "/v1/review/submit",
                                    None,
                                    Some(&object!{
                                        "reviewee" => reviewee,
                                        "rating" => rating,
                                        "review" => review
                                    }));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
