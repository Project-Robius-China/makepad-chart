use crate::config::{ChartConfig, ChartType, DashboardConfig, GridPosition, save_dashboard, load_all_dashboards};
use crate::data::{DataTable, parse_csv};
use crate::transform::table_to_chart_data;
use crate::ui::{self, AppScreen};
use crate::ui::chart_cell::{ChartCellRef, ChartCellWidgetRefExt};
use makepad_widgets::*;
use makepad_charts::*;
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::ui::chart_cell::ChartCell;

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

                        dashboard_list = <ScrollYView> {
                            width: Fill, height: Fill
                            <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 10
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

                        // 2x2 Chart Grid
                        chart_grid = <View> {
                            width: Fill, height: Fill
                            flow: Down
                            spacing: 20

                            // Row 1
                            <View> {
                                width: Fill, height: Fill
                                flow: Right
                                spacing: 20

                                chart_0_0 = <ChartCell> {}

                                chart_0_1 = <ChartCell> {}
                            }

                            // Row 2
                            <View> {
                                width: Fill, height: Fill
                                flow: Right
                                spacing: 20

                                chart_1_0 = <ChartCell> {}

                                chart_1_1 = <ChartCell> {}
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

                        <Label> {
                            text: "Configure Chart"
                            draw_text: {
                                text_style: {font_size: 18.0},
                                color: (COLOR_TEXT)
                            }
                            padding: {bottom: 20}
                        }

                        <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 15

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

                            <Label> {
                                text: "Chart Type"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            chart_type_dropdown = <DropDown> {
                                width: 400, height: 30
                                labels: ["Bar Chart", "Line Chart", "Pie Chart", "Scatter Plot", "Radar Chart", "Polar Area", "Bubble Chart", "Horizontal Bar", "Combo Chart", "Chord Diagram"]
                                values: [Bar, Line, Pie, Scatter, Radar, PolarArea, Bubble, HorizontalBar, Combo, Chord]
                            }

                            <Label> {
                                text: "Grid Position - Row"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            row_dropdown = <DropDown> {
                                width: 400, height: 30
                                labels: ["Row 0 (Top)", "Row 1 (Bottom)"]
                                values: [0, 1]
                            }

                            <Label> {
                                text: "Grid Position - Column"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            column_dropdown = <DropDown> {
                                width: 400, height: 30
                                labels: ["Column 0 (Left)", "Column 1 (Right)"]
                                values: [0, 1]
                            }

                            <Label> {
                                text: "Data Source"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            data_source_label = <Label> {
                                text: "No data loaded"
                                draw_text: {
                                    text_style: {font_size: 12.0},
                                    color: #888
                                }
                            }

                            <Label> {
                                text: "X-Axis Column"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            x_column_input = <TextInput> {
                                width: 400, height: 30
                                text: ""
                            }

                            <Label> {
                                text: "Y-Axis Column(s) (comma separated)"
                                draw_text: {
                                    text_style: {font_size: 14.0},
                                    color: (COLOR_TEXT)
                                }
                                padding: {top: 10}
                            }

                            y_columns_input = <TextInput> {
                                width: 400, height: 30
                                text: ""
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
    current_dashboard_index: Option<usize>,

    #[rust]
    config_target_position: Option<GridPosition>,

    #[rust]
    selected_chart_type: ChartType,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_charts::live_design(cx);
        ui::chart_cell::live_design(cx);
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

        if self.ui.button(ids!(add_chart_btn)).clicked(actions) {
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

        // Handle chart type dropdown changes
        if let Some(selected) = self.ui.drop_down(ids!(chart_type_dropdown)).selected(actions) {
            self.selected_chart_type = match selected {
                0 => ChartType::Bar,
                1 => ChartType::Line,
                2 => ChartType::Pie,
                3 => ChartType::Scatter,
                4 => ChartType::Radar,
                5 => ChartType::PolarArea,
                6 => ChartType::Bubble,
                7 => ChartType::HorizontalBar,
                8 => ChartType::Combo,
                9 => ChartType::Chord,
                _ => ChartType::Bar,
            };
        }

        // Handle row dropdown changes
        if let Some(selected) = self.ui.drop_down(ids!(row_dropdown)).selected(actions) {
            if let Some(pos) = &mut self.config_target_position {
                pos.row = selected;
            }
        }

        // Handle column dropdown changes
        if let Some(selected) = self.ui.drop_down(ids!(column_dropdown)).selected(actions) {
            if let Some(pos) = &mut self.config_target_position {
                pos.col = selected;
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
            }
        }
    }

    fn render_dashboard(&mut self, cx: &mut Cx) {
        if let Some(dashboard) = self.current_dashboard.clone() {
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
                self.render_chart(cx, chart_config);
            }
        }

        self.ui.redraw(cx);
    }

    fn hide_all_charts_at(&mut self, cx: &mut Cx, row: usize, col: usize) {
        // Use ChartCell's show_empty method to hide all charts and show empty label
        match (row, col) {
            (0, 0) => self.ui.chart_cell(ids!(chart_0_0)).show_empty(cx),
            (0, 1) => self.ui.chart_cell(ids!(chart_0_1)).show_empty(cx),
            (1, 0) => self.ui.chart_cell(ids!(chart_1_0)).show_empty(cx),
            (1, 1) => self.ui.chart_cell(ids!(chart_1_1)).show_empty(cx),
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
        let chart_data = match table_to_chart_data(table, &config.x_column, &config.y_columns) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to convert table to chart data: {}", e);
                return;
            }
        };

        // Create chart options
        let options = ChartOptions::new()
            .with_title(&config.title)
            .with_animation_duration(500.0);

        // Use ChartCell's set_data_options method to display the correct chart type
        let row = config.position.row;
        let col = config.position.col;

        match (row, col) {
            (0, 0) => self.ui.chart_cell(ids!(chart_0_0)).set_data_options(cx, config.chart_type, chart_data, options),
            (0, 1) => self.ui.chart_cell(ids!(chart_0_1)).set_data_options(cx, config.chart_type, chart_data, options),
            (1, 0) => self.ui.chart_cell(ids!(chart_1_0)).set_data_options(cx, config.chart_type, chart_data, options),
            (1, 1) => self.ui.chart_cell(ids!(chart_1_1)).set_data_options(cx, config.chart_type, chart_data, options),
            _ => {}
        }
    }

    fn setup_chart_config(&mut self, cx: &mut Cx) {
        if let Some(table) = self.data_tables.first() {
            let data_source_text = format!("{} ({} columns)", table.name, table.column_count());
            self.ui
                .label(ids!(data_source_label))
                .set_text(cx, &data_source_text);

            // Pre-fill with first column suggestions
            if !table.columns.is_empty() {
                self.ui
                    .text_input(ids!(x_column_input))
                    .set_text(cx, &table.columns[0]);

                if table.columns.len() > 1 {
                    self.ui
                        .text_input(ids!(y_columns_input))
                        .set_text(cx, &table.columns[1]);
                }
            }
        }

        // Set initial position in dropdowns
        if let Some(pos) = self.config_target_position {
            self.ui.drop_down(ids!(row_dropdown)).set_selected_item(cx, 1);
            self.ui.drop_down(ids!(column_dropdown)).set_selected_item(cx, 1);
        }

        self.ui.label(ids!(config_status)).set_text(cx, "");
        self.ui.redraw(cx);
    }

    fn apply_chart_config(&mut self, cx: &mut Cx) {
        let title = self.ui.text_input(ids!(chart_title_input)).text();
        let x_column = self.ui.text_input(ids!(x_column_input)).text();
        let y_columns_text = self.ui.text_input(ids!(y_columns_input)).text();

        let y_columns: Vec<String> = y_columns_text
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

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
                self.selected_chart_type,
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

    fn save_current_dashboard(&mut self, cx: &mut Cx) {
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
            selected_chart_type: ChartType::Bar,
        }
    }
}

app_main!(App);
