#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SoundEffectId {
    Enter,
    Selected,
    Cancel,
    Cursor,
    MatIn,
    MatOut,
    Get1,
    Get2,
    Get3,
    Effect1,
    Effect2,
}

impl From<SoundEffectId> for u8 {
    fn from(id: SoundEffectId) -> u8 {
        match id {
            SoundEffectId::Enter => 0u8,
            SoundEffectId::Selected => 1u8,
            SoundEffectId::Cancel => 2u8,
            SoundEffectId::Cursor => 3u8,
            SoundEffectId::MatIn => 4u8,
            SoundEffectId::MatOut => 5u8,
            SoundEffectId::Get1 => 6u8,
            SoundEffectId::Get2 => 7u8,
            SoundEffectId::Get3 => 8u8,
            SoundEffectId::Effect1 => 9u8,
            SoundEffectId::Effect2 => 10u8,
        }
    }
}
