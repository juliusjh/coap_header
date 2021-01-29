use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
pub enum MsgType {
    Confirmable,
    NonConfirmable,
    Acknowlegment,
    Reset,
}

#[derive(Debug)]
pub enum Method {
    Empty,
    Get,
    Post,
    Other(u8),
}

#[derive(Debug)]
pub enum Success {
    Created,
    Deleted,
    Other(u8),
}

#[derive(Debug)]
pub enum ClientError {
    BadRequest,
    Unauthorized,
    Other(u8),
}

#[derive(Debug)]
pub enum ServerError {
    Internal,
    NotImplemented,
    Other(u8),
}

#[derive(Debug)]
pub enum Signaling {
    Unassigend,
    CSM,
    Other(u8),
}

#[derive(Debug)]
pub enum RqCode {
    Method(Method),
    Success(Success),
    ClientError(ClientError),
    ServerError(ServerError),
    Signaling(Signaling),
}

#[derive(Debug)]
pub struct Header {
    ver: u8,
    msg_type: MsgType,
    token_length: u8,
    rq_type: RqCode,
    msg_id: u16,
}

impl Header {
    pub fn new(ver: u8, msg_type: MsgType, token_length: u8, rq_type: RqCode, msg_id: u16) -> Self {
        Self {
            ver: ver,
            msg_type: msg_type,
            token_length: token_length,
            rq_type: rq_type,
            msg_id: msg_id,
        }
    }

    pub fn from_bytes(data: [u8; 4]) -> Result<Self, &'static str> {
        let ver = (0xC0 & data[0]) >> 6;
        let msg_type_number = (0x30 & data[0]) >> 4;
        let msg_type = MsgType::try_from(msg_type_number)?;
        let token_length = 0x0F & data[0];
        let rq_code_0 = (data[1] >> 5) & 0x07;
        let rq_code_1 = (data[1] >> 0) & 0x1F;
        let rq_code = RqCode::try_from((rq_code_0, rq_code_1))?;
        let msg_id = ((data[2] as u16) << 8) | (data[3] as u16);

        let res = Self {
            ver: ver,
            msg_type: msg_type,
            token_length: token_length,
            rq_type: rq_code,
            msg_id: msg_id,
        };
        Ok(res)
    }

    /* TODO
    pub to_bytes(&self) -> [u8, 4] {

    }

    pub is_valid(&self) -> bool {

    }
    */
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Version: {}\nType: {:?}\nToken length: {}\nCode: {}\nMessage ID: {}",
            self.ver,
            self.msg_type,
            self.token_length,
            self.rq_type.to_string(),
            self.msg_id
        )
    }
}

impl std::fmt::Display for RqCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Method(v) => write!(f, "Method {:?}", v),
            Self::Success(v) => write!(f, "Success {:?}", v),
            Self::ClientError(v) => write!(f, "ClientError {:?}", v),
            Self::ServerError(v) => write!(f, "ServerError {:?}", v),
            Self::Signaling(v) => write!(f, "Signaling {:?}", v),
        }
    }
}

impl From<RqCode> for (u8, u8) {
    fn from(value: RqCode) -> (u8, u8) {
        match value {
            RqCode::Method(m) => (0, u8::from(m)),
            RqCode::Success(s) => (2, u8::from(s)),
            RqCode::ClientError(ce) => (4, u8::from(ce)),
            RqCode::ServerError(se) => (5, u8::from(se)),
            RqCode::Signaling(s) => (7, u8::from(s)),
        }
    }
}

impl TryFrom<(u8, u8)> for RqCode {
    type Error = &'static str;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::Method(Method::try_from(value.1)?)),
            2 => Ok(Self::Success(Success::try_from(value.1)?)),
            4 => Ok(Self::ClientError(ClientError::try_from(value.1)?)),
            5 => Ok(Self::ServerError(ServerError::try_from(value.1)?)),
            7 => Ok(Self::Signaling(Signaling::try_from(value.1)?)),
            _ => Err("Only the values 0, 2, 5, 7 are valid RqCodes"),
        }
    }
}

impl From<Signaling> for u8 {
    fn from(value: Signaling) -> u8 {
        match value {
            Signaling::Unassigend => 0,
            Signaling::CSM => 1,
            Signaling::Other(v) => v,
        }
    }
}

impl TryFrom<u8> for Signaling {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unassigend),
            1 => Ok(Self::CSM),
            _ => {
                if value > 5 {
                    Err("Only values between 0 and 5 are valid Signaling codes.")
                } else {
                    Ok(Self::Other(value))
                }
            }
        }
    }
}

impl From<ServerError> for u8 {
    fn from(value: ServerError) -> u8 {
        match value {
            ServerError::Internal => 0,
            ServerError::NotImplemented => 1,
            ServerError::Other(v) => v,
        }
    }
}

impl TryFrom<u8> for ServerError {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Internal),
            1 => Ok(Self::NotImplemented),
            _ => {
                if value > 5 {
                    Err("Only values between 0 and 5 are valid .")
                } else {
                    Ok(Self::Other(value))
                }
            }
        }
    }
}

impl From<ClientError> for u8 {
    fn from(value: ClientError) -> u8 {
        match value {
            ClientError::BadRequest => 0,
            ClientError::Unauthorized => 1,
            ClientError::Other(v) => v,
        }
    }
}

impl TryFrom<u8> for ClientError {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::BadRequest),
            1 => Ok(Self::Unauthorized),
            _ => {
                if value > 9 && value != 12 && value != 13 && value != 15 {
                    Err("Only values between 0 and 9 and the values 12, 13, 15 are valid client error.")
                } else {
                    Ok(Self::Other(value))
                }
            }
        }
    }
}

impl From<Success> for u8 {
    fn from(value: Success) -> u8 {
        match value {
            Success::Created => 0,
            Success::Deleted => 1,
            Success::Other(v) => v,
        }
    }
}

impl TryFrom<u8> for Success {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Created),
            1 => Ok(Self::Deleted),
            _ => {
                if value > 5 && value != 31 {
                    Err("Only values between 0 and 5 and the value 31 are valid success codes.")
                } else {
                    Ok(Self::Other(value))
                }
            }
        }
    }
}

impl From<MsgType> for u8 {
    fn from(value: MsgType) -> u8 {
        match value {
            MsgType::Confirmable => 0,
            MsgType::NonConfirmable => 1,
            MsgType::Acknowlegment => 2,
            MsgType::Reset => 3,
        }
    }
}

impl TryFrom<u8> for MsgType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Confirmable),
            1 => Ok(Self::NonConfirmable),
            2 => Ok(Self::Acknowlegment),
            3 => Ok(Self::Reset),
            _ => Err("Only values between 0 and 4 are valid message types."),
        }
    }
}

impl From<Method> for u8 {
    fn from(value: Method) -> u8 {
        match value {
            Method::Empty => 0,
            Method::Get => 1,
            Method::Post => 2,
            Method::Other(v) => v,
        }
    }
}

impl TryFrom<u8> for Method {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Get),
            2 => Ok(Self::Post),
            _ => {
                if value > 7 {
                    Err("Only values between 0 and 7 are valid methods.")
                } else {
                    Ok(Self::Other(value))
                }
            }
        }
    }
}
