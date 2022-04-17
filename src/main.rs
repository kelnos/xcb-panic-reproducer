use xcb::x;

fn main() -> xcb::Result<()> {
    let (conn, screen_num) = xcb::Connection::connect_with_extensions(
        None,
        &[xcb::Extension::ScreenSaver],
        &[]
    )?;
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let wid: x::Window = conn.generate_id();
    conn.send_and_check_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: x::COPY_FROM_PARENT,
        value_list: &[],
    })?;

    Ok(())
}
