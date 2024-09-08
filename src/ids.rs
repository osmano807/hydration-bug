use serde::{Serialize, Deserialize};

#[derive(Debug, thiserror::Error, PartialEq, Clone, Serialize, Deserialize)]
pub enum IdError {
    #[error("Dummy")]
    Dummy,
}

macro_rules! declare_id {
    ($id_name:ident, $id_default_value:expr) => {
        #[derive(
            Serialize,
            Deserialize,
            Clone,
            PartialEq
        )]
        // Tell serde to deserialize data into a String and then try to convert it, using our impl traits with checks
        // https://github.com/serde-rs/serde/issues/939#issuecomment-939514114
        #[serde(try_from = "String", into = "String")]
        pub struct $id_name(String);

        impl std::fmt::Display for $id_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Convert the [u8; 8] to a str and write it
                write!(
                    f,
                    "{}",
                    self.0
                )
            }
        }

        // required by diesel
        impl std::fmt::Debug for $id_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($id_name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl $id_name {
            /// Generates a 8 char unique id
            /// with zbase32 encoding (http://philzimmermann.com/docs/human-oriented-base-32-encoding.txt)
            fn new() -> Self {
                Self($id_default_value)
            }

            fn try_from_str(value: &str) -> Result<Self, IdError> {
                Ok(Self(value.to_string()))
            }
        }

        impl Default for $id_name {
            fn default() -> Self {
                $id_name::new()
            }
        }

        impl TryFrom<&str> for $id_name {
            type Error = IdError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::try_from_str(value)
            }
        }

        impl TryFrom<String> for $id_name {
            type Error = IdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Ok(Self(value))
            }
        }

        impl From<$id_name> for String {
            fn from(value: $id_name) -> Self {
                value.0
            }
        }

        impl std::str::FromStr for $id_name {
            type Err = IdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::try_from_str(value)
            }
        }
    };
}

declare_id!(PacienteId, "pppppppp".to_string());
declare_id!(AtendimentoId, "aaaaaaaa".to_string());
declare_id!(EvolucaoId, "eeeeeeee".to_string());
