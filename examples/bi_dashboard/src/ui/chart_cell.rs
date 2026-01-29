use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::{
    BarChartWidgetRefExt, LineChartWidgetRefExt, PieChartWidgetRefExt,
    ScatterChartWidgetRefExt, RadarChartWidgetRefExt, PolarAreaChartWidgetRefExt,
    BubbleChartWidgetRefExt, HorizontalBarChartWidgetRefExt, ComboChartWidgetRefExt,
    ChordChartWidgetRefExt,
};
use crate::config::ChartType;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_charts::chart::bar_chart::BarChart;
    use makepad_charts::chart::line_chart::LineChart;
    use makepad_charts::chart::pie_chart::PieChart;
    use makepad_charts::chart::scatter_chart::ScatterChart;
    use makepad_charts::chart::radar_chart::RadarChart;
    use makepad_charts::chart::polar_area_chart::PolarAreaChart;
    use makepad_charts::chart::bubble_chart::BubbleChart;
    use makepad_charts::chart::horizontal_bar_chart::HorizontalBarChart;
    use makepad_charts::chart::combo_chart::ComboChart;
    use makepad_charts::chart::chord_chart::ChordChart;

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

        bar_chart = <BarChart> { visible: false, width: Fill, height: Fill }
        line_chart = <LineChart> { visible: false, width: Fill, height: Fill }
        pie_chart = <PieChart> { visible: false, width: Fill, height: Fill }
        scatter_chart = <ScatterChart> { visible: false, width: Fill, height: Fill }
        radar_chart = <RadarChart> { visible: false, width: Fill, height: Fill }
        polar_chart = <PolarAreaChart> { visible: false, width: Fill, height: Fill }
        bubble_chart = <BubbleChart> { visible: false, width: Fill, height: Fill }
        hbar_chart = <HorizontalBarChart> { visible: false, width: Fill, height: Fill }
        combo_chart = <ComboChart> { visible: false, width: Fill, height: Fill }
        chord_chart = <ChordChart> { visible: false, width: Fill, height: Fill }

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
        // Hide all charts first
        self.bar_chart(ids!(bar_chart)).set_visible(cx, false);
        self.line_chart(ids!(line_chart)).set_visible(cx, false);
        self.pie_chart(ids!(pie_chart)).set_visible(cx, false);
        self.scatter_chart(ids!(scatter_chart)).set_visible(cx, false);
        self.radar_chart(ids!(radar_chart)).set_visible(cx, false);
        self.polar_area_chart(ids!(polar_chart)).set_visible(cx, false);
        self.bubble_chart(ids!(bubble_chart)).set_visible(cx, false);
        self.horizontal_bar_chart(ids!(hbar_chart)).set_visible(cx, false);
        self.combo_chart(ids!(combo_chart)).set_visible(cx, false);
        self.chord_chart(ids!(chord_chart)).set_visible(cx, false);
        self.label(ids!(empty_label)).set_visible(cx, false);

        // Show and configure the appropriate chart
        match chart_type {
            ChartType::Bar => {
                self.bar_chart(ids!(bar_chart)).set_data(data);
                self.bar_chart(ids!(bar_chart)).set_options(options);
                self.bar_chart(ids!(bar_chart)).set_visible(cx, true);
            }
            ChartType::Line => {
                self.line_chart(ids!(line_chart)).set_data(data);
                self.line_chart(ids!(line_chart)).set_options(options);
                self.line_chart(ids!(line_chart)).set_visible(cx, true);
            }
            ChartType::Pie => {
                self.pie_chart(ids!(pie_chart)).set_data(data);
                self.pie_chart(ids!(pie_chart)).set_options(options);
                self.pie_chart(ids!(pie_chart)).set_visible(cx, true);
            }
            ChartType::Scatter => {
                self.scatter_chart(ids!(scatter_chart)).set_data(data);
                self.scatter_chart(ids!(scatter_chart)).set_options(options);
                self.scatter_chart(ids!(scatter_chart)).set_visible(cx, true);
            }
            ChartType::Radar => {
                self.radar_chart(ids!(radar_chart)).set_data(data);
                self.radar_chart(ids!(radar_chart)).set_options(options);
                self.radar_chart(ids!(radar_chart)).set_visible(cx, true);
            }
            ChartType::PolarArea => {
                self.polar_area_chart(ids!(polar_chart)).set_data(data);
                self.polar_area_chart(ids!(polar_chart)).set_options(options);
                self.polar_area_chart(ids!(polar_chart)).set_visible(cx, true);
            }
            ChartType::Bubble => {
                self.bubble_chart(ids!(bubble_chart)).set_data(data);
                self.bubble_chart(ids!(bubble_chart)).set_options(options);
                self.bubble_chart(ids!(bubble_chart)).set_visible(cx, true);
            }
            ChartType::HorizontalBar => {
                self.horizontal_bar_chart(ids!(hbar_chart)).set_data(data);
                self.horizontal_bar_chart(ids!(hbar_chart)).set_options(options);
                self.horizontal_bar_chart(ids!(hbar_chart)).set_visible(cx, true);
            }
            ChartType::Combo => {
                self.combo_chart(ids!(combo_chart)).set_data(data);
                self.combo_chart(ids!(combo_chart)).set_options(options);
                self.combo_chart(ids!(combo_chart)).set_visible(cx, true);
            }
            ChartType::Chord => {
                // self.chord_chart(ids!(chord_chart)).set_data(data);
                // self.chord_chart(ids!(chord_chart)).set_options(options);
                // self.chord_chart(ids!(chord_chart)).set_visible(cx, true);
            }
        }
    }

    /// Show empty state
    pub fn show_empty(&self, cx: &mut Cx) {
        // Hide all charts
        self.bar_chart(ids!(bar_chart)).set_visible(cx, false);
        self.line_chart(ids!(line_chart)).set_visible(cx, false);
        self.pie_chart(ids!(pie_chart)).set_visible(cx, false);
        self.scatter_chart(ids!(scatter_chart)).set_visible(cx, false);
        self.radar_chart(ids!(radar_chart)).set_visible(cx, false);
        self.polar_area_chart(ids!(polar_chart)).set_visible(cx, false);
        self.bubble_chart(ids!(bubble_chart)).set_visible(cx, false);
        self.horizontal_bar_chart(ids!(hbar_chart)).set_visible(cx, false);
        self.combo_chart(ids!(combo_chart)).set_visible(cx, false);
        self.chord_chart(ids!(chord_chart)).set_visible(cx, false);

        // Show empty label
        self.label(ids!(empty_label)).set_visible(cx, true);
    }
}
