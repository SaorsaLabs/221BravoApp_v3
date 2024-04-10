use candid::{Principal, CandidType};
use serde::{Serialize, Deserialize};
use types::MessageId;
use crate::core::utils::{canister_call, log};
use super::constants::BOT_NAME;

// TYPES FOR BOT MESSAGE
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub bot_name: String,
    #[serde(default)]
    pub bot_display_name: Option<String>,
    pub messages: Vec<BotMessage>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct BotMessage {
    pub content: MessageContentInitial,
    #[serde(default)]
    pub message_id: Option<MessageId>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum MessageContentInitial {
    Text(TextContent)
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TextContent {
    pub text: String,
}

impl From<String> for TextContent {
    fn from(value: String) -> Self {
        TextContent { text: value }
    }
}


#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    Blocked,
    ContentValidationError(ContentValidationError),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ContentValidationError {
    Empty,
    TextTooLong(u32),
    InvalidPoll(InvalidPollReason),
    TransferCannotBeZero,
    InvalidTypeForForwarding,
    PrizeEndDateInThePast,
    UnauthorizedToSendProposalMessages,
    Unauthorized,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum InvalidPollReason {
    TooFewOptions(u32),
    TooManyOptions(u32),
    OptionTooLong(u32),
    DuplicateOptions,
    EndDateInThePast,
    PollsNotValidForDirectChats,
}

// Send Text Message
pub async fn send__text_message(user_canister: String, message: String) -> Result<String, String> {

    let mut msgs: Vec<BotMessage> = Vec::new();
    let tmess = BotMessage{
        content: MessageContentInitial::Text(TextContent { text: message }),
        message_id: None,
    };
    msgs.push(tmess);
    let args = Args {
        bot_name: String::from(BOT_NAME),
        bot_display_name: None,
        messages: msgs,
    };

    let call: Result<(Response,), (ic_cdk::api::call::RejectionCode, String)>  = 
    canister_call(&user_canister, "c2c_handle_bot_messages", args, None).await;

    match call {
        Ok(v) => {
            Ok(format!("Call Success :: {:?}", v.0))
        },
        Err(e) => {
            Err(format!("Call Error :: {:?}, {}", e.0, e.1))
        }
    }
}