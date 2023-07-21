use crate::*;

impl<Endpoint> std::fmt::Debug for Connect<Endpoint>
where
    Endpoint: xwebtransport_core::EndpointConnect,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connect::Connect(inner) => f.debug_tuple("Connect::Connect").field(inner).finish(),
            Connect::Connecting(inner) => {
                f.debug_tuple("Connect::Connecting").field(inner).finish()
            }
        }
    }
}

impl<Endpoint> std::fmt::Debug for Accept<Endpoint>
where
    Endpoint: xwebtransport_core::EndpointAccept,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accept::Accept(inner) => f.debug_tuple("Accept::Accept").field(inner).finish(),
            Accept::Connecting(inner) => f.debug_tuple("Accept::Connecting").field(inner).finish(),
        }
    }
}

impl<Connect> std::fmt::Debug for OpenBi<Connect>
where
    Connect: xwebtransport_core::OpenBiStream,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenBi::Open(inner) => f.debug_tuple("OpenBi::Open").field(inner).finish(),
            OpenBi::Opening(inner) => f.debug_tuple("OpenBi::Opening").field(inner).finish(),
        }
    }
}

impl<Connect> std::fmt::Debug for OpenUni<Connect>
where
    Connect: xwebtransport_core::OpenUniStream,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenUni::Open(inner) => f.debug_tuple("OpenUni::Open").field(inner).finish(),
            OpenUni::Opening(inner) => f.debug_tuple("OpenUni::Opening").field(inner).finish(),
        }
    }
}
