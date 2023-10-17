use ethers_providers::{Middleware, Provider, ProviderError};

/// Provider provide by metamask
///
/// This provide close some unsupport rpc method in metamask
#[derive(Debug)]
pub struct MetamaskProvider {
    #[cfg(not(target_arch = "wasm32"))]
    provider: Provider<ethers_providers::Http>,

    #[cfg(target_arch = "wasm32")]
    provider: Provider<crate::MetamaskRpc>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Middleware for MetamaskProvider {
    type Error = ProviderError;

    type Provider = ethers_providers::Http;

    type Inner = Provider<ethers_providers::Http>;

    fn inner(&self) -> &Self::Inner {
        &self.provider
    }
}

#[cfg(target_arch = "wasm32")]
impl Middleware for MetamaskProvider {
    type Error = ProviderError;

    type Provider = crate::MetamaskRpc;

    type Inner = Provider<crate::MetamaskRpc>;

    fn inner(&self) -> &Self::Inner {
        &self.provider
    }
}
