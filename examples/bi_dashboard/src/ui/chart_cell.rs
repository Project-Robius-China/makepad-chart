use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::ComboChartWidgetRefExt;
use makepad_charts::chart::combo_chart::DatasetType;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_charts::chart::combo_chart::ComboChart;

    COLOR_PANEL = #2a2a2a

    pub ChartCell = {{ChartCell}} {
        width: Fill, height: Fill
        flow: Overlay
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_PANEL);
            }
        }

        combo_chart = <ComboChart> { visible: false, width: Fill, height: Fill }

        empty_label = <Label> {
            visible: true
            text: ""
            draw_text: {
                text_style: {font_size: 14.0},
                color: #888
            }
        }

        // Settings button in top-right corner for opening drawer
        settings_btn = <Button> {
            width: 32, height: 32
            margin: { top: 8, right: 8 }
            align: { x: 1.0, y: 0.0 }
            text: "⚙"
            draw_text: {
                text_style: { font_size: 16.0 }
                color: #888
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let sz = self.rect_size;
                    sdf.box(0.0, 0.0, sz.x, sz.y, 4.0);
                    sdf.fill(mix(#333, #444, self.hover));
                    return sdf.result;
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ChartCellAction {
    SettingsClicked,
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ChartCell {
    #[deref]
    view: View,
}

impl Widget for ChartCell {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChartCell {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // Check if settings button was clicked
        if self.view.button(ids!(settings_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ChartCellAction::SettingsClicked,
            );
        }
    }
}

impl ChartCellRef {
    /// Set chart data and options with per-column chart types for combo charts
    pub fn set_combo_data_options(&self, cx: &mut Cx, data: ChartData, options: ChartOptions, chart_types: Vec<DatasetType>) {
        for t in data.datasets.iter() {
            println!("t.background_color {:?}", t.background_color);
        }
        // Hide combo chart and empty label first
        self.combo_chart(ids!(combo_chart)).set_visible(cx, false);
        self.label(ids!(empty_label)).set_visible(cx, false);
        // Configure and show combo chart
        self.combo_chart(ids!(combo_chart)).set_data(data);
        self.combo_chart(ids!(combo_chart)).set_options(options);
        self.combo_chart(ids!(combo_chart)).set_dataset_types(chart_types);
        self.combo_chart(ids!(combo_chart)).update_title(cx);
        self.combo_chart(ids!(combo_chart)).set_visible(cx, true);
    }

    /// Show empty state
    pub fn show_empty(&self, cx: &mut Cx) {
        // Hide combo chart
        self.combo_chart(ids!(combo_chart)).set_visible(cx, false);

        // Show empty label
        self.label(ids!(empty_label)).set_visible(cx, true);

        // Trigger redraw
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }

    /// Check if this chart cell's settings button was clicked
    pub fn settings_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ChartCellAction::SettingsClicked = item.cast() {
                return true;
            }
        }
        false
    }
}
