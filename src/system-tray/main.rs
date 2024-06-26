use tao::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
    menu::{ContextMenu, MenuId, MenuItemAttributes},
    system_tray::{SystemTray, SystemTrayBuilder, Icon},
};
use image::io::Reader as ImageReader;
use std::io::Cursor;

fn load_icon(path: &str) -> Icon {
    let file = std::fs::read(path).expect("Failed to read icon file");
    let image = ImageReader::new(Cursor::new(file))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image");
    let rgba = image.into_rgba8();
    let (width, height) = rgba.dimensions();
    Icon::from_rgba(rgba.into_raw(), width, height).expect("Failed to create icon")
}

fn main() {
    let event_loop = EventLoop::new();

    let mut tray_menu = ContextMenu::new();
    let show_id = MenuId::new("0");
    let exit_id = MenuId::new("1");
    tray_menu.add_item(MenuItemAttributes::new("Show").with_id(show_id));
    tray_menu.add_item(MenuItemAttributes::new("Exit").with_id(exit_id));

    let tray_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "image path");
    let tray_icon = load_icon(tray_icon_path);

    let system_tray = SystemTrayBuilder::new(tray_icon, Some(tray_menu))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MenuEvent {
                menu_id,
                origin: _,
                ..
            } => {
                if menu_id == show_id {
                    println!("Show clicked");
                    // Handle Show action
                } else if menu_id == exit_id {
                    println!("Exit clicked");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
