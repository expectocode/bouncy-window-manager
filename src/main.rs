use std::collections::HashMap;
use std::collections::hash_map::Entry;

use rand::{self, thread_rng, Rng};
use xcb;

fn move_window(conn: &xcb::Connection, window: xcb::Window, d_x: i32, d_y: i32) {
    // TODO
    let values = [
        (xcb::CONFIG_WINDOW_X as u16, 10),
        (xcb::CONFIG_WINDOW_Y as u16, 10),
    ];
    xcb::configure_window(conn, window, &values);
}

fn resize_window(conn: &xcb::Connection, window: xcb::Window, width: u32, height: u32) {
    // TODO
    let values = [
        (xcb::CONFIG_WINDOW_WIDTH as u16, width),
        (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
    ];
    xcb::configure_window(conn, window, &values);
}

fn assign_direction() -> i32 {
    let x: f32 = rand::random();
    (x * 360f32) as i32
}

fn assign_speed() -> f32 {
    let x: f32 = rand::random();
    x * 2f32
}

fn assign_size() -> (u32, u32) {
    let mut rng = thread_rng();
    (rng.gen_range(20, 100), rng.gen_range(20, 100))
}

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    dbg!(screen_num);

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();

    let mut windows = HashMap::new();

    loop {
        let tree = xcb::query_tree(&conn, root).get_reply().unwrap();
        let children = tree.children();
        dbg!(children);

        if children.len() == 0 {
            continue;
        }

        for &child in children {
            match windows.entry(child) {
                Entry::Occupied(o) => {
                    dbg!(o.get());
                }
                Entry::Vacant(o) => {
                    let (w, h) = assign_size();
                    resize_window(&conn, child, w, h);
                    o.insert((assign_direction(), assign_speed()));
                }
            }
        }

        move_window(&conn, children[0], 0, 0);
    }
}
