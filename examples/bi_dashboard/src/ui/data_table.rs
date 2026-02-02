use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    COLOR_PANEL = #2a2a2a
    COLOR_BORDER = #3a3a3a
    COLOR_TEXT = #e0e0e0
    COLOR_PRIMARY = #4a90e2
    COLOR_HOVER = #5aa0f2
    COLOR_HEADER = #333333
    COLOR_ROW_BG = #252525

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
            width: 200, height: 30
            labels: []
            values: []
        }

        body_input = <TextInput> {
            width: Fill, height: 30
            text: ""
        }
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
                width: 200
                text: "Column"
                draw_text: {
                    text_style: {font_size: 14.0},
                    color: (COLOR_TEXT)
                }
            }

            <Label> {
                width: Fill
                text: "Body"
                draw_text: {
                    text_style: {font_size: 14.0},
                    color: (COLOR_TEXT)
                }
            }
        }

        // Rows will be rendered via PortalList
        rows_list = <PortalList> {
            width: Fill, height: Fit
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
#[derive(Clone, Debug, Default)]
pub struct TableRowData {
    pub column_index: usize,
    pub body: String,
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

        // Handle PortalList events for rows
        let actions = cx.capture_actions(|cx| {
            self.portal_list(ids!(rows_list)).handle_event(cx, event, scope);
        });

        // Sync changes back to our data
        for (row_idx, item) in self.portal_list(ids!(rows_list)).items_with_actions(&actions) {
            if let Some(selected) = item.drop_down(ids!(column_dropdown)).selected(&actions) {
                if row_idx < self.rows.len() {
                    self.rows[row_idx].column_index = selected;
                }
            }
            let body = item.text_input(ids!(body_input)).text();
            if row_idx < self.rows.len() {
                self.rows[row_idx].body = body;
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Draw the main view structure
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
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
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
    pub fn add_row_with_data(&mut self, column_index: usize, body: String) {
        self.rows.push(TableRowData { column_index, body });
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
    pub fn add_row_with_data(&self, cx: &mut Cx, column_index: usize, body: String) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_row_with_data(column_index, body);
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
}
