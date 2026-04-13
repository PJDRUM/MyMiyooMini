use std::collections::VecDeque;
use std::fs::{self, File};
use std::marker::PhantomData;

use anyhow::Result;
use async_trait::async_trait;
use common::battery::Battery;
use common::command::Command;
use common::constants::ALLIUM_MENU_STATE;
use common::display::Display;
use common::game_info::GameInfo;
use common::geom::{Alignment, Point, Rect};
use common::locale::Locale;
use common::platform::{DefaultPlatform, Key, KeyEvent, Platform};
use common::resources::Resources;
use common::retroarch::RetroArchCommand;
use common::stylesheet::Stylesheet;
use common::view::{ButtonHint, ButtonHints, NullView, SettingsList, StatusBar, View};
use log::warn;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::retroarch_info::RetroArchInfo;

#[derive(Serialize, Deserialize, Default)]
pub struct IngameMenuState {}

pub struct IngameMenu<B>
where
    B: Battery + 'static,
{
    rect: Rect,
    name: common::view::Label<String>,
    status_bar: StatusBar<B>,
    menu: SettingsList,
    button_hints: ButtonHints<String>,
    entries: Vec<MenuEntry>,
    has_retroarch: bool,
    _phantom_battery: PhantomData<B>,
}

impl<B> IngameMenu<B>
where
    B: Battery + 'static,
{
    pub fn new(
        rect: Rect,
        _state: IngameMenuState,
        res: Resources,
        battery: B,
        retroarch_info: Option<RetroArchInfo>,
    ) -> Self {
        let Rect { x, y, w, .. } = rect;

        let game_info = res.get::<GameInfo>();
        let locale = res.get::<Locale>();
        let styles = res.get::<Stylesheet>();

        let name = common::view::Label::new(
            Point::new(x + styles.ui.margin_x, y + styles.ui.margin_y),
            game_info.name.clone(),
            Alignment::Left,
            None,
        );

        let mut status_bar = StatusBar::new(
            res.clone(),
            Point::new(w as i32 - styles.ui.margin_y, y + styles.ui.margin_y),
            battery,
        );

        let mut button_hints = ButtonHints::new(
            res.clone(),
            vec![ButtonHint::new(
                res.clone(),
                Point::zero(),
                Key::Menu,
                locale.t("ingame-menu-continue"),
                Alignment::Left,
            )],
            vec![
                ButtonHint::new(
                    res.clone(),
                    Point::zero(),
                    Key::A,
                    locale.t("button-select"),
                    Alignment::Right,
                ),
                ButtonHint::new(
                    res.clone(),
                    Point::zero(),
                    Key::B,
                    locale.t("button-back"),
                    Alignment::Right,
                ),
            ],
        );

        let status_bar_rect = status_bar.bounding_box(&styles);
        let button_hints_rect = button_hints.bounding_box(&styles);
        let content_top = y
            + styles.ui.margin_y
            + styles.ui.ui_font.size.max(status_bar_rect.h) as i32
            + styles.ui.margin_y / 2;
        let content_height = (button_hints_rect.y - content_top) as u32;

        let entries = MenuEntry::entries();
        let menu = SettingsList::new(
            res.clone(),
            Rect::new(
                x + styles.ui.margin_x,
                content_top,
                w - styles.ui.margin_x as u32 * 2,
                content_height,
            ),
            entries.iter().map(|e| e.as_str(&locale)).collect(),
            entries
                .iter()
                .map(|_| Box::new(NullView) as Box<dyn View>)
                .collect(),
            styles.ui.ui_font.size + styles.ui.padding_y as u32,
        );

        drop(game_info);
        drop(locale);
        drop(styles);

        Self {
            rect,
            name,
            status_bar,
            menu,
            button_hints,
            entries,
            has_retroarch: retroarch_info.is_some(),
            _phantom_battery: PhantomData,
        }
    }

    pub async fn load_or_new(
        rect: Rect,
        res: Resources,
        battery: B,
        info: Option<RetroArchInfo>,
    ) -> Result<Self> {
        if ALLIUM_MENU_STATE.exists() {
            let file = File::open(ALLIUM_MENU_STATE.as_path())?;
            if let Ok(state) = serde_json::from_reader::<_, IngameMenuState>(file) {
                return Ok(Self::new(rect, state, res, battery, info));
            }
            warn!("failed to deserialize state file, deleting");
            fs::remove_file(ALLIUM_MENU_STATE.as_path())?;
        }

        Ok(Self::new(rect, Default::default(), res, battery, info))
    }

    pub fn save(&self) -> Result<()> {
        let file = File::create(ALLIUM_MENU_STATE.as_path())?;
        serde_json::to_writer(file, &IngameMenuState::default())?;
        Ok(())
    }

    async fn select_entry(&mut self, commands: Sender<Command>) -> Result<bool> {
        match self.entries[self.menu.selected()] {
            MenuEntry::Continue => {
                commands.send(Command::Exit).await?;
            }
            MenuEntry::Quit => {
                if self.has_retroarch {
                    commands
                        .send(Command::RetroArchCommand(RetroArchCommand::Quit))
                        .await?;
                } else {
                    tokio::process::Command::new("pkill")
                        .arg("retroarch")
                        .spawn()?
                        .wait()
                        .await?;
                }
                commands.send(Command::Exit).await?;
            }
        }
        Ok(true)
    }
}

#[async_trait(?Send)]
impl<B> View for IngameMenu<B>
where
    B: Battery,
{
    fn draw(
        &mut self,
        display: &mut <DefaultPlatform as Platform>::Display,
        styles: &Stylesheet,
    ) -> Result<bool> {
        let mut drawn = false;

        drawn |= self.name.should_draw() && self.name.draw(display, styles)?;
        drawn |= self.status_bar.should_draw() && self.status_bar.draw(display, styles)?;
        drawn |= self.menu.should_draw() && self.menu.draw(display, styles)?;
        if self.button_hints.should_draw() {
            display.load(self.button_hints.bounding_box(styles))?;
            drawn |= self.button_hints.draw(display, styles)?;
        }

        #[cfg(feature = "debug-ui")]
        if drawn {
            common::view::draw_debug_bounds(self, display, styles, 0)?;
        }

        Ok(drawn)
    }

    fn should_draw(&self) -> bool {
        self.name.should_draw()
            || self.status_bar.should_draw()
            || self.menu.should_draw()
            || self.button_hints.should_draw()
    }

    fn set_should_draw(&mut self) {
        self.name.set_should_draw();
        self.status_bar.set_should_draw();
        self.menu.set_should_draw();
        self.button_hints.set_should_draw();
    }

    async fn handle_key_event(
        &mut self,
        event: KeyEvent,
        commands: Sender<Command>,
        bubble: &mut VecDeque<Command>,
    ) -> Result<bool> {
        match event {
            KeyEvent::Pressed(Key::Menu | Key::B) => {
                commands.send(Command::Exit).await?;
                Ok(true)
            }
            KeyEvent::Pressed(Key::A) => self.select_entry(commands).await,
            KeyEvent::Pressed(Key::Left | Key::Right)
            | KeyEvent::Autorepeat(Key::Left | Key::Right) => Ok(true),
            _ => self.menu.handle_key_event(event, commands, bubble).await,
        }
    }

    fn children(&self) -> Vec<&dyn View> {
        vec![&self.name, &self.status_bar, &self.menu, &self.button_hints]
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        vec![
            &mut self.name,
            &mut self.status_bar,
            &mut self.menu,
            &mut self.button_hints,
        ]
    }

    fn bounding_box(&mut self, _styles: &Stylesheet) -> Rect {
        self.rect
    }

    fn set_position(&mut self, point: Point) {
        self.rect.x = point.x;
        self.rect.y = point.y;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MenuEntry {
    Continue,
    Quit,
}

impl MenuEntry {
    fn as_str(&self, locale: &Locale) -> String {
        match self {
            MenuEntry::Continue => locale.t("ingame-menu-continue"),
            MenuEntry::Quit => locale.t("ingame-menu-quit"),
        }
    }

    fn entries() -> Vec<Self> {
        vec![MenuEntry::Continue, MenuEntry::Quit]
    }
}
