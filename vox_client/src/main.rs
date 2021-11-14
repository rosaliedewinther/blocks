use vox_window::main_loop::main_loop_run;

mod game;
mod ui_data;

fn main() {
    vox_shared::logger::setup_logger();
    let g = crate::game::VoxGame {};
    main_loop_run(g, None, None, true,  vox_shared::constants::MS_PER_TICK);
}
