use endpoint_libs::libs::error_code::ErrorCode;
use endpoint_libs::libs::ws::*;
use endpoint_libs::libs::types::*;
use endpoint_libs::model;
use num_derive::FromPrimitive;
use rust_decimal::Decimal;
use serde::*;
use strum_macros::{Display, EnumString};
use tokio_postgres::types::*;
use endpoint_libs::libs::types::WithBlockchainTransactionHash;
use endpoint_libs::libs::types::WithBlockchainAddress;

#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_block_chain")]
pub enum EnumBlockChain {
  ///
  #[postgres(name = "EthereumMainnet")]
  EthereumMainnet = 0,
  ///
  #[postgres(name = "EthereumGoerli")]
  EthereumGoerli = 1,
  ///
  #[postgres(name = "BscMainnet")]
  BscMainnet = 2,
  ///
  #[postgres(name = "BscTestnet")]
  BscTestnet = 3,
  ///
  #[postgres(name = "LocalNet")]
  LocalNet = 4,
  ///
  #[postgres(name = "EthereumSepolia")]
  EthereumSepolia = 5,
}
#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_dex")]
pub enum EnumDex {
  ///
  #[postgres(name = "UniSwap")]
  UniSwap = 0,
  ///
  #[postgres(name = "PancakeSwap")]
  PancakeSwap = 1,
  ///
  #[postgres(name = "SushiSwap")]
  SushiSwap = 2,
}
#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_dex_path_format")]
pub enum EnumDexPathFormat {
  ///
  #[postgres(name = "Json")]
  Json = 0,
  ///
  #[postgres(name = "TransactionData")]
  TransactionData = 1,
  ///
  #[postgres(name = "TransactionHash")]
  TransactionHash = 2,
}
#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_role")]
pub enum EnumRole {
  ///
  #[postgres(name = "guest")]
  Guest = 0,
  ///
  #[postgres(name = "user")]
  User = 1,
  ///
  #[postgres(name = "admin")]
  Admin = 3,
  ///
  #[postgres(name = "developer")]
  Developer = 4,
}
#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_service")]
pub enum EnumService {
  ///
  #[postgres(name = "auth")]
  Auth = 1,
  ///
  #[postgres(name = "user")]
  User = 2,
}
#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_Endpoint")]
pub enum EnumEndpoint {
  ///
  #[postgres(name = "Signup")]
  Signup = 10010,
  ///
  #[postgres(name = "Login")]
  Login = 10020,
  ///
  #[postgres(name = "Authorize")]
  Authorize = 10030,
  ///
  #[postgres(name = "Logout")]
  Logout = 10040,
  ///
  #[postgres(name = "UserListEvents")]
  UserListEvents = 20010,
  ///
  #[postgres(name = "UserListSignals")]
  UserListSignals = 20020,
}

impl EnumEndpoint {
  pub fn schema(&self) -> model::EndpointSchema {
      let schema = match self {
          Self::Signup => SignupRequest::SCHEMA,
          Self::Login => LoginRequest::SCHEMA,
          Self::Authorize => AuthorizeRequest::SCHEMA,
          Self::Logout => LogoutRequest::SCHEMA,
          Self::UserListEvents => UserListEventsRequest::SCHEMA,
          Self::UserListSignals => UserListSignalsRequest::SCHEMA,
      };
      serde_json::from_str(schema).unwrap()
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBadRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalServerError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotImplemented {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDatabaseError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidService {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserForbidden {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreeTos {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreePrivacyPolicy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNoAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserInvalidAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorTokenNotTop25 {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorImmutableStrategy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserWhitelistedWalletNotSameNetworkAsStrategy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDuplicateRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidExpression {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidEnumLevel {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidArgument {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidState {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidSeq {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidMethod {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProtocolViolation {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMalformedRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnknownUser {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBlockedUser {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidPassword {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorTemporarilyUnavailable {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnexpectedException {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBackPressureIncreased {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidPublicId {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRange {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBankAccountAlreadyExists {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInsufficientFunds {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogicalError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRestrictedUserPrivileges {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorIdenticalReplacement {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRecoveryQuestions {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRole {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorWrongRecoveryAnswers {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessageNotDelivered {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNoReply {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNullAttribute {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorConsentMissing {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorActiveSubscriptionRequired {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUsernameAlreadyRegistered {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRecoveryQuestionsNotSet {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMustSubmitAllRecoveryQuestions {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRecoveryToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRoutingError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnauthorizedMessage {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorAuthError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalError {}
#[derive(
  Debug,
  Clone,
  Copy,
  ToSql,
  FromSql,
  Serialize,
  Deserialize,
  FromPrimitive,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  EnumString,
  Display,
  Hash,
)]
#[postgres(name = "enum_ErrorCode")]
pub enum EnumErrorCode {
  /// Custom Bad Request
  #[postgres(name = "BadRequest")]
  BadRequest = 100400,
  /// Custom Internal Server Error
  #[postgres(name = "InternalServerError")]
  InternalServerError = 100500,
  /// Custom Method not implemented
  #[postgres(name = "NotImplemented")]
  NotImplemented = 100501,
  /// Custom NotFoundResource
  #[postgres(name = "NotFound")]
  NotFound = 100404,
  /// Custom Database error
  #[postgres(name = "DatabaseError")]
  DatabaseError = 100601,
  /// Custom Invalid Service
  #[postgres(name = "InvalidService")]
  InvalidService = 100602,
  /// Custom Insufficient role for user
  #[postgres(name = "UserForbidden")]
  UserForbidden = 101403,
  /// Custom User not found
  #[postgres(name = "UserNotFound")]
  UserNotFound = 101404,
  /// Custom Must agree to the terms of service
  #[postgres(name = "UserMustAgreeTos")]
  UserMustAgreeTos = 101601,
  /// Custom Must agree to the privacy policy
  #[postgres(name = "UserMustAgreePrivacyPolicy")]
  UserMustAgreePrivacyPolicy = 101602,
  /// Custom No auth token
  #[postgres(name = "UserNoAuthToken")]
  UserNoAuthToken = 101604,
  /// Custom token invalid
  #[postgres(name = "UserInvalidAuthToken")]
  UserInvalidAuthToken = 101605,
  /// Audit Token is not top 25
  #[postgres(name = "TokenNotTop25")]
  TokenNotTop25 = 102602,
  /// Audit Strategy is immutable
  #[postgres(name = "ImmutableStrategy")]
  ImmutableStrategy = 102603,
  /// Audit User whitelisted wallet not same network as strategy
  #[postgres(name = "UserWhitelistedWalletNotSameNetworkAsStrategy")]
  UserWhitelistedWalletNotSameNetworkAsStrategy = 102604,
  /// Custom Duplicate request
  #[postgres(name = "DuplicateRequest")]
  DuplicateRequest = 103001,
  /// Custom Invalid expression
  #[postgres(name = "InvalidExpression")]
  InvalidExpression = 104000,
  /// SQL 22P02 InvalidEnumLevel
  #[postgres(name = "InvalidEnumLevel")]
  InvalidEnumLevel = 3484946,
  /// SQL R0000 Error
  #[postgres(name = "Error")]
  Error = 4349632,
  /// SQL R0001 InvalidArgument
  #[postgres(name = "InvalidArgument")]
  InvalidArgument = 45349633,
  /// SQL R0002 InvalidState
  #[postgres(name = "InvalidState")]
  InvalidState = 45349634,
  /// SQL R0003 InvalidSeq
  #[postgres(name = "InvalidSeq")]
  InvalidSeq = 45349635,
  /// SQL R0004 InvalidMethod
  #[postgres(name = "InvalidMethod")]
  InvalidMethod = 45349636,
  /// SQL R0005 ProtocolViolation
  #[postgres(name = "ProtocolViolation")]
  ProtocolViolation = 45349637,
  /// SQL R0006 MalformedRequest
  #[postgres(name = "MalformedRequest")]
  MalformedRequest = 45349638,
  /// SQL R0007 UnknownUser
  #[postgres(name = "UnknownUser")]
  UnknownUser = 45349639,
  /// SQL R0008 BlockedUser
  #[postgres(name = "BlockedUser")]
  BlockedUser = 45349640,
  /// SQL R0009 InvalidPassword
  #[postgres(name = "InvalidPassword")]
  InvalidPassword = 45349641,
  /// SQL R000A InvalidToken
  #[postgres(name = "InvalidToken")]
  InvalidToken = 45349642,
  /// SQL R000B TemporarilyUnavailable
  #[postgres(name = "TemporarilyUnavailable")]
  TemporarilyUnavailable = 45349643,
  /// SQL R000C UnexpectedException
  #[postgres(name = "UnexpectedException")]
  UnexpectedException = 45349644,
  /// SQL R000D BackPressureIncreased
  #[postgres(name = "BackPressureIncreased")]
  BackPressureIncreased = 45349645,
  /// SQL R000E InvalidPublicId
  #[postgres(name = "InvalidPublicId")]
  InvalidPublicId = 45349646,
  /// SQL R000F InvalidRange
  #[postgres(name = "InvalidRange")]
  InvalidRange = 45349647,
  /// SQL R000G BankAccountAlreadyExists
  #[postgres(name = "BankAccountAlreadyExists")]
  BankAccountAlreadyExists = 45349648,
  /// SQL R000H InsufficientFunds
  #[postgres(name = "InsufficientFunds")]
  InsufficientFunds = 45349649,
  /// SQL R000M LogicalError
  #[postgres(name = "LogicalError")]
  LogicalError = 45349654,
  /// SQL R000N RestrictedUserPrivileges
  #[postgres(name = "RestrictedUserPrivileges")]
  RestrictedUserPrivileges = 45349655,
  /// SQL R000O IdenticalReplacement
  #[postgres(name = "IdenticalReplacement")]
  IdenticalReplacement = 45349656,
  /// SQL R000R InvalidRecoveryQuestions
  #[postgres(name = "InvalidRecoveryQuestions")]
  InvalidRecoveryQuestions = 45349659,
  /// SQL R000S InvalidRole
  #[postgres(name = "InvalidRole")]
  InvalidRole = 45349660,
  /// SQL R000T WrongRecoveryAnswers
  #[postgres(name = "WrongRecoveryAnswers")]
  WrongRecoveryAnswers = 45349661,
  /// SQL R000U MessageNotDelivered
  #[postgres(name = "MessageNotDelivered")]
  MessageNotDelivered = 45349662,
  /// SQL R000V NoReply
  #[postgres(name = "NoReply")]
  NoReply = 45349663,
  /// SQL R000W NullAttribute
  #[postgres(name = "NullAttribute")]
  NullAttribute = 45349664,
  /// SQL R000X ConsentMissing
  #[postgres(name = "ConsentMissing")]
  ConsentMissing = 45349665,
  /// SQL R000Y ActiveSubscriptionRequired
  #[postgres(name = "ActiveSubscriptionRequired")]
  ActiveSubscriptionRequired = 45349666,
  /// SQL R000Z UsernameAlreadyRegistered
  #[postgres(name = "UsernameAlreadyRegistered")]
  UsernameAlreadyRegistered = 45349667,
  /// SQL R0010 RecoveryQuestionsNotSet
  #[postgres(name = "RecoveryQuestionsNotSet")]
  RecoveryQuestionsNotSet = 45349668,
  /// SQL R0011 MustSubmitAllRecoveryQuestions
  #[postgres(name = "MustSubmitAllRecoveryQuestions")]
  MustSubmitAllRecoveryQuestions = 45349669,
  /// SQL R0012 InvalidRecoveryToken
  #[postgres(name = "InvalidRecoveryToken")]
  InvalidRecoveryToken = 45349670,
  /// SQL R0018 RoutingError
  #[postgres(name = "RoutingError")]
  RoutingError = 45349676,
  /// SQL R0019 UnauthorizedMessage
  #[postgres(name = "UnauthorizedMessage")]
  UnauthorizedMessage = 45349677,
  /// SQL R001B AuthError
  #[postgres(name = "AuthError")]
  AuthError = 45349679,
  /// SQL R001G InternalError
  #[postgres(name = "InternalError")]
  InternalError = 45349684,
}

impl From<EnumErrorCode> for ErrorCode {
  fn from(e: EnumErrorCode) -> Self {
      ErrorCode::new(e as _)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
  pub username: String,
  pub token: uuid::Uuid,
  pub service: EnumService,
  pub device_id: String,
  pub device_os: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
  pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
  pub username: String,
  pub password: String,
  pub service: EnumService,
  pub device_id: String,
  pub device_os: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
  pub username: String,
  pub display_name: String,
  #[serde(default)]
  pub avatar: Option<String>,
  pub role: EnumRole,
  pub user_id: i64,
  pub user_token: uuid::Uuid,
  pub admin_token: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogoutResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
  pub username: String,
  pub password: String,
  pub email: String,
  pub phone: String,
  pub agreed_tos: bool,
  pub agreed_privacy: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupResponse {
  pub username: String,
  pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListEventsRequest {
  #[serde(default)]
  pub kind: Option<i32>,
  #[serde(default)]
  pub limit: Option<i32>,
  #[serde(default)]
  pub offset: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListEventsResponse {
  pub total: i64,
  pub events: Vec<UserListEventsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListEventsRow {
  pub event_id: i64,
  pub kind: i32,
  pub severity: i32,
  pub chain_id: i32,
  pub block_id: String,
  pub block_time: i64,
  #[serde(with = "WithBlockchainTransactionHash")]
  pub transaction_hash: H256,
  pub wallet_addresses: Vec<Address>,
  pub contract_addresses: Vec<Address>,
  pub detail: serde_json::Value,
  pub signals: Vec<i32>,
  pub signal_kinds: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListSignalsRequest {
  #[serde(default)]
  pub kind: Option<i32>,
  #[serde(default)]
  pub signal_id: Option<i64>,
  #[serde(default)]
  pub limit: Option<i32>,
  #[serde(default)]
  pub offset: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListSignalsResponse {
  pub total: i64,
  pub signals: Vec<UserListSignalsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListSignalsRow {
  pub signal_id: i64,
  pub kind: i32,
  pub chain_id: i32,
  pub block_id: String,
  pub block_time: i64,
  #[serde(with = "WithBlockchainTransactionHash")]
  pub transaction_hash: H256,
  #[serde(with = "WithBlockchainAddress")]
  pub from_address: Address,
  #[serde(with = "WithBlockchainAddress")]
  pub contract_address: Address,
  pub detail: serde_json::Value,
  #[serde(default)]
  pub value: Option<f64>,
  #[serde(default)]
  pub token: Option<String>,
  #[serde(default)]
  pub value_usd: Option<f64>,
}
impl WsRequest for SignupRequest {
  type Response = SignupResponse;
  const METHOD_ID: u32 = 10010;
  const SCHEMA: &'static str = r#"{
"name": "Signup",
"code": 10010,
"parameters": [
  {
    "name": "username",
    "ty": "String"
  },
  {
    "name": "password",
    "ty": "String"
  },
  {
    "name": "email",
    "ty": "String"
  },
  {
    "name": "phone",
    "ty": "String"
  },
  {
    "name": "agreed_tos",
    "ty": "Boolean"
  },
  {
    "name": "agreed_privacy",
    "ty": "Boolean"
  }
],
"returns": [
  {
    "name": "username",
    "ty": "String"
  },
  {
    "name": "user_id",
    "ty": "BigInt"
  }
],
"stream_response": null,
"description": "",
"json_schema": null
}"#;
}
impl WsResponse for SignupResponse {
  type Request = SignupRequest;
}

impl WsRequest for LoginRequest {
  type Response = LoginResponse;
  const METHOD_ID: u32 = 10020;
  const SCHEMA: &'static str = r#"{
"name": "Login",
"code": 10020,
"parameters": [
  {
    "name": "username",
    "ty": "String"
  },
  {
    "name": "password",
    "ty": "String"
  },
  {
    "name": "service",
    "ty": {
      "EnumRef": "service"
    }
  },
  {
    "name": "device_id",
    "ty": "String"
  },
  {
    "name": "device_os",
    "ty": "String"
  }
],
"returns": [
  {
    "name": "username",
    "ty": "String"
  },
  {
    "name": "display_name",
    "ty": "String"
  },
  {
    "name": "avatar",
    "ty": {
      "Optional": "String"
    }
  },
  {
    "name": "role",
    "ty": {
      "EnumRef": "role"
    }
  },
  {
    "name": "user_id",
    "ty": "BigInt"
  },
  {
    "name": "user_token",
    "ty": "UUID"
  },
  {
    "name": "admin_token",
    "ty": "UUID"
  }
],
"stream_response": null,
"description": "",
"json_schema": null
}"#;
}
impl WsResponse for LoginResponse {
  type Request = LoginRequest;
}

impl WsRequest for AuthorizeRequest {
  type Response = AuthorizeResponse;
  const METHOD_ID: u32 = 10030;
  const SCHEMA: &'static str = r#"{
"name": "Authorize",
"code": 10030,
"parameters": [
  {
    "name": "username",
    "ty": "String"
  },
  {
    "name": "token",
    "ty": "UUID"
  },
  {
    "name": "service",
    "ty": {
      "EnumRef": "service"
    }
  },
  {
    "name": "device_id",
    "ty": "String"
  },
  {
    "name": "device_os",
    "ty": "String"
  }
],
"returns": [
  {
    "name": "success",
    "ty": "Boolean"
  }
],
"stream_response": null,
"description": "",
"json_schema": null
}"#;
}
impl WsResponse for AuthorizeResponse {
  type Request = AuthorizeRequest;
}

impl WsRequest for LogoutRequest {
  type Response = LogoutResponse;
  const METHOD_ID: u32 = 10040;
  const SCHEMA: &'static str = r#"{
"name": "Logout",
"code": 10040,
"parameters": [],
"returns": [],
"stream_response": null,
"description": "",
"json_schema": null
}"#;
}
impl WsResponse for LogoutResponse {
  type Request = LogoutRequest;
}

impl WsRequest for UserListEventsRequest {
  type Response = UserListEventsResponse;
  const METHOD_ID: u32 = 20010;
  const SCHEMA: &'static str = r#"{
"name": "UserListEvents",
"code": 20010,
"parameters": [
  {
    "name": "kind",
    "ty": {
      "Optional": "Int"
    }
  },
  {
    "name": "limit",
    "ty": {
      "Optional": "Int"
    }
  },
  {
    "name": "offset",
    "ty": {
      "Optional": "Int"
    }
  }
],
"returns": [
  {
    "name": "total",
    "ty": "BigInt"
  },
  {
    "name": "events",
    "ty": {
      "DataTable": {
        "name": "UserListEventsRow",
        "fields": [
          {
            "name": "event_id",
            "ty": "BigInt"
          },
          {
            "name": "kind",
            "ty": "Int"
          },
          {
            "name": "severity",
            "ty": "Int"
          },
          {
            "name": "chain_id",
            "ty": "Int"
          },
          {
            "name": "block_id",
            "ty": "String"
          },
          {
            "name": "block_time",
            "ty": "BigInt"
          },
          {
            "name": "transaction_hash",
            "ty": "BlockchainTransactionHash"
          },
          {
            "name": "wallet_addresses",
            "ty": {
              "Vec": "BlockchainAddress"
            }
          },
          {
            "name": "contract_addresses",
            "ty": {
              "Vec": "BlockchainAddress"
            }
          },
          {
            "name": "detail",
            "ty": "Object"
          },
          {
            "name": "signals",
            "ty": {
              "Vec": "Int"
            }
          },
          {
            "name": "signal_kinds",
            "ty": {
              "Vec": "Int"
            }
          }
        ]
      }
    }
  }
],
"stream_response": null,
"description": "",
"json_schema": null
}"#;
}
impl WsResponse for UserListEventsResponse {
  type Request = UserListEventsRequest;
}

impl WsRequest for UserListSignalsRequest {
  type Response = UserListSignalsResponse;
  const METHOD_ID: u32 = 20020;
  const SCHEMA: &'static str = r#"{
"name": "UserListSignals",
"code": 20020,
"parameters": [
  {
    "name": "kind",
    "ty": {
      "Optional": "Int"
    }
  },
  {
    "name": "signal_id",
    "ty": {
      "Optional": "BigInt"
    }
  },
  {
    "name": "limit",
    "ty": {
      "Optional": "Int"
    }
  },
  {
    "name": "offset",
    "ty": {
      "Optional": "Int"
    }
  }
],
"returns": [
  {
    "name": "total",
    "ty": "BigInt"
  },
  {
    "name": "signals",
    "ty": {
      "DataTable": {
        "name": "UserListSignalsRow",
        "fields": [
          {
            "name": "signal_id",
            "ty": "BigInt"
          },
          {
            "name": "kind",
            "ty": "Int"
          },
          {
            "name": "chain_id",
            "ty": "Int"
          },
          {
            "name": "block_id",
            "ty": "String"
          },
          {
            "name": "block_time",
            "ty": "BigInt"
          },
          {
            "name": "transaction_hash",
            "ty": "BlockchainTransactionHash"
          },
          {
            "name": "from_address",
            "ty": "BlockchainAddress"
          },
          {
            "name": "contract_address",
            "ty": "BlockchainAddress"
          },
          {
            "name": "detail",
            "ty": "Object"
          },
          {
            "name": "value",
            "ty": {
              "Optional": "Numeric"
            }
          },
          {
            "name": "token",
            "ty": {
              "Optional": "String"
            }
          },
          {
            "name": "value_usd",
            "ty": {
              "Optional": "Numeric"
            }
          }
        ]
      }
    }
  }
],
"stream_response": null,
"description": "",
"json_schema": null
}"#;
}
impl WsResponse for UserListSignalsResponse {
  type Request = UserListSignalsRequest;
}
