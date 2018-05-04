use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  leases_api: Box<::apis::LeasesApi>,
  pki_backend_api: Box<::apis::PkiBackendApi>,
  token_backend_api: Box<::apis::TokenBackendApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      leases_api: Box::new(::apis::LeasesApiClient::new(rc.clone())),
      pki_backend_api: Box::new(::apis::PkiBackendApiClient::new(rc.clone())),
      token_backend_api: Box::new(::apis::TokenBackendApiClient::new(rc.clone())),
    }
  }

  pub fn leases_api(&self) -> &::apis::LeasesApi{
    self.leases_api.as_ref()
  }

  pub fn pki_backend_api(&self) -> &::apis::PkiBackendApi{
    self.pki_backend_api.as_ref()
  }

  pub fn token_backend_api(&self) -> &::apis::TokenBackendApi{
    self.token_backend_api.as_ref()
  }


}
