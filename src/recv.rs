#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(not(feature = "std"))]
use core2::io::{Read, Write};

use crate::Vec;

use consts::*;
use proto::*;
use frame::*;

use crate::Result;
use crate::Error;

#[derive(Debug, PartialEq)]
enum State {
    /// Sending ZRINIT
    SendingZRINIT,

    /// Processing ZFILE supplementary data
    ProcessingZFILE,

    /// Receiving file's content
    ReceivingData,

    /// Checking length of received data
    CheckingData,

    /// All works done, exiting
    Done,
}

impl State {
    fn new() -> State {
        State::SendingZRINIT
    }

    fn next(self, frame: &Frame) -> State {
        match (self, frame.get_frame_type()) {
            (State::SendingZRINIT, ZFILE)   => State::ProcessingZFILE,
            (State::SendingZRINIT, _)       => State::SendingZRINIT,

            (State::ProcessingZFILE, ZDATA) => State::ReceivingData,
            (State::ProcessingZFILE, _)     => State::ProcessingZFILE,

            (State::ReceivingData, ZDATA)   => State::ReceivingData,
            (State::ReceivingData, ZEOF)    => State::CheckingData,

            (State::CheckingData, ZDATA)    => State::ReceivingData,
            (State::CheckingData, ZFIN)     => State::Done,

            (s, _) => {
               error!("Unexpected (state, frame) combination: {:#?} {}", s, frame);
               s // don't change current state
            },
        }
    }
}

/// Receives data by Z-Modem protocol
pub fn recv<RW, W>(mut rw: RW, mut w: W, delay_10ms: &mut dyn FnMut() -> ()) -> Result<usize>
    where RW: Read + Write,
          W:  Write,
{
    let mut count = 0;

    let mut state = State::new();

    write_zrinit(&mut rw)?;

    while state != State::Done {
        if !find_zpad(&mut rw)? {
            continue;
        }

        let frame = match parse_header(&mut rw)? {
            Some(x) => x,
            None    => { recv_error(&mut rw, &state, count)?; continue },
        };

        state = state.next(&frame);
        debug!("State: {:?}", state);

        // do things according new state
        match state {
            State::SendingZRINIT => {
                write_zrinit(&mut rw)?;
            },
            State::ProcessingZFILE => {
                let mut buf = Vec::new();

                if recv_zlde_frame(frame.get_header(), &mut rw, &mut buf)?.is_none() {
                    write_znak(&mut rw)?;
                }
                else {
                    write_zrpos(&mut rw, count)?;

                    // TODO: process supplied data
                    /*
                    rprint!("Got filename \"");
                    for c in buf {
                        if c == 0 {
                            break;
                        }
                        rprint!("{}", c as char);
                    }
                    rprint!("\"\n");
                    */
                }
            },
            State::ReceivingData => {
                if frame.get_count() != count ||
                    !recv_data(frame.get_header(), &mut count, &mut rw, &mut w)? {
                    write_zrpos(&mut rw, count)?;
                }
            },
            State::CheckingData => {
                if frame.get_count() != count {
                    error!("ZEOF offset mismatch: frame({}) != recv({})", frame.get_count(), count);
                    // receiver ignores the ZEOF because a new zdata is coming
                }
                else {
                    write_zrinit(&mut rw)?;
                }
            },
            State::Done => {
                write_zfin(&mut rw)?;
                delay_10ms(); // sleep a bit. Lionel: needed, really?
            },
        }
    }

    Ok(count as usize)
}

fn recv_error<W>(w: &mut W, state: &State, count: u32) -> Result<()>
    where W: Write
{
    // TODO: flush input

    let result;
    match *state {
        State::ReceivingData => result = write_zrpos(w, count),
        _                    => result = write_znak(w),
    }
    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::from(e)),
    }
}

