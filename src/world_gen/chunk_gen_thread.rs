use crate::positions::MetaChunkPos;
use crate::world_gen::meta_chunk::MetaChunk;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SendError, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

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
            match message {
                Ok((pos, seed)) => {
                    let timer = Instant::now();
                    println!("started generation for {:?}", pos);
                    let result = gen_chunk_request_done
                        .send((MetaChunk::load_or_gen(pos, seed, false), pos));
                    match result {
                        Err(e) => println!("error while sending generated chunk: {}", e),
                        Ok(_) => {
                            (println!(
                                "done generation for: {:?} in {} sec",
                                pos,
                                timer.elapsed().as_secs_f32()
                            ))
                        }
                    }
                }
                Err(e) => {
                    if e == TryRecvError::Disconnected {
                        return;
                    }
                }
            }
            if message.is_err() {
            } else {
            }
        });
        return ChunkGenThread {
            chunk_generator_requester: gen_chunk_request,
            chunk_generator_receiver: gen_chunk_receiver_done,
            chunk_generator_thread: chunk_gen_thread,
        };
    }
    pub fn request(
        &self,
        pos: MetaChunkPos,
        seed: u32,
    ) -> Result<(), SendError<(MetaChunkPos, u32)>> {
        self.chunk_generator_requester.send((pos, seed))
    }
    pub fn get(&self) -> Result<(MetaChunk, MetaChunkPos), TryRecvError> {
        self.chunk_generator_receiver.try_recv()
    }
}
