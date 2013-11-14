/** A library providing functions to read and write files in the MIDI file format. Most of this was
 * taken from the high-level view provided at 
 *
 * http://faydoc.tripod.com/formats/mid.htm
 *
 * and the more comprehensive set of pages at 
 *
 * http://www.recordingblogs.com/sa/tabid/88/Default.aspx?topic=Musical+Instrument+Digital+Interface+(MIDI)
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
use std::vec::{with_capacity, append_one};
use std::rt::io::{File, io_error};

// TODO:  Write a Rust macro to chain Option<> Pattern matches, so Nones always just return None,
// but assume you got the other thing?
// TODO: Parallelize the track reads rather than make it sequential?


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
    delta_time: u32,
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
    let path = &Path::new(filename);
    let mut reader = File::open(&Path::new(filename));

    let mut contents_buf = with_capacity(100);
    let bytes_read : Option<uint> = reader.read(contents_buf);

    match parse_header(contents_buf) {
        Some(header) => {
            match parse_all_tracks(header, contents_buf) {
                Some(tracks) => {
                    let new_midifile = MidiFile{header: header, tracks : tracks};
                    Some(new_midifile)
                }
                None => { None }
            } // match parse_all_tracks
        }
        None => { None }
    } // match parse_header
}


/// Parses the first 14 bytes, which comprise a MIDI header.
fn parse_header(buf : &[u8]) -> Option<MidiHeader> {
    let err = buf[0] != ('M' as u8) || buf[1] != ('T' as u8)
           || buf[2] != ('h' as u8) || buf[3] != ('d' as u8)
           || buf[4] != 0           || buf[5] != 0
           || buf[6] != 0           || buf[7] != 6;

    if err {
        error!("Malformed MIDI header -- first 8 bytes nonstandard.");
        None
    } else {
        let ff = u16_from_u8_at(buf, 8);
        let num_tracks = u16_from_u8_at(buf, 10);
        let ticks_per_quarter = u16_from_u8_at(buf, 12);

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

/// Parses all the tracks in a MIDI file, read into a buffer.
fn parse_all_tracks(header : MidiHeader, buf : &[u8]) -> Option<~[MidiTrack]> {
    None

    // get a list of the offsets from each length.
    // parallel call parse_track on all of them.
    // get the response, if all succeed, collect them, else none.

}

/// Parses an individual track beginning at the specified offset.
fn parse_track(buf : &[u8], offset : u32) -> Option<MidiTrack> {
    // chunk ID (4 bytes of MTrk)
    let err = buf[0] != ('M' as u8) || buf[1] != ('T' as u8)
           || buf[2] != ('r' as u8) || buf[3] != ('k' as u8);
    if err {
        error!("Malformed MIDI header -- first 4 bytes nonstandard, offset is {}", offset);
        None
    } else {
        // track size
        let track_size = get_track_size(buf, offset);
        let mut event_offset = offset + 8;
        let mut midi_events = with_capacity(0);
        let mut error = false;
        // events in sequence.
        while event_offset < ((offset + 4) + track_size) {
            match parse_event(buf, event_offset) {
                None => {
                    error!("Malformed event, somewhere near offset {}", event_offset);
                    error = true;
                    break;
                }
                Some((x, new_offset)) => {
                    midi_events = append_one(midi_events, x);
                    event_offset = new_offset
                }
            }
        }
        match error {
            true => Some(MidiTrack{ track_length : track_size, events : midi_events }),
            false => None
        }
    }
}

fn parse_event(buf : &[u8], offset : u32) -> Option<(MidiEvent, u32)> {
    match parse_ticks(buf, offset) {
        (ticks, new_offset) => {
           match parse_message(buf, new_offset) {
               None => { None }
               Some((message, new_offset)) => { 
                    Some((MidiEvent{ delta_time : ticks, message : message }, new_offset))
               }
           }
        }
    }
}

// MIDI spec says length should be at most 4 bytes, so some hardcoded values here. Should probably
// have more safety bits than the simple assert.
// 
// A small reminder of how MIDI Events work: you start with the number of ticks, followed by a MIDI
// message. This parses the ticks, which is variable-length.
//
// The number of ticks can be expressed with at least 1 and at most 4 bytes. All bytes must have a
// '1' in the highest order position, except the last, which must have a 0. When you've read all the
// bytes, you combine the bottom 7 of all of them into one long bitstring, then evaluate it for the
// number of ticks.
//
// The code is a little hairy since making a bunch of 8-bit bytes into 7-bit bytes to combine to
// some bitstring that is a multiple of 7... maybe my bitflip-foo isn't so good, but it's hard to
// find a way to do it easily without allocating a binary buffer or something.
fn parse_ticks(buf : &[u8], offset : u32) -> (u32, u32) {
    println!("----------");
    let mut time_offset = 0;
    let mut time_buffer : [u32, ..4] = [0,0,0,0];
    let mut return_value = (0,0);
    loop {
        assert!(time_offset < 4);
        let curr = buf[offset + time_offset];
        time_buffer[time_offset] = (lower_seven_bits(curr) as u32);
        if msb_is_one(curr) {
            time_offset += 1;
        } else {
            let mut loop_offset = 0;
            let mut time_ticks = 0;
            while loop_offset <= time_offset {
                let byte_correction = time_offset - loop_offset;
                let contribution = (time_buffer[0 + loop_offset] << (8 * byte_correction)) >> byte_correction;
                time_ticks = time_ticks | contribution;
                loop_offset += 1;
            }
            return_value = (time_ticks, offset + time_offset + 1);
            break;
        }
    }
    return_value
}

fn msb_is_one(number : u8) -> bool {
    number > 127
}
fn lower_seven_bits(number : u8) -> u8 {
    number & 0b01111111
}

fn parse_message(buf : &[u8], offset : u32) -> Option<(MidiMessage, u32)> {
  None
}


// Helper functions

// In C, I'd memcpy two uint8 bytes into a pointer to a uint16, but give there's no
// memcpy here (well, without `unsafe`) I'm using silly bit tricks to do number conversions.
// Got these from how Rust io::net parses IP addresses.
fn u16_from_u8_at(buf : &[u8], offset : u32) -> u16 {
   (buf[offset] as u16 << 8) | (buf[offset + 1] as u16)
}

fn u32_from_u8_at(buf : &[u8], offset : u32) -> u32 {
   (buf[offset] as u32 << 24)
   | (buf[offset + 1] as u32 << 16)
   | (buf[offset + 2] as u32 << 8)
   | (buf[offset + 3] as u32)
}

fn get_track_size(buf : &[u8], offset : u32) -> u32 {
    let size = u32_from_u8_at(buf, offset + 4);
    size
}

fn file_format_from_u16(value : u16) -> Option<FileFormat> {
    match value {
        1 => Some(SingleTrack),
        2 => Some(MultipleSynchronous),
        3 => Some(MultipleAsynchronous),
        _ => None
    }
}

// Writing
// Undefined for now, since we just want to read.


//Tests!
#[test]
fn test_parse_header_standard() {
   let test1 = [0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06,
                0x00, 0x01,
                0x00, 0x05,
                0x00, 0xa0];
   let rslt = parse_header(test1);
   match rslt {
       None => { assert!(false) }
       Some(x) => {
            assert!(x.num_tracks == 5);
            assert!(x.ticks_per_quarter == 160);
            match x.file_format {
                SingleTrack => assert!(true),
                _ => assert!(false)
            }
       }
   }

   let test2  = [0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06,
                 0x00, 0x02,
                 0x0a, 0x00,
                 0x01, 0x00];
   let rslt2 = parse_header(test2);
   match rslt2 {
       None => { assert!(false) }
       Some(x) => {
            assert!(x.num_tracks == 2560);
            assert!(x.ticks_per_quarter == 256);
            match x.file_format {
                MultipleSynchronous => assert!(true),
                _ => assert!(false)
            }
       }
   }
}

#[test]
fn test_parse_header_fail() {
    let test3  = [0x4D, 0x34, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06,
                 0x00, 0x02,
                 0x0a, 0x00,
                 0x01, 0x00];
   let rslt3 = parse_header(test3);
   match rslt3 {
       None => { assert!(true) }
       Some(_) => { assert!(false) }
   }
}

#[test]
fn test_parse_ticks_easy() {
    let test_buf = [0x50, 0x90, 0x26, 0x3C];
    match parse_ticks(test_buf, 0) {
        (ticks, new_offset) => {
            assert!(ticks == 80);
            assert!(new_offset == 1);
        }
    }
}

#[test]
fn test_parse_ticks_hard() {
    let test_buf = [0x83, 0x60, 0x26, 0x00];
    match parse_ticks(test_buf, 0) {
        (ticks, new_offset) => {
            assert!(ticks == 480);
            assert!(new_offset == 2);
        }
    }
}

