use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use crossbeam_queue::SegQueue;
use psimple::Simple;
use pulse::sample;
use pulse::stream::Direction;

use self::Action::*;
use crate::mp3::MP3Decoder;

const BUFFER_SIZE: usize = 1_000;
const DEFAULT_RATE: u32 = 44100;
const PB_NAME: &str = "MP3";
const PB_DESC: &str = "MP3 Playback";

enum Action {
    Load(PathBuf),
    Stop,
}

#[derive(Clone)]
struct EventLoop {
    queue: Arc<SegQueue<Action>>,
    playing: Arc<Mutex<bool>>,
}

impl EventLoop {
    fn new() -> Self {
        EventLoop {
            queue: Arc::new(SegQueue::new()),
            playing: Arc::new(Mutex::new(false)),
        }
    }
}

pub struct Player {
    app_state: Arc<Mutex<super::State>>,
    event_loop: EventLoop,
}

impl Player {
    pub(crate) fn new(app_state: Arc<Mutex<super::State>>) -> Self {
        let event_loop = EventLoop::new();

        {
            let app_state = app_state.clone();
            let event_loop = event_loop.clone();
            thread::spawn(move || {
                let mut buffer = [0u8; BUFFER_SIZE];
                let spec = sample::Spec {
                    format: sample::Format::S16be,
                    channels: 2,
                    rate: DEFAULT_RATE,
                };
                let mut playback = Simple::new(
                    None,
                    PB_NAME,
                    Direction::Playback,
                    None,
                    PB_DESC,
                    &spec,
                    None,
                    None,
                )
                .unwrap();
                let mut source = None;
                loop {
                    if let Ok(action) = event_loop.queue.pop() {
                        match action {
                            Load(path) => {
                                let file = File::open(path).unwrap();
                                source = Some(MP3Decoder::new(BufReader::new(file)).unwrap());
                                let rate = source
                                    .as_ref()
                                    .map(|source| source.sample_rate())
                                    .unwrap_or(DEFAULT_RATE);
                                playback = Simple::new(
                                    None,
                                    PB_NAME,
                                    Direction::Playback,
                                    None,
                                    PB_DESC,
                                    &spec,
                                    None,
                                    None,
                                )
                                .unwrap();
                                app_state.lock().unwrap().stopped = false;
                            }
                            Stop => {}
                        }
                    } else if *event_loop.playing.lock().unwrap() {
                        let mut written = false;
                        if let Some(ref mut source) = source {
                            let size = iter_to_buffer(source, &mut buffer);
                            if size > 0 {
                                playback.write(&buffer[..size]).unwrap();
                                written = true;
                            }
                        }
                        if !written {
                            app_state.lock().unwrap().stopped = true;
                            *event_loop.playing.lock().unwrap() = false;
                            source = None;
                        }
                    }
                }
            });
        }

        Player {
            app_state,
            event_loop,
        }
    }

    pub fn load(&self, path: PathBuf) {
        self.event_loop.queue.push(Load(path));
    }
}

/**
 * utility functions
 */
fn iter_to_buffer<I: Iterator<Item = u8>>(iter: &mut I, buffer: &mut [u8; BUFFER_SIZE]) -> usize {
    let mut iter = iter.take(BUFFER_SIZE);
    let mut index = 0;

    while let Some(sample1) = iter.next() {
        buffer[index] = sample1;
        index += 1;
    }
    index
}
