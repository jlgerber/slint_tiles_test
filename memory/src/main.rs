slint::slint!(
    import { MainWindow } from "../ui/main.slint";
);
fn main() {
    use slint::Model;
    let main_win = MainWindow::new();
    let mut tiles: Vec<TileData> = main_win.get_memory_tiles().iter().collect();

    tiles.extend(tiles.clone());

    //randomly mix the tiels
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);


    // assing
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_win.set_memory_tiles(tiles_model.clone().into());

    main_win.on_tile_open(move |tile_index| {
        let mut tile = tiles_model.row_data(tile_index as usize).unwrap();
        tile.is_open = !tile.is_open;
        tiles_model.set_row_data(tile_index as usize, tile);
    });
    main_win.run();
}
