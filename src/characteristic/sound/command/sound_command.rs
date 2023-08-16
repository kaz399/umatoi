//! Official Specification: <https://toio.github.io/toio-spec/docs/ble_sound>

use crate::payload::ToPayload;
use std::cmp;

use super::super::def::command_id::CommandId;
use super::super::def::midi_note::MidiNote;
use super::super::def::sound_effect_id::SoundEffectId;

// PlaySoundEffect
// ref:<https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-sound-effects>

#[derive(Debug, Copy, Clone)]
pub struct PlaySoundEffect {
    pub command: CommandId,
    pub id: SoundEffectId,
    pub volume: u8,
}

impl Default for PlaySoundEffect {
    fn default() -> Self {
        Self {
            command: CommandId::PlaySoundEffect,
            id: SoundEffectId::Enter,
            volume: 0xffu8,
        }
    }
}

impl ToPayload<Vec<u8>> for PlaySoundEffect {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.command.into(), self.id.into(), self.volume];
        payload
    }
}

// PlayMidiNote
// ref:<https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers>

#[derive(Debug, Clone)]
pub struct PlayMidiNote {
    pub command: CommandId,
    pub repeat: u8,
    pub midi_notes: Vec<MidiNote>,
}

impl Default for PlayMidiNote {
    fn default() -> Self {
        Self {
            command: CommandId::PlayMidiNotes,
            repeat: 1u8,
            midi_notes: Vec::<MidiNote>::new(),
        }
    }
}

impl ToPayload<Vec<u8>> for PlayMidiNote {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![self.command.into(), self.repeat];
        let num_of_notes = cmp::min(self.midi_notes.len(), u8::MAX.into());
        assert!(num_of_notes <= u8::MAX.into());
        payload.push(num_of_notes.try_into().unwrap());
        let midi_notes = &self.midi_notes[0..num_of_notes];
        for note in midi_notes {
            payload.extend(note.to_payload().to_vec());
        }
        payload
    }
}

// StopSound
// ref:<https://toio.github.io/toio-spec/en/docs/ble_sound/#stop-playing>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StopSound {
    pub command: CommandId,
}

impl Default for StopSound {
    fn default() -> Self {
        Self {
            command: CommandId::StopSound,
        }
    }
}

impl ToPayload<Vec<u8>> for StopSound {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.command.into()];
        payload
    }
}
