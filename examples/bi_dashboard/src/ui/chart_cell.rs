use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::ComboChartWidgetRefExt;
use makepad_charts::chart::combo_chart::DatasetType;
use crate::config::ChartType;

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
            text: "Click 'Add Chart' to add a chart here"
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
    /// Set chart data and options based on chart type
    pub fn set_data_options(&self, cx: &mut Cx, chart_type: ChartType, data: ChartData, options: ChartOptions) {
        // Hide combo chart and empty label first
        self.combo_chart(ids!(combo_chart)).set_visible(cx, false);
        self.label(ids!(empty_label)).set_visible(cx, false);

        // Convert ChartType to DatasetType
        let dataset_type = match chart_type {
            ChartType::Bar => DatasetType::Bar,
            ChartType::Line => DatasetType::Line,
        };

        // Set dataset types for all datasets in the data
        let dataset_types = vec![dataset_type; data.datasets.len()];

        // Configure and show combo chart
        self.combo_chart(ids!(combo_chart)).set_data(data);
        self.combo_chart(ids!(combo_chart)).set_options(options);
        self.combo_chart(ids!(combo_chart)).set_dataset_types(dataset_types);
        self.combo_chart(ids!(combo_chart)).set_visible(cx, true);
    }

    /// Show empty state
    pub fn show_empty(&self, cx: &mut Cx) {
        // Hide combo chart
        self.combo_chart(ids!(combo_chart)).set_visible(cx, false);

        // Show empty label
        self.label(ids!(empty_label)).set_visible(cx, true);
    }
}
