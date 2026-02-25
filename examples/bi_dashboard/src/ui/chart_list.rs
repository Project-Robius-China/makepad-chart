use makepad_widgets::*;
use makepad_components::editable_list::{MpEditableListWidgetExt, EditableListAction};
pub use crate::ui::chart_cell::{ChartCellWidgetRefExt, ChartDataGroup};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_components::theme::*;
    use makepad_components::editable_list::*;
    use makepad_components::button::*;
    use makepad_components::drawer::*;
    use crate::ui::chart_cell::ChartCell;

    pub ChartList = {{ChartList}} {
        width: Fill,
        height: Fill,
        flow: Overlay,

        list = <MpEditableList> {
            width: Fill,
            height: Fill,

            content = {
                list = {
                    width: Fill,
                    height: Fill,
                    flow: RightWrap,
                    padding: 10,

                    // Template for chart cells - ChartCell will be instantiated from this
                    template: <ChartCell> {
                        width: 400,
                        height: 500,
                        margin: 10,
                    }

                    // Plus button to add new charts
                    plus_template: <View> {
                        width: 100, height: 280
                        margin: 10
                        align: { x: 0.5, y: 0.5 }
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                return #2a2a2a;
                            }
                        }
                        plus_button = <Button> {
                            width: Fill, height: Fill
                            text: "+"
                            draw_text: {
                                text_style: { font_size: 32.0 }
                                color: #888
                            }
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return mix(#2a2a2a, #3a3a3a, self.hover);
                                }
                            }
                        }
                    }

                    // Setting button overlay - positioned at bottom of chart
                    setting_template: <View> {
                        width: Fill
                        height: Fill
                        flow: Down
                        align: { x: 0.5, y: 1.0 }
                        padding: { bottom: 8, left: 8, right: 8 }
                        setting_button = <Button> {
                            width: Fill
                            height: 36
                            text: "⚙ Settings"
                            draw_text: {
                                color: #fff
                            }
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return mix(#4a90e2, #5aa0f2, self.hover);
                                }
                            }
                        }
                    }
                }
            }

            // Use default drawer - no customization for now
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ChartListAction {
    None,
    AddChart,
    ConfigureChart(Option<usize>),
    MoveLeft(usize),
    MoveRight(usize),
    DeleteChart(usize),
}

#[derive(Live, LiveHook, Widget)]
pub struct ChartList {
    #[deref]
    view: View,
}

impl Widget for ChartList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }
}

impl WidgetMatchEvent for ChartList {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let editable_list = self.view.mp_editable_list(ids!(list));
        let list = editable_list.list();

        // Check if add button was clicked
        if list.add_requested(actions) { 
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ChartListAction::AddChart,
            );
        }

        // Handle EditableListAction from MpEditableList (drawer button clicks)
        if let Some(item) = actions.find_widget_action(editable_list.widget_uid()) {
            match item.cast() {
                EditableListAction::Configure(idx) => {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        ChartListAction::ConfigureChart(idx),
                    );
                }
                EditableListAction::MoveLeft(idx) => {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        ChartListAction::MoveLeft(idx),
                    );
                }
                EditableListAction::MoveRight(idx) => {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        ChartListAction::MoveRight(idx),
                    );
                }
                EditableListAction::Delete(idx) => {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        ChartListAction::DeleteChart(idx),
                    );
                }
                _ => {}
            }
        }
    }
}

impl ChartList {
    /// Set chart data for a specific index
    /// Creates the chart cell from template if it doesn't exist
    pub fn set_data(&mut self, cx: &mut Cx, idx: usize, data: ChartDataGroup) {
        let (chart_data, options, chart_types) = data;
        let list = self.view.mp_editable_list(ids!(list)).list();
        list.set_child(cx, idx, |cx, widget: WidgetRef| {
            // Cast the WidgetRef to ChartCellRef and set data
            widget.as_chart_cell().set_combo_data_options(cx, chart_data, options, chart_types);
        });
    }

    /// Delete a chart at the given index
    pub fn delete_chart(&mut self, cx: &mut Cx, idx: usize) {
        self.view.mp_editable_list(ids!(list)).delete_item(cx, idx);
    }
}

impl ChartListRef {
    /// Set chart data for a specific index
    pub fn set_data(&self, cx: &mut Cx, idx: usize, data: ChartDataGroup) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(cx, idx, data);
        }
    }

    /// Check if add chart was requested
    pub fn add_requested(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let ChartListAction::AddChart = item.cast() {
                    return true;
                }
            }
        }
        false
    }

    /// Check if configure was requested and return the chart index
    pub fn configure_requested(&self, actions: &Actions) -> Option<Option<usize>> {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let ChartListAction::ConfigureChart(idx) = item.cast() {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Check if move left was requested and return the chart index
    pub fn move_left_requested(&self, actions: &Actions) -> Option<usize> {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let ChartListAction::MoveLeft(idx) = item.cast() {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Check if move right was requested and return the chart index
    pub fn move_right_requested(&self, actions: &Actions) -> Option<usize> {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let ChartListAction::MoveRight(idx) = item.cast() {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Check if delete was requested and return the chart index
    pub fn delete_requested(&self, actions: &Actions) -> Option<usize> {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let ChartListAction::DeleteChart(idx) = item.cast() {
                    return Some(idx);
                }
            }
        }
        None
    }
}
