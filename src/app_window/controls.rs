/*
 * Copyright 2023, WiltonDB Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;

#[derive(Default)]
pub(super) struct AppWindowControls {
    layout: AppWindowLayout,

    pub(super) font_normal: nwg::Font,
    pub(super) font_small: nwg::Font,

    pub(super) icon: nwg::Icon,
    pub(super) window: nwg::Window,

    pub(super) file_menu: nwg::Menu,
    pub(super) file_connect_menu_item: nwg::MenuItem,
    pub(super) file_exit_menu_item: nwg::MenuItem,
    pub(super) help_menu: nwg::Menu,
    pub(super) help_about_menu_item: nwg::MenuItem,
    pub(super) help_website_menu_item: nwg::MenuItem,

    pub(super) filter_combo: nwg::ComboBox<String>,
    pub(super) filter_input: nwg::TextInput,
    pub(super) filter_button: nwg::Button,

    pub(super) settings_view: nwg::ListView,
    pub(super) reload_button: nwg::Button,
    pub(super) close_button: nwg::Button,
    pub(super) status_bar: nwg::StatusBar,

    pub(super) about_notice: ui::SyncNotice,
    pub(super) connect_notice: ui::SyncNotice,
    pub(super) load_settings_notice: ui::SyncNotice,
    pub(super) setting_notice: ui::SyncNotice,
}

impl ui::Controls for AppWindowControls {
    fn build(&mut self) -> Result<(), nwg::NwgError> {
        // fonts
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .small()
                .build())
            .build(&mut self.font_small)?;

        // window

        nwg::Icon::builder()
            .source_embed(Some(&nwg::EmbedResource::load(None)
                .expect("Error loading embedded resource")))
            .source_embed_id(2)
            .build(&mut self.icon)?;

        nwg::Window::builder()
            .size((640, 480))
            .icon(Some(&self.icon))
            .center(true)
            .title("WiltonDB Configuration Tool")
            .build(&mut self.window)?;

        // menu

        nwg::Menu::builder()
            .parent(&self.window)
            .text("File")
            .build(&mut self.file_menu)?;
        nwg::MenuItem::builder()
            .parent(&self.file_menu)
            .text("DB Connection")
            .build(&mut self.file_connect_menu_item)?;
        nwg::MenuItem::builder()
            .parent(&self.file_menu)
            .text("Exit")
            .build(&mut self.file_exit_menu_item)?;

        nwg::Menu::builder()
            .parent(&self.window)
            .text("Help")
            .build(&mut self.help_menu)?;
        nwg::MenuItem::builder()
            .parent(&self.help_menu)
            .text("About")
            .build(&mut self.help_about_menu_item)?;
        nwg::MenuItem::builder()
            .parent(&self.help_menu)
            .text("Website")
            .build(&mut self.help_website_menu_item)?;

        // filter panel

        nwg::ComboBox::builder()
            .parent(&self.window)
            .font(Some(&self.font_normal))
            .collection(vec!(
                "All Settings".to_string(),
                setting_groups::NETWORKING.to_string(),
                setting_groups::LOGGING.to_string(),
                setting_groups::MEMORY.to_string(),
                setting_groups::ESCAPE_HATCHES.to_string(),
            ))
            .selected_index(Some(0))
            .build(&mut self.filter_combo)?;
        nwg::TextInput::builder()
            .placeholder_text(Some("Search by setting name with '*' and '?' wildcards"))
            .parent(&self.window)
            .font(Some(&self.font_normal))
            .build(&mut self.filter_input)?;

        nwg::Button::builder()
            .parent(&self.window)
            .font(Some(&self.font_normal))
            .text("Search")
            .build(&mut self.filter_button)?;

        // settings view

        nwg::ListView::builder()
            .parent(&self.window)
            .item_count(10)
            .list_style(nwg::ListViewStyle::Detailed)
            .focus(true)
            .ex_flags(nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)
            .build(&mut self.settings_view)?;
        self.settings_view.set_headers_enabled(true);
        self.settings_view.insert_column(nwg::InsertListViewColumn{
            index: Some(0),
            fmt: Some(nwg::ListViewColumnFlags::LEFT),
            width: Some(200),
            text: Some("Name".to_string())
        });
        self.settings_view.set_column_sort_arrow(0, Some(nwg::ListViewColumnSortArrow::Down));
        self.settings_view.insert_column(nwg::InsertListViewColumn{
            index: Some(1),
            fmt: Some(nwg::ListViewColumnFlags::LEFT),
            width: Some(100),
            text: Some("Value".to_string())
        });
        self.settings_view.insert_column(nwg::InsertListViewColumn{
            index: Some(2),
            fmt: Some(nwg::ListViewColumnFlags::LEFT),
            width: Some(400),
            text: Some("Description".to_string())
        });

        // buttons

        nwg::Button::builder()
            .text("Reload settings")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.reload_button)?;
        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        // other

        nwg::StatusBar::builder()
            .parent(&self.window)
            .font(Some(&self.font_small))
            .build(&mut self.status_bar)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.about_notice)?;
        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.connect_notice)?;
        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.load_settings_notice)?;
        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.setting_notice)?;

        self.layout.build(&self)?;

        Ok(())
    }

    fn update_tab_order(&self) {
        ui::tab_order_builder()
            .control(&self.filter_combo)
            .control(&self.filter_input)
            .control(&self.filter_button)
            .control(&self.reload_button)
            .control(&self.close_button)
            .build();
    }
}
