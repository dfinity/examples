use crate::agent::AbstractAgent;
use crate::{agent::Request, requests::CommitStateRequest};
use candid::{CandidType, Principal};
use std::fmt::Debug;
use std::{collections::VecDeque, error::Error, fmt::Display};

#[derive(Clone, Debug)]
pub struct MockError {
    pub message: String,
}

impl From<String> for MockError {
    fn from(message: String) -> Self {
        MockError { message }
    }
}

impl From<&str> for MockError {
    fn from(message: &str) -> Self {
        MockError {
            message: message.to_string(),
        }
    }
}

impl Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MockError {}

struct CallSpec {
    raw_request: Vec<u8>,
    raw_response: Vec<u8>,
    canister_id: Principal,
}

impl CallSpec {
    fn new<Req>(canister_id: Principal, request: Req, response: Req::Response) -> Result<Self, ()>
    where
        Req: Request,
    {
        let raw_request = request.payload().expect("Request is not encodable");
        let raw_response = candid::encode_one(response).expect("Response is not encodable");

        Ok(Self {
            raw_request,
            raw_response,
            canister_id,
        })
    }
}

pub struct MockAgent {
    expected_calls: VecDeque<CallSpec>,
    self_canister_id: Principal,
}

impl MockAgent {
    pub fn new(self_canister_id: Principal) -> Self {
        Self {
            self_canister_id,
            expected_calls: VecDeque::default(),
        }
    }

    pub fn add_call<Req>(
        mut self,
        canister_id: Principal,
        request: Req,
        response: Req::Response,
    ) -> Self
    where
        Req: Request,
    {
        let call = CallSpec::new(canister_id, request, response)
            .expect("Creating a new call specification failed");
        self.expected_calls.push_back(call);

        let commit_state = CallSpec::new(self.self_canister_id, CommitStateRequest {}, ())
            .expect("CommittState call creation failed");
        self.expected_calls.push_back(commit_state);
        self
    }

    pub fn finished_calls(&self) -> bool {
        self.expected_calls.is_empty()
    }
}

impl AbstractAgent for MockAgent {
    type Error = MockError;
    // Infallable !
    async fn call<R: Request + Debug + CandidType>(
        &mut self,
        canister_id: impl Into<Principal> + Send,
        request: R,
    ) -> Result<R::Response, Self::Error> {
        println!("started call...");
        let Ok(raw_request) = request.payload() else {
            panic!("Cannot encode the request");
        };

        let expected_call = self
            .expected_calls
            .pop_front()
            .expect("Consumed all expected requests");

        if raw_request != expected_call.raw_request {
            println!("request: {:#?}", request);
            println!("{:?}\n{:?}", raw_request, expected_call.raw_request);
            panic!("Request doesn't match");
        }
        let canister_id: Principal = canister_id.into();

        assert_eq!(
            canister_id, expected_call.canister_id,
            "observed {canister_id}, expected {}",
            expected_call.canister_id
        );

        let reply = candid::decode_one::<R::Response>(&expected_call.raw_response)
            .expect("Unable to decode the response");

        println!("successfully called canister ID: {}", canister_id);
        return Ok(reply);
    }
}
