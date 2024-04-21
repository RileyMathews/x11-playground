use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt, CreateWindowAux, EventMask, PropMode, WindowClass};
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let atoms = AtomCollection::new(&conn)?.reply()?;
    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0,
        0,
        screen.width_in_pixels,
        30,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .event_mask(EventMask::EXPOSURE),
    )?;

    x11rb::wrapper::ConnectionExt::change_property32(
        &conn,
        PropMode::REPLACE,
        win_id,
        atoms._NET_WM_WINDOW_TYPE,
        atoms.ATOM,
        &[atoms._NET_WM_WINDOW_TYPE_DOCK],
    )?
    .check()?;

    x11rb::wrapper::ConnectionExt::change_property32(
        &conn,
        PropMode::REPLACE,
        win_id,
        atoms._NET_WM_STRUT,
        atoms.CARDINAL,
        &[0, 0, 30, 0],
    )?
    .check()?;

    conn.map_window(win_id)?;

    let _ = conn.flush();
    loop {
        println!("Event: {:?}", conn.wait_for_event()?);
    }
}

x11rb::atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_NORMAL,
        _NET_WM_WINDOW_TYPE_DOCK,
        _NET_WM_WINDOW_TYPE_DIALOG,
        _NET_WM_WINDOW_TYPE_TOOLBAR,
        _NET_WM_WINDOW_TYPE_UTILITY,
        _NET_WM_WINDOW_TYPE_DESKTOP,
        _NET_WM_WINDOW_TYPE_NOTIFICATION,
        _NET_WM_STATE,
        _NET_WM_STATE_STICKY,
        _NET_WM_STATE_ABOVE,
        _NET_WM_STATE_BELOW,
        _NET_WM_NAME,
        _NET_WM_STRUT,
        _NET_WM_STRUT_PARTIAL,
        WM_NAME,
        UTF8_STRING,
        COMPOUND_TEXT,
        CARDINAL,
        ATOM,
        WM_CLASS,
        STRING,
    }
}
