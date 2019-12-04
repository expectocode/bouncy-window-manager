use xcb;

fn move_window(conn: &xcb::Connection, window: xcb::Window, d_x: i32, d_y: i32) {
    // TODO
    let values = [(xcb::CONFIG_WINDOW_X as u16, 10), (xcb::CONFIG_WINDOW_Y as u16, 10)];
    xcb::configure_window(conn, window, &values);
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

    loop {
        let tree = xcb::query_tree(&conn, root).get_reply().unwrap();
        let children = tree.children();
        dbg!(children);

        if children.len() == 0 {
            continue;
        }

        move_window(&conn, children[0], 0, 0);
    }

    return;

    let reply = xcb::grab_pointer(
        &conn,
        true,
        root,
        (xcb::EVENT_MASK_BUTTON_RELEASE
            | xcb::EVENT_MASK_BUTTON_PRESS
            | xcb::EVENT_MASK_BUTTON_MOTION
            | xcb::EVENT_MASK_POINTER_MOTION) as u16,
        xcb::GRAB_MODE_ASYNC as u8,
        xcb::GRAB_MODE_ASYNC as u8,
        xcb::NONE,
        cursor,
        xcb::CURRENT_TIME,
    )
    .get_reply()
    .unwrap();

    if reply.status() as u32 == xcb::GRAB_STATUS_SUCCESS {
        dbg!("got that good grab");
    }

    loop {
        let ev = conn.wait_for_event().unwrap();
        match ev.response_type() {
            xcb::MOTION_NOTIFY => {dbg!("hmmm");},
            xcb::BUTTON_PRESS => {dbg!("press");},
            xcb::BUTTON_RELEASE => {dbg!("release");},
            _ => unreachable!(),
        }
    }
}
