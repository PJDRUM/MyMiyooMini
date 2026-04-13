use std::collections::VecDeque;
use std::fs::{self, File};
use std::marker::PhantomData;

use anyhow::Result;
use async_trait::async_trait;
use common::battery::Battery;
use common::command::Command;
use common::constants::ALLIUM_LAUNCHER_STATE;
use common::display::Display;
use common::geom::{Point, Rect};
use common::platform::{DefaultPlatform, Platform};
use common::resources::Resources;
use common::stylesheet::Stylesheet;
use common::view::{StatusBar, View};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::view::games::GamesState;
use crate::view::Games;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppState {
    games: GamesState,
}

#[derive(Debug)]
pub struct App<B>
where
    B: Battery + 'static,
{
    rect: Rect,
    res: Resources,
    status_bar: StatusBar<B>,
    games: Games,
    dirty: bool,
    _phantom_battery: PhantomData<B>,
}

impl<B> App<B>
where
    B: Battery + 'static,
{
    pub fn new(
        rect: Rect,
        res: Resources,
        games: Games,
        battery: B,
    ) -> Result<Self> {
        let Rect { y, w, .. } = rect;

        let status_bar = StatusBar::new(
            res.clone(),
            Point::new(
                w as i32 - res.get::<Stylesheet>().ui.margin_x,
                y + res.get::<Stylesheet>().ui.margin_y,
            ),
            battery,
        );

        Ok(Self {
            rect,
            res: res.clone(),
            games,
            status_bar,
            dirty: true,
            _phantom_battery: PhantomData,
        })
    }

    pub fn load_or_new(rect: Rect, res: Resources, battery: B) -> Result<Self> {
        let tab_rect = {
            let styles = res.get::<Stylesheet>();
            let font_size = styles.tab_font_size().max(styles.status_bar_font_size()) as u32;
            Rect::new(
                rect.x,
                rect.y + font_size as i32 + styles.ui.margin_y + styles.ui.margin_y / 2,
                rect.w,
                rect.h - font_size - styles.ui.margin_y as u32 - styles.ui.margin_y as u32 / 2,
            )
        };

        if ALLIUM_LAUNCHER_STATE.exists() {
            let file = File::open(ALLIUM_LAUNCHER_STATE.as_path())?;
            if let Ok(state) = serde_json::from_reader::<_, AppState>(file) {
                let games = Games::load_or_new(tab_rect, res.clone(), Some(state.games))
                    .unwrap_or_else(|_| Games::load_or_new(tab_rect, res.clone(), None).unwrap());
                return Self::new(rect, res, games, battery);
            }
            warn!("failed to deserialize state file, deleting");
            fs::remove_file(ALLIUM_LAUNCHER_STATE.as_path())?;
        }

        let games = Games::load_or_new(tab_rect, res.clone(), None)?;
        Self::new(rect, res, games, battery)
    }

    pub fn save(&self) -> Result<()> {
        let file = File::create(ALLIUM_LAUNCHER_STATE.as_path())?;
        let state = AppState { games: self.games.save() };
        serde_json::to_writer(file, &state)?;
        Ok(())
    }

    fn view(&self) -> &dyn View {
        &self.games
    }

    fn view_mut(&mut self) -> &mut dyn View {
        &mut self.games
    }
}

#[async_trait(?Send)]
impl<B> View for App<B>
where
    B: Battery,
{
    fn draw(
        &mut self,
        display: &mut <DefaultPlatform as Platform>::Display,
        styles: &Stylesheet,
    ) -> Result<bool> {
        if self.dirty {
            display.load(self.bounding_box(styles))?;
            self.dirty = false;
        }

        #[cfg(feature = "debug-ui-redraw")]
        {
            let bg_color = StylesheetColor::Background.to_color(styles);
            let full_rect =
                common::geom::Rect::new(0, 0, display.size().width, display.size().height);
            common::display::fill_rect(&mut display.pixmap_mut(), full_rect, bg_color);
        }

        let mut drawn = false;

        if self.status_bar.should_draw() {
            display.load(self.status_bar.bounding_box(styles))?;
            drawn |= self.status_bar.draw(display, styles)?;
        }
        drawn |= self.view().should_draw() && self.view_mut().draw(display, styles)?;

        #[cfg(feature = "debug-ui")]
        common::view::draw_debug_bounds(self, display, styles, 0)?;

        Ok(drawn)
    }

    fn should_draw(&self) -> bool {
        self.status_bar.should_draw() || self.view().should_draw()
    }

    fn set_should_draw(&mut self) {
        self.dirty = true;
        self.status_bar.set_should_draw();
        self.view_mut().set_should_draw();
    }

    async fn handle_key_event(
        &mut self,
        event: common::platform::KeyEvent,
        commands: tokio::sync::mpsc::Sender<Command>,
        bubble: &mut VecDeque<Command>,
    ) -> Result<bool> {
        self.view_mut().handle_key_event(event, commands, bubble).await
    }

    fn children(&self) -> Vec<&dyn View> {
        vec![&self.status_bar, self.view()]
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        let view: &mut dyn View = &mut self.games;
        vec![&mut self.status_bar, view]
    }

    fn bounding_box(&mut self, _styles: &Stylesheet) -> Rect {
        self.rect
    }

    fn set_position(&mut self, _point: Point) {
        unimplemented!()
    }
}
