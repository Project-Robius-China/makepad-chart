use crate::config::{ChartConfig, ChartType, DashboardConfig, YColumnConfig, load_all_dashboards, save_dashboard};
use crate::data::{DataTable, parse_csv};
use crate::transform::table_to_combo_chart_data;
use crate::ui::chart_list::ChartListWidgetRefExt;
use crate::ui::{self, AppScreen};
use makepad_components::flexible_data_table::{FlexibleDataTableWidgetRefExt, LiveHiddenCell};
use makepad_components::theme::{apply_theme, ThemeMode};
use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::combo_chart::DatasetType;
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::ui::chart_cell::ChartCell;
    use crate::ui::chart_list::ChartList;
    use makepad_components::flexible_data_table::FlexibleDataTable;
    //use crate::ui::data_table::DataTable;

    COLOR_BG = #1a1a1a
    COLOR_PANEL = #2a2a2a
    COLOR_BORDER = #3a3a3a
    COLOR_TEXT = #e0e0e0
    COLOR_PRIMARY = #4a90e2
    COLOR_HOVER = #5aa0f2

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(1400, 900)},
            body = <View> {
                width: Fill, height: Fill
                flow: Down
                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return (COLOR_BG);
                    }
                }

                // Header
                <View> {
                    width: Fill, height: 60
                    flow: Right, align: {x: 0.5, y: 0.5}
                    padding: {left: 20, right: 20}
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return (COLOR_PANEL);
                        }
                    }

                    <Label> {
                        text: "Business Intelligence Dashboard"
                        draw_text: {
                            text_style: {font_size: 20.0},
                            color: (COLOR_TEXT)
                        }
                    }
                }

                // Main content area with overlay for screen switching
                <View> {
                    width: Fill, height: Fill
                    flow: Overlay

                    // Home Screen
                    home_screen = <View> {
                        visible: true
                        width: Fill, height: Fill
                        flow: Down
                        padding: 20

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10
                            padding: {bottom: 20}

                            new_dashboard_btn = <Button> {
                                text: "New Dashboard"
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                                    }
                                }
                            }

                            import_csv_btn = <Button> {
                                text: "Import CSV"
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                                    }
                                }
                            }
                        }

                        <Label> {
                            text: "Your Dashboards"
                            draw_text: {
                                text_style: {font_size: 18.0},
                                color: (COLOR_TEXT)
                            }
                            padding: {bottom: 10}
                        }

                        dashboard_list = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 10

                            dashboard_btn_0 = <Button> {
                                visible: false
                                width: Fill, height: 50
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PANEL), (COLOR_HOVER), self.hover);
                                    }
                                }
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                            }

                            dashboard_btn_1 = <Button> {
                                visible: false
                                width: Fill, height: 50
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PANEL), (COLOR_HOVER), self.hover);
                                    }
                                }
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                            }

                            dashboard_btn_2 = <Button> {
                                visible: false
                                width: Fill, height: 50
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PANEL), (COLOR_HOVER), self.hover);
                                    }
                                }
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                            }

                            dashboard_btn_3 = <Button> {
                                visible: false
                                width: Fill, height: 50
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PANEL), (COLOR_HOVER), self.hover);
                                    }
                                }
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                            }
                        }

                        no_dashboards_label = <Label> {
                            text: "No saved dashboards. Create a new one or import CSV data first."
                            draw_text: {
                                text_style: {font_size: 12.0},
                                color: #888
                            }
                        }
                    }

                    // Import CSV Screen
                    import_screen = <View> {
                        visible: false
                        width: Fill, height: Fill
                        flow: Down
                        padding: 20

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10
                            padding: {bottom: 20}

                            back_from_import_btn = <Button> {
                                text: "Back"
                            }
                        }

                        <Label> {
                            text: "Import CSV File"
                            draw_text: {
                                text_style: {font_size: 18.0},
                                color: (COLOR_TEXT)
                            }
                            padding: {bottom: 10}
                        }

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10
                            padding: {bottom: 20}

                            csv_path_input = <TextInput> {
                                width: 400, height: 30
                                text: "sample_data/sales.csv"
                            }

                            load_csv_btn = <Button> {
                                text: "Load CSV"
                            }
                        }

                        import_status = <Label> {
                            text: ""
                            draw_text: {
                                text_style: {font_size: 12.0},
                                color: (COLOR_TEXT)
                            }
                            padding: {bottom: 10}
                        }

                        csv_preview = <ScrollYView> {
                            width: Fill, height: Fill
                            <View> {
                                width: Fill, height: Fit
                                flow: Down
                            }
                        }
                    }

                    // Dashboard Screen
                    dashboard_screen = <View> {
                        visible: false
                        width: Fill, height: Fill
                        flow: Down
                        padding: 20

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10
                            padding: {bottom: 20}

                            back_from_dashboard_btn = <Button> {
                                text: "Back to Home"
                            }

                            add_chart_btn = <Button> {
                                text: "Add Chart"
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                                    }
                                }
                            }

                            save_dashboard_btn = <Button> {
                                text: "Save Dashboard"
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                                    }
                                }
                            }
                        }

                        dashboard_name = <Label> {
                            text: "Dashboard"
                            draw_text: {
                                text_style: {font_size: 18.0},
                                color: (COLOR_TEXT)
                            }
                            padding: {bottom: 10}
                        }
                        chart_list = <ChartList> {
                            width: Fill, height: Fill
                        }
                        // Scrollable Chart Grid - Flow Right
                        // <ScrollXYView> {
                        //     width: Fill, height: Fill
                        //     debug: true,
                            
                        //     // chart_grid = <View> {
                        //     //     width: Fit, height: Fill
                        //     //     flow: Right
                        //     //     spacing: 20
                        //     //     padding: 10

                        //     //     chart_0_0 = <ChartCell> {
                        //     //         width: 400, height: Fill
                        //     //     }

                        //     //     chart_0_1 = <ChartCell> {
                        //     //         width: 400, height: Fill
                        //     //     }

                        //     //     // Add Chart button at the end
                        //     //     add_chart_inline_btn = <Button> {
                        //     //         width: 100, height: Fill
                        //     //         text: "+"
                        //     //         draw_text: {
                        //     //             text_style: {font_size: 32.0}
                        //     //         }
                        //     //         draw_bg: {
                        //     //             fn pixel(self) -> vec4 {
                        //     //                 return mix((COLOR_PANEL), (COLOR_HOVER), self.hover);
                        //     //             }
                        //     //         }
                        //     //     }
                        //     // }
                        // }
                    }

                    // Chart Configuration Screen
                    chart_config_screen = <View> {
                        visible: false
                        width: Fill, height: Fill
                        flow: Down
                        padding: 20

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10
                            padding: {bottom: 20}

                            back_from_config_btn = <Button> {
                                text: "Cancel"
                            }

                            apply_config_btn = <Button> {
                                text: "Apply"
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                                    }
                                }
                            }
                        }
                        <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 15
                            padding: {bottom: 20}

                            <Label> {
                                text: "Configure Chart"
                                draw_text: {
                                    text_style: {font_size: 18.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {bottom: 10}
                            }
                            <View> {
                                flow: Right
                                <Label> {
                                    text: "Chart Title"
                                    draw_text: {
                                        text_style: {font_size: 14.0},
                                        color: (COLOR_TEXT)
                                    }
                                }

                                chart_title_input = <TextInput> {
                                    width: 400, height: 30
                                    text: "My Chart"
                                }
                            }

                            <Label> {
                                text: "Data Columns (1st row = X-axis, additional rows = Y-axis)"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            column_config_table = <FlexibleDataTable> {
                                width: Fill, height: Fit
                            }

                            config_status = <Label> {
                                text: ""
                                draw_text: {
                                    text_style: {font_size: 12.0},
                                    color: #ff6666
                                }
                                padding: {top: 10}
                            }
                        }
                    }

                    // Right Drawer for chart quick actions
                    chart_drawer = <View> {
                        visible: false
                        width: Fill, height: Fill
                        flow: Overlay

                        // Semi-transparent backdrop
                        drawer_backdrop = <View> {
                            width: Fill, height: Fill
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.0, 0.0, 0.0, 0.5);
                                }
                            }
                        }

                        // Drawer panel on the right
                        <View> {
                            width: Fill, height: Fill
                            flow: Right
                            align: { x: 1.0 }

                            drawer_panel = <View> {
                                width: 280, height: Fill
                                flow: Down
                                padding: 20
                                spacing: 10
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return (COLOR_PANEL);
                                    }
                                }

                                <Label> {
                                    text: "Chart Options"
                                    draw_text: {
                                        text_style: {font_size: 16.0},
                                        color: (COLOR_TEXT)
                                    }
                                    padding: {bottom: 10}
                                }

                                drawer_configure_btn = <Button> {
                                    width: Fill, height: 40
                                    text: "Configure Chart"
                                    draw_bg: {
                                        fn pixel(self) -> vec4 {
                                            return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                                        }
                                    }
                                }

                                drawer_move_left_btn = <Button> {
                                    width: Fill, height: 40
                                    text: "Move Left"
                                }

                                drawer_move_right_btn = <Button> {
                                    width: Fill, height: 40
                                    text: "Move Right"
                                }

                                drawer_delete_btn = <Button> {
                                    width: Fill, height: 40
                                    text: "Delete Chart"
                                    draw_bg: {
                                        fn pixel(self) -> vec4 {
                                            return mix(#cc4444, #ff6666, self.hover);
                                        }
                                    }
                                }

                                <View> { width: Fill, height: Fill }

                                drawer_close_btn = <Button> {
                                    width: Fill, height: 40
                                    text: "Close"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    current_screen: AppScreen,

    #[rust]
    data_tables: Vec<DataTable>,

    #[rust]
    dashboards: Vec<DashboardConfig>,

    #[rust]
    current_dashboard: Option<DashboardConfig>,

    #[rust]
    #[allow(dead_code)]
    current_dashboard_index: Option<usize>,
    #[rust]
    chart_index_to_change: Option<usize>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_components::live_design(cx);
        makepad_charts::live_design(cx);
        ui::live_design(cx);
        // Initialize theme colors link for makepad_components
        apply_theme(cx, ThemeMode::Dark);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.match_event(cx, event);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Home screen actions
        if self.ui.button(ids!(new_dashboard_btn)).clicked(actions) {
            let dashboard = DashboardConfig::new("New Dashboard".to_string());
            self.current_dashboard = Some(dashboard);
            self.navigate_to(cx, AppScreen::DashboardInit);
        }

        if self.ui.button(ids!(import_csv_btn)).clicked(actions) {
            self.navigate_to(cx, AppScreen::Import);
        }

        // Dashboard list item clicks
        let dashboard_btn_ids = [
            ids!(dashboard_btn_0),
            ids!(dashboard_btn_1),
            ids!(dashboard_btn_2),
            ids!(dashboard_btn_3),
        ];

        for (i, btn_id) in dashboard_btn_ids.iter().enumerate() {
            if self.ui.button(*btn_id).clicked(actions) {
                if i < self.dashboards.len() {
                    self.current_dashboard = Some(self.dashboards[i].clone());
                    self.current_dashboard_index = Some(i);
                    self.navigate_to(cx, AppScreen::DashboardInit);
                }
                break;
            }
        }

        // Import screen actions
        if self.ui.button(ids!(back_from_import_btn)).clicked(actions) {
            self.navigate_to(cx, AppScreen::Home);
        }

        if self.ui.button(ids!(load_csv_btn)).clicked(actions) {
            self.load_csv_file(cx);
        }

        // Dashboard screen actions
        if self.ui.button(ids!(back_from_dashboard_btn)).clicked(actions) {
            self.navigate_to(cx, AppScreen::Home);
        }

        if self.ui.button(ids!(add_chart_btn)).clicked(actions)
            || self.ui.button(ids!(add_chart_inline_btn)).clicked(actions) {
            if self.data_tables.is_empty() {
                self.ui.label(ids!(config_status))
                    .set_text(cx, "Please import CSV data first");
            } else {
                self.navigate_to(cx, AppScreen::ChartConfig(None));
            }
        }

        if self.ui.button(ids!(save_dashboard_btn)).clicked(actions) {
            self.save_current_dashboard(cx);
        }

        // Chart config screen actions
        if self.ui.button(ids!(back_from_config_btn)).clicked(actions) {
            self.navigate_to(cx, AppScreen::DashboardInit);
        }

        if self.ui.button(ids!(apply_config_btn)).clicked(actions) {
            if let Some(chart_index_to_change) = self.chart_index_to_change {
                self.apply_chart_config(cx, chart_index_to_change);
            }
            
        }

        // Handle FlexibleDataTable Add Row button click (row is added internally by the widget)
        // We can use this hook if we need to do something when a row is added
        let _ = self.ui.flexible_data_table(ids!(column_config_table)).add_row_clicked(actions);

        // Handle ChartList add chart request (from + button)
        if self.ui.chart_list(ids!(chart_list)).add_requested(actions) {
            if self.data_tables.is_empty() {
                self.ui.label(ids!(config_status))
                    .set_text(cx, "Please import CSV data first");
            } else {
                self.navigate_to(cx, AppScreen::ChartConfig(None));
            }
        }

        //Handle ChartList configure request (from MpEditableList drawer)
        if let Some(idx_opt) = self.ui.chart_list(ids!(chart_list)).configure_requested(actions) {
            if let Some(idx) = idx_opt {
                // Configure existing chart - get the actual position of the chart at this Vec index
                if let Some(dashboard) = &self.current_dashboard {
                    if let Some(chart) = dashboard.charts.get(idx) {
                        // self.config_target_position = Some(chart.position.clone());
                        self.chart_index_to_change = Some(idx);
                        self.navigate_to(cx, AppScreen::ChartConfig(Some(chart.clone())));
                    }
                }
            }
        }

        // // Handle ChartList move left request
        // if let Some(idx) = self.ui.chart_list(ids!(chart_list)).move_left_requested(actions) {
        //     if idx > 0 {
        //         self.swap_charts(cx, idx, idx - 1);
        //     }
        // }

        // // Handle ChartList move right request
        // if let Some(idx) = self.ui.chart_list(ids!(chart_list)).move_right_requested(actions) {
        //     self.swap_charts(cx, idx, idx + 1);
        // }

        // // Handle ChartList delete request
        // if let Some(idx) = self.ui.chart_list(ids!(chart_list)).delete_requested(actions) {
        //     self.delete_chart(cx, idx);
        // }
    }
}

impl App {
    fn navigate_to(&mut self, cx: &mut Cx, screen: AppScreen) {
        // Hide all screens
        self.ui.view(ids!(home_screen)).set_visible(cx, false);
        self.ui.view(ids!(import_screen)).set_visible(cx, false);
        self.ui.view(ids!(dashboard_screen)).set_visible(cx, false);
        self.ui.view(ids!(chart_config_screen)).set_visible(cx, false);

        // Show selected screen
        match &screen {
            AppScreen::Home => {
                self.ui.view(ids!(home_screen)).set_visible(cx, true);
                self.load_dashboard_list(cx);
            }
            AppScreen::Import => {
                self.ui.view(ids!(import_screen)).set_visible(cx, true);
            }
            AppScreen::DashboardInit => {
                self.ui.view(ids!(dashboard_screen)).set_visible(cx, true);
                self.render_dashboard(cx);
            }
            AppScreen::DashboardUpdate => {
                self.ui.view(ids!(dashboard_screen)).set_visible(cx, true);
            }
            AppScreen::ChartConfig(chart_config) => {
                self.ui.view(ids!(chart_config_screen)).set_visible(cx, true);
                self.setup_chart_config(cx, chart_config.clone());
            }
        }

        self.current_screen = screen;
        self.ui.redraw(cx);
    }

    fn load_csv_file(&mut self, cx: &mut Cx) {
        let path = self.ui.text_input(ids!(csv_path_input)).text();

        match parse_csv(&path) {
            Ok(table) => {
                let status = format!(
                    "Loaded: {} ({} rows, {} columns)",
                    table.name,
                    table.row_count(),
                    table.column_count()
                );
                self.ui.label(ids!(import_status)).set_text(cx, &status);

                // Add to data tables if not already present
                if !self.data_tables.iter().any(|t| t.name == table.name) {
                    self.data_tables.push(table);
                }

                self.ui.redraw(cx);
            }
            Err(e) => {
                let error = format!("Error: {}", e);
                self.ui.label(ids!(import_status)).set_text(cx, &error);
                self.ui.redraw(cx);
            }
        }
    }

    fn load_dashboard_list(&mut self, cx: &mut Cx) {
        match load_all_dashboards() {
            Ok(dashboards) => {
                self.dashboards = dashboards;
            }
            Err(e) => {
                eprintln!("Failed to load dashboards: {}", e);
                self.dashboards = Vec::new();
            }
        }

        // Show/hide the "no dashboards" label
        self.ui.label(ids!(no_dashboards_label)).set_visible(cx, self.dashboards.is_empty());

        // Update dashboard buttons
        let btn_ids = [
            ids!(dashboard_btn_0),
            ids!(dashboard_btn_1),
            ids!(dashboard_btn_2),
            ids!(dashboard_btn_3),
        ];

        for (i, btn_id) in btn_ids.iter().enumerate() {
            let btn = self.ui.button(*btn_id);
            if i < self.dashboards.len() {
                btn.set_text(cx, &self.dashboards[i].name);
                btn.set_visible(cx, true);
            } else {
                btn.set_visible(cx, false);
            }
        }
    }

    fn render_dashboard(&mut self, cx: &mut Cx) {
        if let Some(dashboard) = self.current_dashboard.clone() {
            self.ui
                .label(ids!(dashboard_name))
                .set_text(cx, &dashboard.name);

            // Render each configured chart
            for (idx, chart_config) in dashboard.charts.iter().enumerate() {
                self.render_chart(cx, idx, chart_config);
            }
        }

        self.ui.redraw(cx);
    }

    fn render_chart(&mut self, cx: &mut Cx, idx: usize, config: &ChartConfig) {
        // Find the data table
        let table = match self.data_tables.iter().find(|t| t.name == config.data_table_name) {
            Some(t) => t,
            None => {
                eprintln!("Data table '{}' not found", config.data_table_name);
                return;
            }
        };

        // Convert to chart data
        let chart_data = match table_to_combo_chart_data(table, &config.x_column, &config.y_columns) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to convert table to chart data: {}", e);
                return;
            }
        };
        // Extract chart types from y_columns
        let chart_types: Vec<DatasetType> = config.y_columns.iter().map(|y| {
            match y.chart_type {
                ChartType::Bar => DatasetType::Bar,
                ChartType::Line => DatasetType::Line,
            }
        }).collect();

        // Create chart options
        let options = ChartOptions::new()
            .with_title(&config.title)
            .with_animation_duration(500.0);

        self.ui.chart_list(ids!(chart_list)).set_data(cx, idx, (chart_data, options, chart_types));
    }

    fn setup_chart_config(&mut self, cx: &mut Cx, chart_config: Option<ChartConfig>) {
        // Clear the DataTable first
        self.ui.flexible_data_table(ids!(column_config_table)).clear(cx);

        if let Some(table) = self.data_tables.first() {
            let data_source_text = format!("{} ({} columns)", table.name, table.column_count());
            self.ui
                .label(ids!(data_source_label))
                .set_text(cx, &data_source_text);

            // Set up column configuration for the FlexibleDataTable
            use makepad_components::flexible_data_table::{LiveColumnConfig, CellValue};
            let columns = vec![
                LiveColumnConfig::dropdown("Column", table.columns.clone(), 180.0),
                LiveColumnConfig::text_input("Label", 120.0),
                LiveColumnConfig::dropdown("Type", vec!["Bar".to_string(), "Line".to_string()], 80.0),
                LiveColumnConfig::color_picker("Color", 150.0),
            ];
            self.ui.flexible_data_table(ids!(column_config_table)).set_columns(cx, columns);

            if let Some(config) = chart_config {
                // Set the title from existing config
                self.ui.text_input(ids!(chart_title_input)).set_text(cx, &config.title);

                // Find x_column index in table.columns
                let x_col_idx = table.columns.iter().position(|c| c == &config.x_column).unwrap_or(0);

                // Add X-axis row
                self.ui.flexible_data_table(ids!(column_config_table)).add_row_with_values(cx, vec![
                    CellValue::DropDown(x_col_idx),
                    CellValue::Text("X-axis".to_string()),
                    CellValue::DropDown(0),
                    CellValue::Color(makepad_widgets::vec4(0.29, 0.56, 0.89, 1.0)),
                ]);

                // Add Y-axis rows from config
                for y_col in &config.y_columns {
                    let col_idx = table.columns.iter().position(|c| c == &y_col.column_name).unwrap_or(0);
                    let chart_type_idx = match y_col.chart_type {
                        ChartType::Bar => 0,
                        ChartType::Line => 1,
                    };
                    let color = y_col.color.map(|c| makepad_widgets::vec4(c[0], c[1], c[2], c[3]))
                        .unwrap_or(makepad_widgets::vec4(0.89, 0.29, 0.29, 1.0));

                    self.ui.flexible_data_table(ids!(column_config_table)).add_row_with_values(cx, vec![
                        CellValue::DropDown(col_idx),
                        CellValue::Text(y_col.label.clone()),
                        CellValue::DropDown(chart_type_idx),
                        CellValue::Color(color),
                    ]);
                }
            } else {
                // Add two default rows (one for X-axis, one for Y-axis)
                // First row for X-axis (index 0), chart_type 0 = Bar
                self.ui.flexible_data_table(ids!(column_config_table)).add_row_with_values(cx, vec![
                    CellValue::DropDown(0),
                    CellValue::Text("X-axis".to_string()),
                    CellValue::DropDown(0),
                    CellValue::Color(makepad_widgets::vec4(0.29, 0.56, 0.89, 1.0)),
                ]);
                // Second row for Y-axis (index 1 if available, otherwise 0), chart_type 0 = Bar
                let y_index = if table.columns.len() > 1 { 1 } else { 0 };
                self.ui.flexible_data_table(ids!(column_config_table)).add_row_with_values(cx, vec![
                    CellValue::DropDown(y_index),
                    CellValue::Text("Y-axis".to_string()),
                    CellValue::DropDown(0),
                    CellValue::Color(makepad_widgets::vec4(0.89, 0.29, 0.29, 1.0)),
                ]);
            }

            // Hide the third and fourth cells (Type and Color) for the first row (X-axis)
            self.ui.flexible_data_table(ids!(column_config_table)).set_hidden_cells(cx, vec![LiveHiddenCell{
                row: 0,
                col: 2,
            }]);
        }

        self.ui.label(ids!(config_status)).set_text(cx, "");
        self.ui.redraw(cx);
    }

    fn apply_chart_config(&mut self, cx: &mut Cx, chart_index_to_change: usize) {
        let title = self.ui.text_input(ids!(chart_title_input)).text();

        // Get rows from the FlexibleDataTable
        let rows = self.ui.flexible_data_table(ids!(column_config_table)).get_rows();

        // Get column names from the data table
        let column_names: Vec<String> = if let Some(table) = self.data_tables.first() {
            table.columns.clone()
        } else {
            Vec::new()
        };

        if rows.is_empty() {
            self.ui.label(ids!(config_status)).set_text(cx, "Please add at least one column");
            return;
        }

        // First row is X-axis, remaining rows are Y-axis columns with their chart types
        // Row cells: [0: Column DropDown, 1: Label Text, 2: Type DropDown, 3: Color]
        let x_column_idx = rows[0].cells.first()
            .and_then(|c| c.as_dropdown())
            .unwrap_or(0);
        let x_column = column_names.get(x_column_idx)
            .cloned()
            .unwrap_or_default();

        // Create YColumnConfig for each Y-axis row (skip the first row which is X-axis)
        let y_columns: Vec<YColumnConfig> = rows.iter()
            .skip(1)
            .filter_map(|row| {
                let column_idx = row.cells.first().and_then(|c| c.as_dropdown()).unwrap_or(0);
                let column_name = column_names.get(column_idx)?.clone();
                let label = row.cells.get(1).and_then(|c| c.as_text()).unwrap_or("").to_string();
                let chart_type_idx = row.cells.get(2).and_then(|c| c.as_dropdown()).unwrap_or(0);
                let chart_type = match chart_type_idx {
                    0 => ChartType::Bar,
                    1 => ChartType::Line,
                    _ => ChartType::Bar,
                };
                let color_vec = row.cells.get(3).and_then(|c| c.as_color()).unwrap_or(makepad_widgets::vec4(0.29, 0.56, 0.89, 1.0));
                let color = Some([color_vec.x, color_vec.y, color_vec.z, color_vec.w]);
                Some(YColumnConfig::new(column_name, label, chart_type, color))
            })
            .collect();

        if y_columns.is_empty() {
            self.ui.label(ids!(config_status)).set_text(cx, "Please add at least one Y-axis column");
            return;
        }

        if let Some(table) = self.data_tables.first() {
            let config = ChartConfig::new(
                table.name.clone(),
                x_column,
                y_columns,
                title,
            );

            // Update the dashboard's charts at the specified index
            if let Some(dashboard) = &mut self.current_dashboard {
                if chart_index_to_change < dashboard.charts.len() {
                    dashboard.charts[chart_index_to_change] = config.clone();
                } else {
                    dashboard.charts.push(config.clone());
                }
            }

            self.navigate_to(cx, AppScreen::DashboardUpdate);
            self.render_chart(cx, chart_index_to_change, &config);
        }
    }

    fn save_current_dashboard(&mut self, _cx: &mut Cx) {
        if let Some(dashboard) = &self.current_dashboard {
            match save_dashboard(dashboard) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to save dashboard: {}", e);
                }
            }
        }
    }
    fn delete_chart(&mut self, cx: &mut Cx, idx: usize) {
        // Remove from dashboard config
        if let Some(dashboard) = &mut self.current_dashboard {
            if idx < dashboard.charts.len() {
                dashboard.charts.remove(idx);
            }
        }

        // Remove from UI using ChartList
        //self.ui.chart_list(ids!(chart_list)).delete_chart(cx, idx);
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
            current_screen: AppScreen::Home,
            data_tables: Vec::new(),
            dashboards: Vec::new(),
            current_dashboard: None,
            current_dashboard_index: None,
            chart_index_to_change: None
        }
    }
}

app_main!(App);
