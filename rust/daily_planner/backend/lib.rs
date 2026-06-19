use candid::{CandidType, Decode, Deserialize, Encode, Nat};
use ic_cdk::api::canister_self;
use ic_cdk_management_canister::{
    http_request, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs, TransformContext,
    TransformFunc,
};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::{Bound, Storable};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::borrow::Cow;
use std::cell::RefCell;

// Create data structures in stable memory so that the data persist across canister upgrades.
type VMem = VirtualMemory<DefaultMemoryImpl>;
type DayDataEntries = StableBTreeMap<String, DayDataEntry, VMem>;
const DAY_DATA_ENTRIES_MEMORY_ID: MemoryId = MemoryId::new(1);
thread_local! {
    /// Static memory manager to manage the memory available for stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize canister state.
    static STATE: RefCell<State> = MEMORY_MANAGER.with(|cell| {
        let mm = cell.borrow();
        let day_data_entries = DayDataEntries::init(mm.get(DAY_DATA_ENTRIES_MEMORY_ID));
        RefCell::new(State {
            day_data_entries,
        })
    });
}

struct State {
    day_data_entries: DayDataEntries,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct Note {
    id: Nat,
    content: String,
    is_completed: bool,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct OnThisDay {
    title: String,
    year: String,
    wiki_link: String,
}

#[derive(Debug, Default, Clone, CandidType, Deserialize)]
struct DayDataEntry {
    notes: Vec<Note>,
    on_this_day: Option<OnThisDay>,
}

// Data needs to be serialized to be stored in stable memory.
// Uses Candid for serialization - this is not efficient, but simple.
impl Storable for DayDataEntry {
    const BOUND: Bound = Bound::Unbounded;
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, DayDataEntry).expect("failed to deserialize DayDataEntry")
    }
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Encode!(&self)
            .expect("failed to serialize DayDataEntry")
            .into()
    }
}

// Query function to get data for a day.
#[ic_cdk::query]
fn get_day_data(date: String) -> Option<DayDataEntry> {
    STATE.with(|s| s.borrow().day_data_entries.get(&date))
}

// Query function to get data for a full month.
#[ic_cdk::query]
fn get_month_data(year: Nat, month: Nat) -> Vec<(String, DayDataEntry)> {
    // `Nat`s display with '_' as thousand separators.
    let month_prefix = format!("{}-{}-", year, month).replace('_', "");
    STATE.with(|s| {
        s.borrow()
            .day_data_entries
            .iter()
            .filter(|(k, _)| k.starts_with(&month_prefix))
            .collect()
    })
}

// Update function to add a new note.
#[ic_cdk::update]
fn add_note(date: String, content: String) -> Result<String, String> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let mut day_data = state.day_data_entries.get(&date).unwrap_or_default();
        let new_note = Note {
            id: day_data.notes.len().into(),
            content,
            is_completed: false,
        };
        day_data.notes.push(new_note);
        state.day_data_entries.insert(date.clone(), day_data);
        Ok(format!("Added note for date: {date}"))
        // Currently returns no errors, but could be extended to e.g. reject creation of notes in the past.
    })
}

// Update function to complete a note.
#[ic_cdk::update]
fn complete_note(date: String, note_id: Nat) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        // Does nothing if the note does not exist.
        if let Some(mut day_data) = state.day_data_entries.get(&date) {
            day_data.notes = day_data
                .notes
                .into_iter()
                .map(|note| {
                    if note.id == note_id {
                        Note {
                            is_completed: true,
                            ..note
                        }
                    } else {
                        note
                    }
                })
                .collect();
            state.day_data_entries.insert(date, day_data);
        }
    })
}

// Update function to fetch and store "On This Day" facts via HTTPS outcall.
#[ic_cdk::update]
async fn fetch_and_store_on_this_day(date: String) -> Result<String, String> {
    let already_fetched = STATE.with(|s| {
        s.borrow()
            .day_data_entries
            .get(&date)
            .and_then(|day_data| day_data.on_this_day)
            .is_some()
    });
    if already_fetched {
        return Err(format!("data already stored for date: {date}"));
    }

    // Generate URL. Target must support IPv6.
    let date_parts: Vec<&str> = date.split('-').collect();
    let month = date_parts[1].trim_start_matches('0');
    let day = date_parts[2].trim_start_matches('0');
    let url = format!("https://byabbe.se/on-this-day/{month}/{day}/events.json");

    // TransformContext is used to specify how the HTTP response is processed before consensus tries to agree on a response.
    // This is useful to e.g. filter out timestamps/sessionIDs out of headers that will be different across the responses the different replicas receive.
    // If the data (including status, headers and body) they receive does not match across the nodes, the canister will reject the response!
    // You can read more about it here: https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/https-outcalls/https-outcalls-how-to-use.
    let request = HttpRequestArgs {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None, // Can be set to limit cost. Our response has no predictable size, so we set no limit.
        headers: vec![],
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
        // Replicated mode: all subnet nodes make the request independently,
        // providing strong integrity guarantees via consensus.
        is_replicated: Some(true),
    };

    // Perform HTTPS outcall. Cycles are automatically calculated and attached.
    // Unused cycles are returned.
    // See https outcall cost calculator: https://7joko-hiaaa-aaaal-ajz7a-cai.icp0.io.
    let quote = match http_request(&request).await {
        Ok(response) => {
            let body_string =
                String::from_utf8(response.body).expect("Response is not UTF-8 encoded.");
            let Some(otd) = http_response_to_on_this_day(&body_string) else {
                return Err(format!("Failed get event for data {date}"));
            };
            otd
        }
        Err(err) => {
            return Err(format!("http_request resulted in an error: {err:?}"));
        }
    };

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let mut day_data = state.day_data_entries.get(&date).unwrap_or_default();
        day_data.on_this_day = Some(quote);
        state.day_data_entries.insert(date.clone(), day_data);
    });

    Ok(format!(
        "data successfully obtained and stored for date: {date}"
    ))
}

// Query function to turn the raw HTTP responses into responses that nodes can run consensus on.
#[ic_cdk::query(hidden = true)]
fn transform(raw: TransformArgs) -> HttpRequestResult {
    HttpRequestResult {
        headers: vec![], // We filter out the headers, as they don't match across nodes.
        ..raw.response
    }
}

fn http_response_to_on_this_day(http: &str) -> Option<OnThisDay> {
    let json: serde_json::Value = serde_json::from_str(http).ok()?;
    let title = json["events"][0]["description"].as_str()?;
    let year = json["events"][0]["year"].as_str()?;
    let wiki_link = json["events"][0]["wikipedia"][0]["wikipedia"].as_str()?;
    Some(OnThisDay {
        title: title.to_string(),
        year: year.to_string(),
        wiki_link: wiki_link.to_string(),
    })
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
