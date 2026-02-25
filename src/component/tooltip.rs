//! A chart tooltip widget with a callout arrow/triangle that points at the target element.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Chart tooltip with callout triangle pointing downward
    pub ChartCalloutTooltipInner = <Tooltip> {
        content: <View> {
            flow: Overlay,
            width: Fit,
            height: Fit,

            rounded_view = <RoundedView> {
                width: Fit,
                height: Fit,
                padding: 10,

                draw_bg: {
                    color: #fff,
                    border_color: #D0D5DD,
                    border_radius: 4.,
                    instance background_color: #333,
                    instance tooltip_pos: vec2(0.0, 0.0),
                    instance target_pos: vec2(0.0, 0.0),
                    instance target_size: vec2(0.0, 0.0),
                    instance expected_dimension_x: 0.0,
                    instance triangle_height: 6.0,
                    // 0 = up, 90 = right, 180 = down, 270 = left
                    instance callout_position: 180.0,

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let rect_size = self.rect_size;
                        let triangle_height = self.triangle_height;

                        // Don't draw if dimensions not calculated yet
                        if self.expected_dimension_x == 0.0 {
                            return sdf.result;
                        }

                        // Draw rounded box with padding for triangle
                        sdf.box(
                            triangle_height,
                            triangle_height,
                            rect_size.x - (triangle_height * 2.0),
                            rect_size.y - (triangle_height * 2.0),
                            max(1.0, self.border_radius)
                        )
                        sdf.fill(self.background_color);

                        // Draw callout triangle
                        let mut vertex1 = vec2(0.0, 0.0);
                        let mut vertex2 = vec2(0.0, 0.0);
                        let mut vertex3 = vec2(0.0, 0.0);

                        if self.callout_position == 0.0 {
                            // Point upwards (tooltip below target)
                            let center_x = rect_size.x / 2.0;
                            vertex1 = vec2(center_x - triangle_height, triangle_height);
                            vertex2 = vec2(center_x, 0.0);
                            vertex3 = vec2(center_x + triangle_height, triangle_height);
                        } else if self.callout_position == 90.0 {
                            // Point rightwards (tooltip left of target)
                            let center_y = rect_size.y / 2.0;
                            vertex1 = vec2(rect_size.x - triangle_height, center_y - triangle_height);
                            vertex2 = vec2(rect_size.x, center_y);
                            vertex3 = vec2(rect_size.x - triangle_height, center_y + triangle_height);
                        } else if self.callout_position == 180.0 {
                            // Point downwards (tooltip above target)
                            let diff_x = self.target_pos.x + self.target_size.x / 2.0 - self.tooltip_pos.x + triangle_height;
                            vertex1 = vec2(
                                min(max(triangle_height * 3.0 + 2.0, diff_x), rect_size.x - triangle_height - 2.0),
                                rect_size.y - triangle_height - 2.0
                            );
                            vertex2 = vec2(vertex1.x - triangle_height, vertex1.y + triangle_height);
                            vertex3 = vec2(vertex1.x - triangle_height * 2.0, vertex1.y);
                        } else {
                            // Point leftwards (270) (tooltip right of target)
                            let center_y = rect_size.y / 2.0;
                            vertex1 = vec2(triangle_height, center_y - triangle_height);
                            vertex2 = vec2(0.0, center_y);
                            vertex3 = vec2(triangle_height, center_y + triangle_height);
                        }

                        sdf.move_to(vertex1.x, vertex1.y);
                        sdf.line_to(vertex2.x, vertex2.y);
                        sdf.line_to(vertex3.x, vertex3.y);
                        sdf.close_path();
                        sdf.fill(self.background_color);

                        return sdf.result;
                    }
                }

                tooltip_label = <Label> {
                    width: Fit,
                    height: Fit,
                    draw_text: {
                        text_style: {font_size: 10},
                        text_wrap: Line,
                        color: #fff,
                    }
                }
            }
        }
    }

    pub ChartCalloutTooltip = {{ChartCalloutTooltip}} {
        tooltip = <ChartCalloutTooltipInner> {}
    }
}

/// Options for displaying a chart tooltip
#[derive(Clone, Debug)]
pub struct ChartTooltipOptions {
    /// Text color. Defaults to white.
    pub text_color: Vec4,
    /// Background color. Defaults to dark gray.
    pub bg_color: Vec4,
    /// Position relative to target (Top, Bottom, Left, Right)
    pub position: TooltipPosition,
    /// Height of the callout triangle
    pub triangle_height: f64,
}

impl Default for ChartTooltipOptions {
    fn default() -> Self {
        Self {
            text_color: vec4(1.0, 1.0, 1.0, 1.0),
            bg_color: vec4(0.2, 0.2, 0.2, 1.0),
            position: TooltipPosition::Top,
            triangle_height: 6.0,
        }
    }
}

/// Position of tooltip relative to target
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Tooltip above target, triangle points down
    #[default]
    Top,
    /// Tooltip below target, triangle points up
    Bottom,
    /// Tooltip left of target, triangle points right
    Left,
    /// Tooltip right of target, triangle points left
    Right,
}

impl TooltipPosition {
    /// Get the callout angle for this position
    pub fn callout_angle(&self) -> f64 {
        match self {
            TooltipPosition::Top => 180.0,    // Triangle points down
            TooltipPosition::Bottom => 0.0,   // Triangle points up
            TooltipPosition::Left => 90.0,    // Triangle points right
            TooltipPosition::Right => 270.0,  // Triangle points left
        }
    }
}

/// A chart tooltip widget with callout triangle
#[derive(Live, LiveHook, Widget)]
pub struct ChartCalloutTooltip {
    #[deref]
    view: View,

    #[rust]
    timer_redraw: Timer,

    #[rust]
    latest_options: Option<(String, Rect, ChartTooltipOptions)>,
}

impl Widget for ChartCalloutTooltip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.timer_redraw.is_event(event).is_some() {
            if let Some((text, target_rect, options)) = self.latest_options.take() {
                self.show_internal(cx, &text, target_rect, options, true);
            }
            cx.stop_timer(self.timer_redraw);
            self.timer_redraw = Timer::empty();
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChartCalloutTooltip {
    /// Show tooltip with text positioned relative to target_rect
    pub fn show(&mut self, cx: &mut Cx, text: &str, target_rect: Rect, options: ChartTooltipOptions) {
        self.show_internal(cx, text, target_rect, options, false);
    }

    fn show_internal(
        &mut self,
        cx: &mut Cx,
        text: &str,
        target_rect: Rect,
        options: ChartTooltipOptions,
        is_internal_redraw: bool,
    ) {
        if !is_internal_redraw {
            self.latest_options = Some((text.to_owned(), target_rect, options.clone()));
        }

        let mut tooltip = self.view.tooltip(ids!(tooltip));
        tooltip.set_text(cx, text);

        // Get tooltip size for positioning
        let tooltip_size = tooltip.view(ids!(rounded_view)).area().rect(cx).size;
        let triangle_height = options.triangle_height;

        // Calculate position based on options
        let (tooltip_pos, callout_angle) = Self::calculate_position(
            &options.position,
            target_rect,
            tooltip_size,
            triangle_height,
        );

        let tooltip_pos_vec = vec2(tooltip_pos.x as f32, tooltip_pos.y as f32);
        let target_pos = vec2(target_rect.pos.x as f32, target_rect.pos.y as f32);
        let target_size = vec2(target_rect.size.x as f32, target_rect.size.y as f32);

        // Hide text if tooltip size not calculated yet
        let mut text_color = options.text_color;
        if tooltip_size.x == 0.0 {
            text_color.w = 0.0;
        }

        tooltip.apply_over(cx, live!(
            content: {
                margin: { left: (tooltip_pos_vec.x), top: (tooltip_pos_vec.y) },
                rounded_view = {
                    draw_bg: {
                        triangle_height: (triangle_height),
                        background_color: (options.bg_color),
                        tooltip_pos: (tooltip_pos_vec),
                        target_pos: (target_pos),
                        target_size: (target_size),
                        expected_dimension_x: (tooltip_size.x),
                        callout_position: (callout_angle),
                    }
                    tooltip_label = {
                        draw_text: { color: (text_color) }
                    }
                }
            }
        ));

        if !is_internal_redraw {
            cx.stop_timer(self.timer_redraw);
            self.timer_redraw = cx.start_timeout(0.05);
        }

        tooltip.show(cx);
    }

    fn calculate_position(
        position: &TooltipPosition,
        target_rect: Rect,
        tooltip_size: DVec2,
        triangle_height: f64,
    ) -> (DVec2, f64) {
        let callout_angle = position.callout_angle();

        let tooltip_pos = match position {
            TooltipPosition::Top => {
                // Center horizontally above target
                DVec2 {
                    x: target_rect.pos.x + target_rect.size.x / 2.0 - tooltip_size.x / 2.0,
                    y: target_rect.pos.y - tooltip_size.y - triangle_height,
                }
            }
            TooltipPosition::Bottom => {
                // Center horizontally below target
                DVec2 {
                    x: target_rect.pos.x + target_rect.size.x / 2.0 - tooltip_size.x / 2.0,
                    y: target_rect.pos.y + target_rect.size.y + triangle_height,
                }
            }
            TooltipPosition::Left => {
                // Center vertically to left of target
                DVec2 {
                    x: target_rect.pos.x - tooltip_size.x - triangle_height,
                    y: target_rect.pos.y + target_rect.size.y / 2.0 - tooltip_size.y / 2.0,
                }
            }
            TooltipPosition::Right => {
                // Center vertically to right of target
                DVec2 {
                    x: target_rect.pos.x + target_rect.size.x + triangle_height,
                    y: target_rect.pos.y + target_rect.size.y / 2.0 - tooltip_size.y / 2.0,
                }
            }
        };

        (tooltip_pos, callout_angle)
    }

    /// Hide the tooltip
    pub fn hide(&mut self, cx: &mut Cx) {
        self.latest_options = None;
        cx.stop_timer(self.timer_redraw);
        self.timer_redraw = Timer::empty();
        self.view.tooltip(ids!(tooltip)).hide(cx);
    }
}

impl ChartCalloutTooltipRef {
    /// Show tooltip with default options (positioned above target)
    pub fn show(&mut self, cx: &mut Cx, text: &str, target_rect: Rect) {
        self.show_with_options(cx, text, target_rect, ChartTooltipOptions::default());
    }

    /// Show tooltip with custom options
    pub fn show_with_options(
        &mut self,
        cx: &mut Cx,
        text: &str,
        target_rect: Rect,
        options: ChartTooltipOptions,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx, text, target_rect, options);
        }
    }

    /// Hide the tooltip
    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }
}
