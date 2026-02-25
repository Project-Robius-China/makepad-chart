use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::ComboChartWidgetRefExt;
use makepad_charts::chart::combo_chart::DatasetType;
pub type ChartDataGroup = (ChartData, ChartOptions, Vec<DatasetType>);

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
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChartCell {
    #[deref]
    view: View,
}

impl Widget for ChartCell {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChartCellRef {
    /// Set chart data and options with per-column chart types for combo charts
    pub fn set_combo_data_options(&self, cx: &mut Cx, data: ChartData, options: ChartOptions, chart_types: Vec<DatasetType>) {
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
}
