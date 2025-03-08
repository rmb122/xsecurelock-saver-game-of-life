mod life;
mod rle;

use life::LifeStatusDiff;
use std::env;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::connect;
use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::ReplyOrIdError;
use x11rb::wrapper::ConnectionExt as _;

use crate::life::LifeStatus;

fn setup_window<C: Connection>(
    conn: &C,
    screen: &Screen,
    mut window_size: (u16, u16),
    parent_window_id: Option<u32>,
) -> Result<Window, ReplyOrIdError> {
    let win_id = conn.generate_id()?;
    let win_aux = CreateWindowAux::new()
        .event_mask(EventMask::EXPOSURE)
        .background_pixel(screen.black_pixel);

    let parent = if let Some(parent) = parent_window_id {
        window_size = (screen.width_in_pixels, screen.height_in_pixels);
        parent
    } else {
        screen.root
    };

    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        parent,
        0,
        0,
        window_size.0,
        window_size.1,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &win_aux,
    )?;

    let title = "game-of-life";
    conn.change_property8(
        PropMode::REPLACE,
        win_id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes(),
    )
    .expect("Set title failed");

    conn.map_window(win_id).expect("Map window failed");
    Ok(win_id)
}

fn draw_diffs<C: Connection>(
    conn: &C,
    window_id: Drawable,
    alive_gc: Gcontext,
    dead_gc: Gcontext,
    diffs: &Vec<LifeStatusDiff>,
    cell_size: u16,
) {
    let mut alive_rectangles: Vec<Rectangle> = Vec::new();
    let mut dead_rectangles: Vec<Rectangle> = Vec::new();

    for diff in diffs {
        let rectangle = Rectangle {
            x: (diff.x * cell_size) as i16,
            y: (diff.y * cell_size) as i16,
            width: cell_size,
            height: cell_size,
        };

        if diff.status == LifeStatus::Alive {
            alive_rectangles.push(rectangle);
        } else {
            dead_rectangles.push(rectangle);
        }
    }
    conn.poly_fill_rectangle(window_id, alive_gc, &alive_rectangles)
        .unwrap();
    conn.poly_fill_rectangle(window_id, dead_gc, &dead_rectangles)
        .unwrap();
}

fn read_number_from_env<T: FromStr>(env_key: &str, default_value: T) -> T {
    if let Ok(env_value) = env::var(env_key) {
        if let Ok(env_value) = env_value.parse::<T>() {
            return env_value;
        } else {
            panic!("{} parse error", env_key);
        }
    } else {
        return default_value;
    };
}

fn read_string_from_env(env_key: &str, default_value: &str) -> String {
    if let Ok(env_value) = env::var(env_key) {
        return env_value;
    } else {
        return default_value.to_string();
    };
}

fn main() {
    let (conn, screen_num) = connect(None).expect("Failed to connect to the X11 server");
    let screen = &conn.setup().roots[screen_num];

    let parent_window_id: Option<u32> = if let Ok(env_window_id) = env::var("XSCREENSAVER_WINDOW") {
        Some(env_window_id.parse().expect("Invalid window id"))
    } else {
        None
    };
    let window_id = setup_window(&conn, screen, (1, 1), parent_window_id).unwrap();
    let cell_size = read_number_from_env("CGOL_CELL_SIZE", 5u16);
    let init_alive_probability = read_number_from_env("CGOL_INIT_ALIVE_PROBABILITY", 0.2f64);
    let init_rle_file = read_string_from_env("CGOL_INIT_RLE_FILE", "");
    let mutation_round_interval = read_number_from_env("CGOL_MUTATION_ROUND_INTERVAL", 0u32);
    let mutation_probability = read_number_from_env("CGOL_MUTATION_PROBABILITY", 0.001f64);
    let round_sleep_time = read_number_from_env("CGOL_ROUND_SLEEP_TIME", 0.1f64);

    // 检查下是否使用 rle 初始化
    let init_rle: Option<String> = if init_rle_file.len() > 0 {
        Some(std::fs::read_to_string(init_rle_file).expect("read rle file failed"))
    } else {
        None
    };

    let white_gc = GcontextWrapper::create_gc(
        &conn,
        window_id,
        &CreateGCAux::new()
            .graphics_exposures(0)
            .foreground(screen.white_pixel),
    )
    .unwrap();
    let black_gc = GcontextWrapper::create_gc(
        &conn,
        window_id,
        &CreateGCAux::new()
            .graphics_exposures(0)
            .foreground(screen.black_pixel),
    )
    .unwrap();

    conn.flush().unwrap();

    loop {
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::Expose(_) => {
                println!("exposed, start rendering...");
                break; // init complete
            }
            event => {
                println!("unknown event {:?}", event);
            }
        }
    }

    let parent_window_id = if let Some(parent_window_id) = parent_window_id {
        parent_window_id
    } else {
        window_id
    };
    let reply = conn
        .get_geometry(parent_window_id)
        .unwrap()
        .reply()
        .unwrap();
    let window_size = (reply.width, reply.height);
    println!("window_size: {:?}", window_size);

    let pixmap = PixmapWrapper::create_pixmap(
        &conn,
        screen.root_depth,
        window_id,
        window_size.0,
        window_size.1,
    )
    .unwrap();

    // use pixmap as local cache to avoid flicker
    conn.poly_fill_rectangle(
        pixmap.pixmap(),
        black_gc.gcontext(),
        &[Rectangle {
            x: 0,
            y: 0,
            width: window_size.0,
            height: window_size.1,
        }],
    )
    .unwrap();

    let mut life = life::Life::new(window_size.0 / cell_size, window_size.1 / cell_size);
    let mut round_count = 0;

    let diffs = if let Some(init_rle) = init_rle {
        life.initialize_rle(init_rle)
    } else {
        life.initialize_random(init_alive_probability)
    };

    draw_diffs(
        &conn,
        pixmap.pixmap(),
        white_gc.gcontext(),
        black_gc.gcontext(),
        &diffs,
        cell_size,
    );
    conn.flush().unwrap();

    let enable_mutation = mutation_round_interval > 0;

    loop {
        if enable_mutation {
            if round_count == mutation_round_interval {
                round_count = 0;

                let diffs = life.add_mutation(mutation_probability);
                draw_diffs(
                    &conn,
                    pixmap.pixmap(),
                    white_gc.gcontext(),
                    black_gc.gcontext(),
                    &diffs,
                    cell_size,
                );
            }
            round_count += 1;
        }

        let diffs = life.next_round();
        draw_diffs(
            &conn,
            pixmap.pixmap(),
            white_gc.gcontext(),
            black_gc.gcontext(),
            &diffs,
            cell_size,
        );

        conn.copy_area(
            pixmap.pixmap(),
            window_id,
            white_gc.gcontext(),
            0,
            0,
            0,
            0,
            window_size.0,
            window_size.1,
        )
        .unwrap();

        conn.flush().unwrap();
        thread::sleep(Duration::from_secs_f64(round_sleep_time));
    }
}
