use collab_ui::collab_panel;
use gpui::{App, Menu, MenuItem, OsAction};
use i18n::t;
use release_channel::ReleaseChannel;
use terminal_view::terminal_panel;
use zed_actions::{ToggleFocus as ToggleDebugPanel, dev};

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    use zed_actions::Quit;

    let mut view_items = vec![
        MenuItem::action(
            t("menu.zoom_in"),
            zed_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            t("menu.zoom_out"),
            zed_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            t("menu.reset_zoom"),
            zed_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action(
            t("menu.reset_all_zoom"),
            zed_actions::ResetAllZoom { persist: false },
        ),
        MenuItem::separator(),
        MenuItem::action(t("menu.toggle_left_dock"), workspace::ToggleLeftDock),
        MenuItem::action(t("menu.toggle_right_dock"), workspace::ToggleRightDock),
        MenuItem::action(t("menu.toggle_bottom_dock"), workspace::ToggleBottomDock),
        MenuItem::action(t("menu.toggle_all_docks"), workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: t("menu.editor_layout").into(),
            items: vec![
                MenuItem::action(t("menu.split_up"), workspace::SplitUp::default()),
                MenuItem::action(t("menu.split_down"), workspace::SplitDown::default()),
                MenuItem::action(t("menu.split_left"), workspace::SplitLeft::default()),
                MenuItem::action(t("menu.split_right"), workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action(t("menu.project_panel"), zed_actions::project_panel::ToggleFocus),
        MenuItem::action(t("menu.outline_panel"), outline_panel::ToggleFocus),
        MenuItem::action(t("menu.collab_panel"), collab_panel::ToggleFocus),
        MenuItem::action(t("menu.terminal_panel"), terminal_panel::ToggleFocus),
        MenuItem::action(t("menu.debugger_panel"), ToggleDebugPanel),
        MenuItem::separator(),
        MenuItem::action(t("menu.diagnostics"), diagnostics::Deploy),
        MenuItem::separator(),
    ];

    if ReleaseChannel::try_global(cx) == Some(ReleaseChannel::Dev) {
        view_items.push(MenuItem::action(
            t("menu.toggle_gpui_inspector"),
            dev::ToggleInspector,
        ));
        view_items.push(MenuItem::separator());
    }

    vec![
        Menu {
            name: t("menu.zed").into(),
            items: vec![
                MenuItem::action(t("menu.about_zed"), zed_actions::About),
                MenuItem::action(t("menu.check_for_updates"), auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: t("menu.settings").into(),
                    items: vec![
                        MenuItem::action(t("menu.open_settings"), zed_actions::OpenSettings),
                        MenuItem::action(t("menu.open_settings_file"), super::OpenSettingsFile),
                        MenuItem::action(t("menu.open_project_settings"), zed_actions::OpenProjectSettings),
                        MenuItem::action(
                            t("menu.open_project_settings_file"),
                            super::OpenProjectSettingsFile,
                        ),
                        MenuItem::action(t("menu.open_default_settings"), super::OpenDefaultSettings),
                        MenuItem::separator(),
                        MenuItem::action(t("menu.open_keymap"), zed_actions::OpenKeymap),
                        MenuItem::action(t("menu.open_keymap_file"), zed_actions::OpenKeymapFile),
                        MenuItem::action(
                            t("menu.open_default_key_bindings"),
                            zed_actions::OpenDefaultKeymap,
                        ),
                        MenuItem::separator(),
                        MenuItem::action(
                            t("menu.select_theme"),
                            zed_actions::theme_selector::Toggle::default(),
                        ),
                        MenuItem::action(
                            t("menu.select_icon_theme"),
                            zed_actions::icon_theme_selector::Toggle::default(),
                        ),
                    ],
                }),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu(t("menu.services"), gpui::SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action(t("menu.extensions"), zed_actions::Extensions::default()),
                #[cfg(not(target_os = "windows"))]
                MenuItem::action(t("menu.install_cli"), install_cli::InstallCliBinary),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action(t("menu.hide_zed"), super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action(t("menu.hide_others"), super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action(t("menu.show_all"), super::ShowAll),
                MenuItem::separator(),
                MenuItem::action(t("menu.quit_zed"), Quit),
            ],
        },
        Menu {
            name: t("menu.file").into(),
            items: vec![
                MenuItem::action(t("menu.new"), workspace::NewFile),
                MenuItem::action(t("menu.new_window"), workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action(t("menu.open_file"), workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        t("menu.open_folder")
                    } else {
                        t("menu.open")
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    t("menu.open_recent"),
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                ),
                MenuItem::action(
                    t("menu.open_remote"),
                    zed_actions::OpenRemote {
                        create_new_window: false,
                        from_existing_connection: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(t("menu.add_folder_to_project"), workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action(t("menu.save"), workspace::Save { save_intent: None }),
                MenuItem::action(t("menu.save_as"), workspace::SaveAs),
                MenuItem::action(t("menu.save_all"), workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    t("menu.close_editor"),
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action(t("menu.close_project"), workspace::CloseProject),
                MenuItem::action(t("menu.close_window"), workspace::CloseWindow),
            ],
        },
        Menu {
            name: t("menu.edit").into(),
            items: vec![
                MenuItem::os_action(t("menu.undo"), editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action(t("menu.redo"), editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action(t("menu.cut"), editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action(t("menu.copy"), editor::actions::Copy, OsAction::Copy),
                MenuItem::action(t("menu.copy_and_trim"), editor::actions::CopyAndTrim),
                MenuItem::os_action(t("menu.paste"), editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action(t("menu.find"), search::buffer_search::Deploy::find()),
                MenuItem::action(t("menu.find_in_project"), workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    t("menu.toggle_line_comment"),
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: t("menu.selection").into(),
            items: vec![
                MenuItem::os_action(
                    t("menu.select_all"),
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action(t("menu.expand_selection"), editor::actions::SelectLargerSyntaxNode),
                MenuItem::action(t("menu.shrink_selection"), editor::actions::SelectSmallerSyntaxNode),
                MenuItem::action(t("menu.select_next_sibling"), editor::actions::SelectNextSyntaxNode),
                MenuItem::action(
                    t("menu.select_previous_sibling"),
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t("menu.add_cursor_above"),
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    t("menu.add_cursor_below"),
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    t("menu.select_next_occurrence"),
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    t("menu.select_previous_occurrence"),
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(t("menu.select_all_occurrences"), editor::actions::SelectAllMatches),
                MenuItem::separator(),
                MenuItem::action(t("menu.move_line_up"), editor::actions::MoveLineUp),
                MenuItem::action(t("menu.move_line_down"), editor::actions::MoveLineDown),
                MenuItem::action(t("menu.duplicate_selection"), editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: t("menu.view").into(),
            items: view_items,
        },
        Menu {
            name: t("menu.go").into(),
            items: vec![
                MenuItem::action(t("menu.back"), workspace::GoBack),
                MenuItem::action(t("menu.forward"), workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action(t("menu.command_palette"), zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action(t("menu.go_to_file"), workspace::ToggleFileFinder::default()),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action(
                    t("menu.go_to_symbol_in_editor"),
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action(t("menu.go_to_line_column"), editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action(t("menu.go_to_definition"), editor::actions::GoToDefinition),
                MenuItem::action(t("menu.go_to_declaration"), editor::actions::GoToDeclaration),
                MenuItem::action(t("menu.go_to_type_definition"), editor::actions::GoToTypeDefinition),
                MenuItem::action(
                    t("menu.find_all_references"),
                    editor::actions::FindAllReferences::default(),
                ),
                MenuItem::separator(),
                MenuItem::action(t("menu.next_problem"), editor::actions::GoToDiagnostic::default()),
                MenuItem::action(
                    t("menu.previous_problem"),
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: t("menu.run").into(),
            items: vec![
                MenuItem::action(
                    t("menu.spawn_task"),
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action(t("menu.start_debugger"), debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action(t("menu.edit_tasks_json"), crate::zed::OpenProjectTasks),
                MenuItem::action(t("menu.edit_debug_json"), zed_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action(t("menu.continue"), debugger_ui::Continue),
                MenuItem::action(t("menu.step_over"), debugger_ui::StepOver),
                MenuItem::action(t("menu.step_into"), debugger_ui::StepInto),
                MenuItem::action(t("menu.step_out"), debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action(t("menu.toggle_breakpoint"), editor::actions::ToggleBreakpoint),
                MenuItem::action(t("menu.edit_breakpoint"), editor::actions::EditLogBreakpoint),
                MenuItem::action(t("menu.clear_all_breakpoints"), debugger_ui::ClearAllBreakpoints),
            ],
        },
        Menu {
            name: t("menu.window").into(),
            items: vec![
                MenuItem::action(t("menu.minimize"), super::Minimize),
                MenuItem::action(t("menu.zoom"), super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: t("menu.help").into(),
            items: vec![
                MenuItem::action(
                    t("menu.view_release_notes_locally"),
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action(t("menu.view_telemetry"), zed_actions::OpenTelemetryLog),
                MenuItem::action(t("menu.view_dependency_licenses"), zed_actions::OpenLicenses),
                MenuItem::action(t("menu.show_welcome"), onboarding::ShowWelcome),
                MenuItem::separator(),
                MenuItem::action(t("menu.file_bug_report"), zed_actions::feedback::FileBugReport),
                MenuItem::action(t("menu.request_feature"), zed_actions::feedback::RequestFeature),
                MenuItem::action(t("menu.email_us"), zed_actions::feedback::EmailZed),
                MenuItem::separator(),
                MenuItem::action(
                    t("menu.documentation"),
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(t("menu.zed_repository"), feedback::OpenZedRepo),
                MenuItem::action(
                    t("menu.zed_twitter"),
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    t("menu.join_the_team"),
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
