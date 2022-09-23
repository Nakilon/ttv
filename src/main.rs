use geng::prelude::*;

mod api;
mod app;
mod font;

fn main() {
    logger::init().unwrap();

    api::refresh_token();
    let geng = Geng::new("ttv");
    let geng = &geng;
    geng::run(
        geng,
        geng::LoadingScreen::new(
            geng,
            geng::EmptyLoadingScreen,
            <app::Assets as geng::LoadAsset>::load(geng, &static_path()),
            {
                let geng = geng.clone();
                move |assets| {
                    let mut assets = assets.unwrap();
                    assets.process();
                    app::State::new(&geng, &Rc::new(assets), api::Client::new())
                }
            },
        ),
    );
}
