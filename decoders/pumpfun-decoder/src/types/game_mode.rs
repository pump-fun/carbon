#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq, Eq)]
pub enum GameMode {
    Auto,
    Manual,
    Climb,
}
