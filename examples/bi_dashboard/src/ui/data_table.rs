use makepad_widgets::*;
use makepad_components::color_picker::*;
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_components::color_picker::color_picker::*;

    COLOR_PANEL = #2a2a2a
    COLOR_BORDER = #3a3a3a
    COLOR_TEXT = #e0e0e0
    COLOR_PRIMARY = #4a90e2
    COLOR_HOVER = #5aa0f2
    COLOR_HEADER = #333333
    COLOR_ROW_BG = #252525

    // Color swatch button for data table
    ColorSwatchBtn = <MpColorPicker> {
        
    }

    pub DataTableRow = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: 5
        spacing: 10
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_ROW_BG);
            }
        }

        column_dropdown = <DropDown> {
            width: 180, height: 30
            labels: []
            values: []
        }

        body_input = <TextInput> {
            width: 120, height: 30
            text: ""
        }

        chart_type_dropdown = <DropDown> {
            width: 80, height: 30
            labels: ["Bar", "Line"]
            values: [Bar, Line]
        }

        color_btn = <ColorSwatchBtn> {}
    }

    pub DataTable = {{DataTable}} {
        width: Fill, height: Fit
        flow: Down
        spacing: 5
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_PANEL);
            }
        }
        padding: 10

        // Header row
        header = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: 5
            spacing: 10
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_HEADER);
                }
            }

            <Label> {
                width: 180
                text: "Column"
                draw_text: {
                    text_style: {font_size: 14.0},
                    color: (COLOR_TEXT)
                }
            }

            <Label> {
                width: 120
                text: "Label"
                draw_text: {
                    text_style: {font_size: 14.0},
                    color: (COLOR_TEXT)
                }
            }

            <Label> {
                width: 80
                text: "Type"
                draw_text: {
                    text_style: {font_size: 14.0},
                    color: (COLOR_TEXT)
                }
            }

            <Label> {
                width: 30
                text: "Color"
                draw_text: {
                    text_style: {font_size: 14.0},
                    color: (COLOR_TEXT)
                }
            }
        }

        // Rows using PortalList
        rows_list = <PortalList> {
            width: Fill, height: 200
            flow: Down
            spacing: 5

            DataTableRow = <DataTableRow> {}
        }

        // Footer with Add Row button
        footer = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: {top: 10}
            align: {x: 1.0, y: 0.5}

            add_row_btn = <Button> {
                width: 100, height: 30
                text: "Add Row"
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                    }
                }
            }
        }
    }
}

/// Represents a row's data
#[derive(Clone, Debug)]
pub struct TableRowData {
    pub column_index: usize,
    pub body: String,
    /// 0 = Bar, 1 = Line
    pub chart_type: usize,
    /// Color for the bar/line
    pub color: Vec4,
}

impl Default for TableRowData {
    fn default() -> Self {
        Self {
            column_index: 0,
            body: String::new(),
            chart_type: 0,
            color: vec4(0.29, 0.56, 0.89, 1.0), // Default blue #4a90e2
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DataTableAction {
    ColorButtonClicked(usize), // Row index
    None,
}

/// A table widget with column dropdown and body columns
#[derive(Live, LiveHook, Widget)]
pub struct DataTable {
    #[deref]
    view: View,

    #[rust]
    rows: Vec<TableRowData>,

    #[rust]
    column_names: Vec<String>,
}

impl Widget for DataTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.rows.len());
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < self.rows.len() {
                        let item = list.item(cx, item_id, live_id!(DataTableRow));
                        let row_data = &self.rows[item_id];

                        // Set dropdown labels from column names
                        item.drop_down(ids!(column_dropdown)).set_labels(cx, self.column_names.clone());
                        item.drop_down(ids!(column_dropdown)).set_selected_item(cx, row_data.column_index);

                        item.text_input(ids!(body_input)).set_text(cx, &row_data.body);

                        // Set chart type dropdown (Bar=0, Line=1)
                        item.drop_down(ids!(chart_type_dropdown)).set_selected_item(cx, row_data.chart_type);

                        // Set color button color
                        item.button(ids!(color_btn)).apply_over(cx, live! {
                            draw_bg: { color: (row_data.color) }
                        });

                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for DataTable {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(ids!(rows_list));

        for (row_idx, item_widget) in list_widget.items_with_actions(actions) {
            if row_idx >= self.rows.len() {
                continue;
            }

            let column_dropdown = item_widget.drop_down(ids!(column_dropdown));
            let body_input = item_widget.text_input(ids!(body_input));
            let chart_type_dropdown = item_widget.drop_down(ids!(chart_type_dropdown));
            let color_btn = item_widget.button(ids!(color_btn));

            // Check column dropdown changes
            if let Some(selected) = column_dropdown.changed(actions) {
                self.rows[row_idx].column_index = selected;
                cx.redraw_all();
            }

            // Check body text input changes
            if let Some(text) = body_input.changed(actions) {
                self.rows[row_idx].body = text;
                cx.redraw_all();
            }

            // Check chart type dropdown changes
            if let Some(selected) = chart_type_dropdown.changed(actions) {
                self.rows[row_idx].chart_type = selected;
                cx.redraw_all();
            }

            // Check color picker changes
            let color_picker = item_widget.mp_color_picker(ids!(color_btn));
            if let Some(hsv) = color_picker.changed(actions) {
                self.rows[row_idx].color = hsv.to_vec4();
                cx.redraw_all();
            }

            // Check color button click
            if color_btn.clicked(actions) {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    DataTableAction::ColorButtonClicked(row_idx),
                );
            }
        }
    }
}

#[allow(dead_code)]
impl DataTable {
    /// Set the column names for the dropdown
    pub fn set_column_names(&mut self, names: Vec<String>) {
        self.column_names = names;
    }

    /// Add a new empty row
    pub fn add_row(&mut self) {
        self.rows.push(TableRowData::default());
    }

    /// Add a row with data
    pub fn add_row_with_data(&mut self, column_index: usize, body: String, chart_type: usize) {
        self.rows.push(TableRowData { column_index, body, chart_type, ..Default::default() });
    }

    /// Set row color
    pub fn set_row_color(&mut self, row_idx: usize, color: Vec4) {
        if row_idx < self.rows.len() {
            self.rows[row_idx].color = color;
        }
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get all rows data
    pub fn get_rows(&self) -> &[TableRowData] {
        &self.rows
    }

    /// Get column names
    pub fn get_column_names(&self) -> &[String] {
        &self.column_names
    }

    /// Clear all rows
    pub fn clear(&mut self) {
        self.rows.clear();
    }
}

#[allow(dead_code)]
impl DataTableRef {
    /// Set column names for the dropdown and redraw
    pub fn set_column_names(&self, cx: &mut Cx, names: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_column_names(names);
            inner.redraw(cx);
        }
    }

    /// Add a new empty row and redraw
    pub fn add_row(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_row();
            inner.redraw(cx);
        }
    }

    /// Add a row with data and redraw
    pub fn add_row_with_data(&self, cx: &mut Cx, column_index: usize, body: String, chart_type: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_row_with_data(column_index, body, chart_type);
            inner.redraw(cx);
        }
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.row_count()
        } else {
            0
        }
    }

    /// Get all rows data
    pub fn get_rows(&self) -> Vec<TableRowData> {
        if let Some(inner) = self.borrow() {
            inner.get_rows().to_vec()
        } else {
            Vec::new()
        }
    }

    /// Get column names
    pub fn get_column_names(&self) -> Vec<String> {
        if let Some(inner) = self.borrow() {
            inner.get_column_names().to_vec()
        } else {
            Vec::new()
        }
    }

    /// Clear all rows and redraw
    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
            inner.redraw(cx);
        }
    }

    /// Check if add_row_btn was clicked
    pub fn add_row_clicked(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.button(ids!(add_row_btn)).clicked(actions)
        } else {
            false
        }
    }

    /// Check if a color button was clicked, returns the row index
    pub fn color_btn_clicked(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DataTableAction::ColorButtonClicked(row_idx) = item.cast() {
                return Some(row_idx);
            }
        }
        None
    }

    /// Set row color and redraw
    pub fn set_row_color(&self, cx: &mut Cx, row_idx: usize, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_row_color(row_idx, color);
            inner.redraw(cx);
        }
    }
}
