use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use rand::{self, thread_rng, Rng};
use xcb;

fn move_window(conn: &xcb::Connection, window: xcb::Window, x: i16, y: i16) {
    let values = [
        (xcb::CONFIG_WINDOW_X as u16, x as u32),
        (xcb::CONFIG_WINDOW_Y as u16, y as u32),
    ];
    xcb::configure_window(conn, window, &values);
}

fn resize_window(conn: &xcb::Connection, window: xcb::Window, width: u32, height: u32) {
    let values = [
        (xcb::CONFIG_WINDOW_WIDTH as u16, width),
        (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
    ];
    xcb::configure_window(conn, window, &values);
}

fn assign_direction() -> f32 {
    (rand::random::<f32>() * ((std::f32::consts::PI / 2.0) - 0.8)) + 0.4
}

fn assign_speed() -> f32 {
    let x: f32 = rand::random();
    (x + 1f32) * 2.3
}

fn assign_size() -> (u32, u32) {
    let mut rng = thread_rng();
    (rng.gen_range(20, 100), rng.gen_range(20, 100))
}

// Returns new direction
fn bounce_move(
    conn: &xcb::Connection,
    root_bottom: i16,
    root_right: i16,
    win: xcb::Window,
    dir: f32,
    speed: f32,
) -> f32 {
    // Move first, fix later ;)
    let orig_geom = xcb::get_geometry(conn, win).get_reply().unwrap();
    let d_x = (dir.cos() * speed) as i16;
    let d_y = (dir.sin() * speed) as i16;
    move_window(conn, win, orig_geom.x() + d_x, orig_geom.y() + d_y);

    let geom = xcb::get_geometry(conn, win).get_reply().unwrap();

    if geom.x() == orig_geom.x() && geom.y() == orig_geom.y() {
        // nudge
        move_window(conn, win, geom.x() + 1, geom.y() + 1);
    }

    let bottom = geom.y() + geom.height() as i16;
    let top = geom.y();
    let right = geom.x() + geom.width() as i16;
    let left = geom.x();

    let mut dir = dir;

    if bottom > root_bottom || top < 0 {
        dir = (std::f32::consts::PI * 2f32) - dir;
    }
    if right > root_right || left < 0 {
        dir = (std::f32::consts::PI) - dir;
    }
    dir
}

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();

    let mut windows = HashMap::new();

    let root_geom = xcb::get_geometry(&conn, root).get_reply().unwrap();
    let root_bottom = root_geom.height() as i16;
    let root_right = root_geom.width() as i16;

    loop {
        let loop_start = Instant::now();
        let tree = xcb::query_tree(&conn, root).get_reply().unwrap();
        let children = tree.children();

        for &child in children {
            match windows.entry(child) {
                Entry::Occupied(mut o) => {
                    let (direction, speed) = o.get();
                    o.get_mut().0 =
                        bounce_move(&conn, root_bottom, root_right, child, *direction, *speed);
                }
                Entry::Vacant(o) => {
                    let (w, h) = assign_size();
                    resize_window(&conn, child, w, h);
                    o.insert((assign_direction(), assign_speed()));
                }
            }
        }
        let now = Instant::now();
        let elapsed = now - loop_start;
        let time_to_sleep = Duration::from_millis(50).checked_sub(elapsed).unwrap_or(Duration::new(0, 0));
        thread::sleep(time_to_sleep);
    }
}
