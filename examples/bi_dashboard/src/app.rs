use crate::config::{ChartConfig, ChartType, DashboardConfig, GridPosition, YColumnConfig, save_dashboard, load_all_dashboards};
use crate::data::{DataTable, parse_csv};
use crate::transform::table_to_combo_chart_data;
use crate::ui::{self, AppScreen};
use crate::ui::chart_cell::ChartCellWidgetRefExt;
use crate::ui::data_table::DataTableWidgetRefExt;
use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::combo_chart::DatasetType;
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::ui::chart_cell::ChartCell;
    use crate::ui::data_table::DataTable;

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

                        // Scrollable Chart Grid - Flow Right
                        <ScrollXYView> {
                            width: Fill, height: Fill

                            chart_grid = <View> {
                                width: Fit, height: Fill
                                flow: Right
                                spacing: 20
                                padding: 10

                                chart_0_0 = <ChartCell> {
                                    width: 400, height: Fill
                                }

                                chart_0_1 = <ChartCell> {
                                    width: 400, height: Fill
                                }

                                // Add Chart button at the end
                                add_chart_inline_btn = <Button> {
                                    width: 100, height: Fill
                                    text: "+"
                                    draw_text: {
                                        text_style: {font_size: 32.0}
                                    }
                                    draw_bg: {
                                        fn pixel(self) -> vec4 {
                                            return mix((COLOR_PANEL), (COLOR_HOVER), self.hover);
                                        }
                                    }
                                }
                            }
                        }
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

                            column_config_table = <DataTable> {
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
    config_target_position: Option<GridPosition>,

    /// Index of the chart selected for drawer actions (0-3 for chart_0_0 to chart_1_1)
    #[rust]
    drawer_selected_chart: Option<usize>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_components::live_design(cx);
        makepad_charts::live_design(cx);
        
        ui::chart_cell::live_design(cx);
        ui::data_table::live_design(cx);
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
            self.navigate_to(cx, AppScreen::Dashboard);
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
                    self.navigate_to(cx, AppScreen::Dashboard);
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
                // Find next available position
                if let Some(pos) = self.find_next_available_position() {
                    self.config_target_position = Some(pos);
                    self.navigate_to(cx, AppScreen::ChartConfig);
                }
            }
        }

        if self.ui.button(ids!(save_dashboard_btn)).clicked(actions) {
            self.save_current_dashboard(cx);
        }

        // Chart config screen actions
        if self.ui.button(ids!(back_from_config_btn)).clicked(actions) {
            self.navigate_to(cx, AppScreen::Dashboard);
        }

        if self.ui.button(ids!(apply_config_btn)).clicked(actions) {
            self.apply_chart_config(cx);
        }

        // Handle DataTable Add Row button click
        if self.ui.data_table(ids!(column_config_table)).add_row_clicked(actions) {
            self.ui.data_table(ids!(column_config_table)).add_row(cx);
        }

        // Handle settings button click on chart cells
        let chart_ids = [
            ids!(chart_0_0),
        ];

        for (idx, id) in chart_ids.iter().enumerate() {
            if self.ui.chart_cell(*id).settings_clicked(actions) {
                self.drawer_selected_chart = Some(idx);
                self.ui.view(ids!(chart_drawer)).set_visible(cx, true);
                self.ui.redraw(cx);
                break;
            }
        }

        // Drawer close button or backdrop click
        if self.ui.button(ids!(drawer_close_btn)).clicked(actions) {
            self.close_drawer(cx);
        }

        // Drawer configure button
        if self.ui.button(ids!(drawer_configure_btn)).clicked(actions) {
            if let Some(idx) = self.drawer_selected_chart {
                let pos = GridPosition { row: idx / 2, col: idx % 2 };
                self.config_target_position = Some(pos);
                self.close_drawer(cx);
                self.navigate_to(cx, AppScreen::ChartConfig);
            }
        }

        // Drawer move left button
        if self.ui.button(ids!(drawer_move_left_btn)).clicked(actions) {
            if let Some(idx) = self.drawer_selected_chart {
                if idx > 0 {
                    self.swap_charts(cx, idx, idx - 1);
                    self.drawer_selected_chart = Some(idx - 1);
                }
            }
        }

        // Drawer move right button
        if self.ui.button(ids!(drawer_move_right_btn)).clicked(actions) {
            if let Some(idx) = self.drawer_selected_chart {
                if idx < 3 {
                    self.swap_charts(cx, idx, idx + 1);
                    self.drawer_selected_chart = Some(idx + 1);
                }
            }
        }

        // Drawer delete button
        if self.ui.button(ids!(drawer_delete_btn)).clicked(actions) {
            if let Some(idx) = self.drawer_selected_chart {
                self.delete_chart(cx, idx);
                self.close_drawer(cx);
            }
        }
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
        match screen {
            AppScreen::Home => {
                self.ui.view(ids!(home_screen)).set_visible(cx, true);
                self.load_dashboard_list(cx);
            }
            AppScreen::Import => {
                self.ui.view(ids!(import_screen)).set_visible(cx, true);
            }
            AppScreen::Dashboard => {
                self.ui.view(ids!(dashboard_screen)).set_visible(cx, true);
                self.render_dashboard(cx);
            }
            AppScreen::ChartConfig => {
                self.ui.view(ids!(chart_config_screen)).set_visible(cx, true);
                self.setup_chart_config(cx);
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
            println!("render_dashboard: {} charts", dashboard.charts.len());
            self.ui
                .label(ids!(dashboard_name))
                .set_text(cx, &dashboard.name);

            // Hide all charts and show empty labels by default
            for row in 0..2 {
                for col in 0..2 {
                    self.hide_all_charts_at(cx, row, col);
                }
            }

            // Render each configured chart
            for chart_config in &dashboard.charts {
                println!("render_dashboard: rendering chart at ({},{})", chart_config.position.row, chart_config.position.col);
                self.render_chart(cx, chart_config);
            }
        }

        self.ui.redraw(cx);
    }

    fn hide_all_charts_at(&mut self, cx: &mut Cx, row: usize, col: usize) {
        // Use ChartCell's show_empty method to hide all charts and show empty label
        match (row, col) {
            (0, 0) => self.ui.chart_cell(ids!(chart_0_0)).show_empty(cx),
            _ => {}
        }
    }

    fn render_chart(&mut self, cx: &mut Cx, config: &ChartConfig) {
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

        // Use ChartCell's set_combo_data_options method to display per-column chart types
        let row = config.position.row;
        let col = config.position.col;

        match (row, col) {
            (0, 0) => self.ui.chart_cell(ids!(chart_0_0)).set_combo_data_options(cx, chart_data, options, chart_types),
            _ => {}
        }
    }

    fn setup_chart_config(&mut self, cx: &mut Cx) {
        // Clear the DataTable first
        self.ui.data_table(ids!(column_config_table)).clear(cx);

        if let Some(table) = self.data_tables.first() {
            let data_source_text = format!("{} ({} columns)", table.name, table.column_count());
            self.ui
                .label(ids!(data_source_label))
                .set_text(cx, &data_source_text);

            // Set column names on the DataTable
            self.ui
                .data_table(ids!(column_config_table))
                .set_column_names(cx, table.columns.clone());

            // Add two default rows (one for X-axis, one for Y-axis)
            // First row for X-axis (index 0), chart_type 0 = Bar
            self.ui.data_table(ids!(column_config_table)).add_row_with_data(cx, 0, "X-axis".to_string(), 0);
            // Second row for Y-axis (index 1 if available, otherwise 0), chart_type 0 = Bar
            let y_index = if table.columns.len() > 1 { 1 } else { 0 };
            self.ui.data_table(ids!(column_config_table)).add_row_with_data(cx, y_index, "Y-axis".to_string(), 0);
        }

        self.ui.label(ids!(config_status)).set_text(cx, "");
        self.ui.redraw(cx);
    }

    fn apply_chart_config(&mut self, cx: &mut Cx) {
        let title = self.ui.text_input(ids!(chart_title_input)).text();

        // Get rows from the DataTable
        let rows = self.ui.data_table(ids!(column_config_table)).get_rows();
        let column_names = self.ui.data_table(ids!(column_config_table)).get_column_names();

        if rows.is_empty() {
            self.ui.label(ids!(config_status)).set_text(cx, "Please add at least one column");
            return;
        }

        // First row is X-axis, remaining rows are Y-axis columns with their chart types
        let x_column = column_names.get(rows[0].column_index)
            .cloned()
            .unwrap_or_default();

        // Create YColumnConfig for each Y-axis row (skip the first row which is X-axis)
        let y_columns: Vec<YColumnConfig> = rows.iter()
            .skip(1)
            .filter_map(|row| {
                let column_name = column_names.get(row.column_index)?.clone();
                let chart_type = match row.chart_type {
                    0 => ChartType::Bar,
                    1 => ChartType::Line,
                    _ => ChartType::Bar,
                };
                let color = Some([row.color.x, row.color.y, row.color.z, row.color.w]);
                Some(YColumnConfig::new(column_name, row.body.clone(), chart_type, color))
            })
            .collect();

        if y_columns.is_empty() {
            self.ui.label(ids!(config_status)).set_text(cx, "Please add at least one Y-axis column");
            return;
        }

        if let (Some(table), Some(dashboard), Some(position)) = (
            self.data_tables.first(),
            &mut self.current_dashboard,
            self.config_target_position,
        ) {
            // Remove any existing chart at this position
            dashboard.charts.retain(|c|
                !(c.position.row == position.row && c.position.col == position.col)
            );

            let chart = ChartConfig::new(
                table.name.clone(),
                x_column,
                y_columns,
                title,
                position,
            );

            dashboard.add_chart(chart);
            self.navigate_to(cx, AppScreen::Dashboard);
        }
    }

    fn find_next_available_position(&self) -> Option<GridPosition> {
        if let Some(dashboard) = &self.current_dashboard {
            for row in 0..2 {
                for col in 0..2 {
                    if dashboard.get_chart_at_position(row, col).is_none() {
                        return Some(GridPosition::new(row, col));
                    }
                }
            }
        }
        None
    }

    fn save_current_dashboard(&mut self, _cx: &mut Cx) {
        if let Some(dashboard) = &self.current_dashboard {
            match save_dashboard(dashboard) {
                Ok(_) => {
                    println!("Dashboard saved successfully");
                }
                Err(e) => {
                    eprintln!("Failed to save dashboard: {}", e);
                }
            }
        }
    }

    fn close_drawer(&mut self, cx: &mut Cx) {
        self.drawer_selected_chart = None;
        self.ui.view(ids!(chart_drawer)).set_visible(cx, false);
        self.ui.redraw(cx);
    }

    fn swap_charts(&mut self, cx: &mut Cx, idx1: usize, idx2: usize) {
        if let Some(dashboard) = &mut self.current_dashboard {
            let pos1 = GridPosition { row: idx1 / 2, col: idx1 % 2 };
            let pos2 = GridPosition { row: idx2 / 2, col: idx2 % 2 };

            // Find charts at these positions
            let chart1 = dashboard.charts.iter().position(|c|
                c.position.row == pos1.row && c.position.col == pos1.col
            );
            let chart2 = dashboard.charts.iter().position(|c|
                c.position.row == pos2.row && c.position.col == pos2.col
            );

            // Swap positions
            match (chart1, chart2) {
                (Some(i1), Some(i2)) => {
                    // Both positions have charts, swap their positions
                    dashboard.charts[i1].position = pos2;
                    dashboard.charts[i2].position = pos1;
                }
                (Some(i1), None) => {
                    // Only first position has a chart, move it to second
                    dashboard.charts[i1].position = pos2;
                }
                (None, Some(i2)) => {
                    // Only second position has a chart, move it to first
                    dashboard.charts[i2].position = pos1;
                }
                (None, None) => {
                    // Neither position has a chart, nothing to do
                }
            }

            self.render_dashboard(cx);
        }
    }

    fn delete_chart(&mut self, cx: &mut Cx, idx: usize) {
        if let Some(dashboard) = &mut self.current_dashboard {
            let pos = GridPosition { row: idx / 2, col: idx % 2 };
            let before_count = dashboard.charts.len();
            dashboard.charts.retain(|c|
                !(c.position.row == pos.row && c.position.col == pos.col)
            );
            let after_count = dashboard.charts.len();
            println!("delete_chart: idx={}, pos=({},{}), before={}, after={}", idx, pos.row, pos.col, before_count, after_count);
            self.render_dashboard(cx);
        } else {
            println!("delete_chart: current_dashboard is None");
        }
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
            config_target_position: None,
            drawer_selected_chart: None,
        }
    }
}

app_main!(App);
