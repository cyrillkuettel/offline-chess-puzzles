use iced::widget::{Button, Container, Checkbox, Column, Text, TextInput, Row, PickList, Scrollable};
use iced::{Element};
use iced::{alignment, Command, Alignment, Length};

use iced_aw::TabLabel;

use crate::{Message, Tab, config, styles};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    CheckPlaySound(bool),
    CheckAutoLoad(bool),
    CheckFlipBoard(bool),
    SelectPieceTheme(styles::PieceTheme),
    SelectBoardTheme(styles::Theme),
    ChangePuzzleDbLocation(String),
    ChangeSearchResultLimit(String),
    ChangeEnginePath(String),
    ChangePressed
}

#[derive(Debug, Clone)]
pub struct SettingsTab {
    pub engine_path: String,
    pub window_width: u32,
    pub window_height: u32,
    piece_theme: styles::PieceTheme,
    pub board_theme: styles::Theme,
    theme: styles::Theme,
    play_sound: bool,
    auto_load_next: bool,
    pub flip_board: bool,

    puzzle_db_location_value: String,
    search_results_limit_value: String,

    settings_status: String,
    pub saved_configs: config::OfflinePuzzlesConfig,
}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {
            engine_path: config::SETTINGS.engine_path.clone().unwrap_or_default(),
            window_width: config::SETTINGS.window_width,
            window_height: config::SETTINGS.window_width,
            piece_theme: config::SETTINGS.piece_theme,
            board_theme: config::SETTINGS.board_theme,
            theme: styles::Theme::Blue,
            play_sound: config::SETTINGS.play_sound,
            auto_load_next: config::SETTINGS.auto_load_next,
            flip_board: config::SETTINGS.flip_board,
            puzzle_db_location_value: String::from(&config::SETTINGS.puzzle_db_location),
            search_results_limit_value: config::SETTINGS.search_results_limit.to_string(),
            settings_status: String::new(),
            saved_configs: config::load_config(),
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
        match message {
            SettingsMessage::SelectPieceTheme(value) => {
                self.piece_theme = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.piece_theme, self.board_theme, self.engine_path.clone()), Message::ChangeSettings)
            }
            SettingsMessage::SelectBoardTheme(value) => {
                self.board_theme = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.piece_theme, self.theme, self.engine_path.clone()), Message::ChangeSettings)
            }
            SettingsMessage::ChangePuzzleDbLocation(value) => {
                self.puzzle_db_location_value = value;
                Command::none()
            }
            SettingsMessage::ChangeEnginePath(value) => {
                self.engine_path = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.piece_theme, self.board_theme, self.engine_path.clone()), Message::ChangeSettings)
            }
            SettingsMessage::ChangeSearchResultLimit(value) => {
                if value.is_empty() {
                    self.search_results_limit_value = String::from("0");
                } else if let Ok(new_val) = value.parse::<usize>() {
                    self.search_results_limit_value = new_val.to_string();
                    self.settings_status = String::from("");
                }
                Command::none()
            }
            SettingsMessage::CheckPlaySound(value) => {
                self.play_sound = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.piece_theme, self.board_theme, self.engine_path.clone()), Message::ChangeSettings)
            }
            SettingsMessage::CheckAutoLoad(value) => {
                self.auto_load_next = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.piece_theme, self.board_theme, self.engine_path.clone()), Message::ChangeSettings)
            }
            SettingsMessage::CheckFlipBoard(value) => {
                self.flip_board = value;
                Command::perform(SettingsTab::send_changes(self.play_sound, self.auto_load_next, self.flip_board, self.piece_theme, self.board_theme, self.engine_path.clone()), Message::ChangeSettings)
            }
            SettingsMessage::ChangePressed => {
                let engine_path = if self.engine_path.is_empty() {
                    None
                } else {
                    Some(self.engine_path.clone())
                };
                let config = config::OfflinePuzzlesConfig {
                    engine_path: engine_path,
                    engine_limit: self.saved_configs.engine_limit.clone(),
                    window_width: self.window_width,
                    window_height: self.window_height,
                    puzzle_db_location: String::from(&self.puzzle_db_location_value),
                    piece_theme: self.piece_theme,
                    search_results_limit: self.search_results_limit_value.parse().unwrap(),
                    play_sound: self.play_sound,
                    auto_load_next: self.auto_load_next,
                    flip_board: self.flip_board,
                    board_theme: self.board_theme,
                    last_min_rating: self.saved_configs.last_min_rating,
                    last_max_rating: self.saved_configs.last_max_rating,
                    last_theme: self.saved_configs.last_theme,
                    last_opening: self.saved_configs.last_opening,
                    last_opening_side: self.saved_configs.last_opening_side,
                };
                let file = std::fs::File::create("settings.json");
                match file {
                    Ok(file) => {
                        if serde_json::to_writer_pretty(file, &config).is_ok() {                
                            self.settings_status = String::from("Settings saved!");
                        } else {
                            self.settings_status = String::from("Error saving config file.");
                        }
                    } Err(_) => self.settings_status = String::from("Error reading config file.")
                }
                Command::none()
            }
        }
    }

    pub fn save_window_size(width: u32, height: u32) {
        let mut config = config::load_config();
        config.window_width = width;
        config.window_height = height;
        let file = std::fs::File::create("settings.json");
        match file {
            Ok(file) => {
                if !serde_json::to_writer_pretty(file, &config).is_ok() {
                    println!("Error saving config file.");
                }
            } Err(_) => println!("Error opening settings file")
        }
    }

    pub async fn send_changes(play_sound: bool, auto_load: bool, flip: bool, pieces: styles::PieceTheme, theme: styles::Theme, engine: String) -> Option<config::OfflinePuzzlesConfig> {
        let engine = if engine.is_empty() {
            None
        } else {
            Some(engine)
        };
        let mut config = config::load_config();
        config.board_theme = theme;
        config.piece_theme = pieces;
        config.play_sound = play_sound;
        config.auto_load_next = auto_load;
        config.flip_board = flip;
        config.engine_path = engine;
        Some(config)
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText('\u{F217}', self.title())
    }

    fn content(&self) -> Element<Message, iced::Renderer<styles::Theme>> {
        let col_settings = Column::new().spacing(10).align_items(Alignment::Center)
            .spacing(10)
            .push(
                Row::new().spacing(5).align_items(Alignment::Center)
                .push(Text::new("Piece Theme:"))
                .push(
                    PickList::new(
                        &styles::PieceTheme::ALL[..],
                        Some(self.piece_theme),
                        SettingsMessage::SelectPieceTheme
                    )
                )
            ).push(
                Row::new().spacing(5).align_items(Alignment::Center)
                .push(Text::new("Board Theme:"))
                .push(
                    PickList::new(
                        &styles::Theme::ALL[..],
                        Some(self.board_theme),
                        SettingsMessage::SelectBoardTheme
                    )
                )
            ).push(
                Row::new().spacing(5).align_items(Alignment::Center)
                    .push(Text::new("Play sound on moves:"))
                    .push(
                        Checkbox::new(
                            "",
                            self.play_sound,
                            SettingsMessage::CheckPlaySound,
                        )
                        .size(20),
                    )
            ).push(
                Row::new().spacing(5).align_items(Alignment::Center)
                    .push(Text::new("Auto load next puzzle:"))
                    .push(
                        Checkbox::new(
                            "",
                            self.auto_load_next,
                            SettingsMessage::CheckAutoLoad,
                        )
                        .size(20),
                    )
            ).push(
                Row::new().spacing(5).align_items(Alignment::Center)
                    .push(Text::new("Flip board:"))
                    .push(
                        Checkbox::new(
                            "",
                            self.flip_board,
                            SettingsMessage::CheckFlipBoard,
                        )
                        .size(20),
                    )
            ).push(
                Row::new().spacing(5).align_items(Alignment::Center)    
                    .push(Text::new("Get the first"))
                    .push(
                        TextInput::new(
                            &self.search_results_limit_value,
                            &self.search_results_limit_value,
                            SettingsMessage::ChangeSearchResultLimit,
                        )
                        .width(Length::Units(80))
                        .padding(10)
                        .size(20))
                    .push(Text::new(" puzzles")
                )
            ).push(
                Text::new("Engine path (with .exe name):")
            ).push(
                TextInput::new(
                    &self.engine_path,
                    &self.engine_path,
                    SettingsMessage::ChangeEnginePath,
                )
                .width(Length::Units(200))
                .padding(10)
                .size(20),
            ).push(
                Button::new(Text::new("Save Changes")).padding(5).on_press(SettingsMessage::ChangePressed)
            ).push(
                Text::new(&self.settings_status).vertical_alignment(alignment::Vertical::Bottom),
            );
        let content: Element<SettingsMessage, iced::Renderer<styles::Theme>> = Container::new(
            Scrollable::new(
                Column::new().spacing(10).spacing(10).push(col_settings)
            )
        ).align_x(alignment::Horizontal::Center).height(Length::Fill).width(Length::Fill).into();

        content.map(Message::Settings)
    }
}
