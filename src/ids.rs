use serde::{Serialize, Deserialize};

pub trait Zbase32Id: Sized {
    const ZBASE32_ALPHABET: [char; 32] = [
        'y', 'b', 'n', 'd', 'r', 'f', 'g', '8', 'e', 'j', 'k', 'm', 'c', 'p', 'q', 'x', 'o', 't',
        '1', 'u', 'w', 'i', 's', 'z', 'a', '3', '4', '5', 'h', '7', '6', '9',
    ];

    fn new() -> Self;

    fn validate_str(value: &str) -> Result<(), Zbase32IdError> {
        if value.len() != 8 {
            return Err(Zbase32IdError::InvalidLength);
        }

        if value.chars().any(|c| !Self::ZBASE32_ALPHABET.contains(&c)) {
            return Err(Zbase32IdError::InvalidCharacters);
        }

        Ok(())
    }

    fn try_from_str(value: &str) -> Result<Self, Zbase32IdError>;
}

#[derive(Debug, thiserror::Error, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Zbase32IdError {
    #[error("Invalid Zbase32 ID: length must be 8 characters")]
    InvalidLength,
    #[error("Invalid Zbase32 ID: contains invalid characters")]
    InvalidCharacters,
    #[error("Invalid Zbase32 ID: empty string")]
    Empty,
}

impl std::str::FromStr for Zbase32IdError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Invalid Zbase32 ID: length must be 8 characters" => Ok(Zbase32IdError::InvalidLength),
            "Invalid Zbase32 ID: contains invalid characters" => {
                Ok(Zbase32IdError::InvalidCharacters)
            }
            "Invalid Zbase32 ID: empty string" => Ok(Zbase32IdError::Empty),
            _ => Err(format!("Unknown Zbase32 ID error: {}", s)),
        }
    }
}

//#[cfg(feature = "ssr")]
//use crate::schema::{atendimentos, documentos, evolucoes, pacientes, planos_atendimentos};

macro_rules! declare_zbase32_id {
    ($id_name:ident) => {
        #[derive(
            Serialize,
            Deserialize,
            Hash,      // required by diesel
            PartialEq, // required by diesel
            Eq,        // required by diesel
            Clone,
            Copy,
        )]
        // Tell serde to deserialize data into a String and then try to convert it, using our impl traits with checks
        // https://github.com/serde-rs/serde/issues/939#issuecomment-939514114
        #[serde(try_from = "String", into = "String")]
        pub struct $id_name([u8; 8]);

        impl std::fmt::Display for $id_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Convert the [u8; 8] to a str and write it
                write!(
                    f,
                    "{}",
                    std::str::from_utf8(&self.0).unwrap_or("Invalid UTF-8")
                )
            }
        }

        // required by diesel
        impl std::fmt::Debug for $id_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($id_name))
                    .field(&std::str::from_utf8(&self.0).unwrap_or("Invalid UTF-8"))
                    .finish()
            }
        }

        impl $id_name {
            // Private method to create a new instance without validation
            fn from_bytes(bytes: [u8; 8]) -> Self {
                Self(bytes)
            }
        }

        impl Zbase32Id for $id_name {
            /// Generates a 8 char unique id
            /// with zbase32 encoding (http://philzimmermann.com/docs/human-oriented-base-32-encoding.txt)
            fn new() -> Self {
                let s = nanoid::nanoid!(8, &Self::ZBASE32_ALPHABET);
                // * unwrap is safe because we know the length is 8 and the characters are valid
                Self::from_bytes(s.as_bytes().try_into().unwrap())
            }

            fn try_from_str(value: &str) -> Result<Self, Zbase32IdError> {
                Self::validate_str(value)?;
                // * unwrap is safe because we know the length is 8 and the characters are valid
                Ok(Self::from_bytes(value.as_bytes().try_into().unwrap()))
            }
        }

        impl Default for $id_name {
            fn default() -> Self {
                $id_name::new()
            }
        }

        impl TryFrom<&str> for $id_name {
            type Error = Zbase32IdError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::try_from_str(value)
            }
        }

        impl TryFrom<String> for $id_name {
            type Error = Zbase32IdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_from_str(&value)
            }
        }

        impl From<$id_name> for String {
            fn from(value: $id_name) -> Self {
                String::from_utf8(value.0.to_vec()).unwrap()
            }
        }

        impl std::str::FromStr for $id_name {
            type Err = Zbase32IdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::try_from_str(value)
            }
        }
    };
}

declare_zbase32_id!(PacienteId);
declare_zbase32_id!(AtendimentoId);
declare_zbase32_id!(DocumentoId);
declare_zbase32_id!(EvolucaoId);
declare_zbase32_id!(PlanoAtendimentoId);
