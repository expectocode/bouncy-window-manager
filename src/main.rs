use std::collections::HashMap;
use std::collections::hash_map::Entry;

use rand;
use xcb;

fn move_window(conn: &xcb::Connection, window: xcb::Window, d_x: i32, d_y: i32) {
    // TODO
    let values = [
        (xcb::CONFIG_WINDOW_X as u16, 10),
        (xcb::CONFIG_WINDOW_Y as u16, 10),
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
    (rand::random(), rand::random())
}

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    dbg!(screen_num);

    let font = conn.generate_id();
    xcb::open_font(&conn, font, "cursor");

    let cursor = conn.generate_id();
    xcb::create_glyph_cursor(&conn, cursor, font, font, 0, 30, 0, 0, 0, 0, 0, 0);

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

        for child in children {
            match windows.entry(child.clone()) {
                Entry::Occupied(o) => {
                    dbg!(o.get());
                }
                Entry::Vacant(o) => {
                    o.insert((assign_direction(), assign_speed()));
                }
            }
        }

        move_window(&conn, children[0], 0, 0);
    }
}
