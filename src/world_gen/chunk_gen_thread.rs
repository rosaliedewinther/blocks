use crate::positions::MetaChunkPos;
use crate::world_gen::meta_chunk::MetaChunk;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;

pub struct ChunkGenThread {
    pub chunk_generator_requester: Sender<(MetaChunkPos, u32)>,
    pub chunk_generator_receiver: Receiver<(MetaChunk, MetaChunkPos)>,
    pub chunk_generator_thread: JoinHandle<()>,
}

impl ChunkGenThread {
    pub fn new() -> ChunkGenThread {
        let (gen_chunk_request, gen_chunk_receiver) = mpsc::channel();
        let (gen_chunk_request_done, gen_chunk_receiver_done) = mpsc::channel();
        let chunk_gen_thread = thread::spawn(move || loop {
            let message: Result<(MetaChunkPos, u32), TryRecvError> = gen_chunk_receiver.try_recv();
            if message.is_err() {
                if message.err().unwrap() == TryRecvError::Disconnected {
                    return;
                }
            } else {
                let unwrapped_message = message.unwrap();
                gen_chunk_request_done.send((
                    MetaChunk::load_or_gen(unwrapped_message.0, unwrapped_message.1),
                    unwrapped_message.0,
                ));
            }
        });
        return ChunkGenThread {
            chunk_generator_requester: gen_chunk_request,
            chunk_generator_receiver: gen_chunk_receiver_done,
            chunk_generator_thread: chunk_gen_thread,
        };
    }
    pub fn request(&self, pos: MetaChunkPos, seed: u32) {
        self.chunk_generator_requester.send((pos, seed));
    }
    pub fn get(&self) -> Option<(MetaChunk, MetaChunkPos)> {
        let message = self.chunk_generator_receiver.try_recv();
        if message.is_ok() {
            return Some(message.unwrap());
        } else {
            return None;
        }
    }
}
