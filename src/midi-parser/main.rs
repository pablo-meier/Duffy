/** A library providing functions to read and write files in the MIDI file format.
 */

#[ name = "Rust MIDI" ]
#[ vers = "0.1" ]
/// Package ID is name concatenated with vers, separated by space, fed to md5sum
#[ package_id = "f34701c762288492b2e7f98f36922860" ]
#[ desc = "MIDI library for rust. Provides programmatic access to data in MIDI files." ]
#[ license = "GPL" ]
#[ author = "Paul Meier" ]
#[ crate_type = "lib" ]

#[ warn(non_camel_case_types) ]

use std::option::{Some, None};
use std::path::Path;
use std::container::Container;
use std::vec::{with_capacity};
use std::rt::io::{File, io_error, Open};

// Reading

/// A MidiFile contains a Header, and a list of Tracks,
pub struct MidiFile {
    header: MidiHeader,
    tracks: ~[MidiTrack]
}

/// A MidiHeader contains the FileFormat, the number of tracks, and the 'ticks' per quarter note.
pub struct MidiHeader {
    file_format: FileFormat,
    num_tracks: u16,
    ticks_per_quarter: u16
}

/// A Miditrack itself only contains its own length and a list of the events.
pub struct MidiTrack {
    track_length: u32,
    events: ~[MidiEvent]
}

pub struct MidiEvent {
    delta_time: u16,
    message: MidiMessage
}

pub struct MidiMessage {
    status_byte: StatusByte,
    message: u16
}

/// MIDI files can have one of three formats, defined in the header of the file.
pub enum FileFormat {
    SingleTrack = 1,
    MultipleSynchronous = 2,
    MultipleAsynchronous = 3
}

/// The various commands a MidiMessage can contain. Codes and descriptions lifted from 
/// http://www.recordingblogs.com/sa/tabid/88/Default.aspx?topic=Status+byte+(of+a+MIDI+message)
pub enum StatusByte {
    /// Release a note and stop playing it.
    NoteOff = 0x80,
    /// Play a note and start sounding it.
    NoteOn = 0x90,
    /// Apply pressure to a note playing, similar to applying pressure to electronic keyboard keys.
    Aftertouch = 0xA0,
    /// Affect a controller, such as a slider, knob, or switch.
    Controller = 0xB0,
    /// Assign a program to a MIDI channel, such as an instrument, patch, or preset.
    ProgramChange = 0xC0,
    /// Apply pressure to a MIDI channel, similar to applying pressure to electronic keyboard keys.
    ChannelPressure = 0xD0,
    /// Change a channel pitch up or down.
    PitchWheel = 0xE0,
    /// Perform some device specific task.
    SystemExclusive = 0xF0,
    /// Set the MIDI time to keep in line with some other device.
    MidiTimeCode = 0xF1,
    /// Cue to a point in the MIDI sequence to be ready to play.
    SongPositionPointer = 0xF2,
    /// Set a sequence for playback.
    SongSelect = 0xF3,
    /// Tune.
    TuneRequest = 0xF6,
    /// Understand the position of the MIDI clock (when synchronized to another device).
    MidiClock = 0xF8,
    /// Start playback of some MIDI sequence.
    MidiStart = 0xFA,
    /// Resume playback of some MIDI sequence.
    MidiContinue = 0xFB,
    /// Stop playback of some MIDI sequence.
    MidiStop = 0xFC,
    /// Understand that a MIDI connection exists (if there are no other MIDI messages).
    ActiveSense = 0xFE,
    /// Reset to default state.
    Reset = 0xFF
}

pub fn parse_file(filename : &str) -> Option<MidiFile> {
    // Open the file according to the filename
    let mut reader = File::open(&Path::new(filename));
    //let filesize = 100;

    // If it's all valid
    let mut contents_buf = with_capacity(100);
    let bytes_read : Option<uint> = reader.read(contents_buf);

    // parse the header
    match parse_header(contents_buf, 0) {
        Some(fake_header) => {
            // parse all the tracks
            let fake_tracks : ~[MidiTrack] = with_capacity(0);

            // initialize a new MidiFile struct
            let new_midifile = MidiFile{header: fake_header, tracks : fake_tracks};

            // return the new MidiFile
            Some(new_midifile)
        }
        None => { None }
    }
}


fn parse_header(buf : &[u8], offset : u32) -> Option<MidiHeader> {
    let err = buf[0] != ('M' as u8) || buf[1] != ('T' as u8)
           || buf[2] != ('h' as u8) || buf[3] != ('d' as u8)
           || buf[4] != 0           || buf[5] != 0
           || buf[6] != 0           || buf[7] != 6;

    if err {
        error!("Malformed MIDI header -- first 8 bytes nonstandard.");
        None
    } else {
        let ff = (buf[8] as u16 << 8) | (buf[9] as u16);
        let num_tracks = (buf[10] as u16 << 8) | (buf[11] as u16);
        let ticks_per_quarter = (buf[12] as u16 << 8) | (buf[13] as u16);

        match file_format_from_u16(ff) {
            Some(x) => { Some(MidiHeader{file_format : x,
                                         num_tracks : num_tracks,
                                         ticks_per_quarter : ticks_per_quarter}) }
            None => {
                error!("Invalid file format in header.");
                None
            }
        }
    }
}

fn file_format_from_u16(value : u16) -> Option<FileFormat> {
    match value {
        1 => Some(SingleTrack),
        2 => Some(MultipleSynchronous),
        3 => Some(MultipleAsynchronous),
        _ => None
    }
}

// Parses a track of the MIDI buffer. NOTE: this is a good candidate for concurrency, since we
// only ever read from the buffer. If we had a routine to scan just for the lengths of all of them,
// we could put a separate process on each track.
// fn parse_track(buf : &[u8], offset : u32)  -> (MidiTrack, uint) {
// }

// Writing
// Undefined for now, since we just want to read.
