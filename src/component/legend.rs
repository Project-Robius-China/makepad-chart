use makepad_widgets::*;
use crate::core::get_color;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub DrawLegendBox = {{DrawLegendBox}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);

            sdf.box(
                1.0,
                1.0,
                self.rect_size.x - 2.0,
                self.rect_size.y - 2.0,
                2.0
            );

            sdf.fill(self.color);

            return sdf.result;
        }
    }

    LegendItem = <View> {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 6,
        align: { y: 0.5 }

        color_box = <View> {
            width: 12, height: 12
            show_bg: true
            draw_bg: {
                instance color: #ff0000
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 2.0);
                    sdf.fill(self.color);
                    return sdf.result;
                }
            }
        }

        label = <Label> {
            width: Fit, height: Fit
            draw_text: {
                text_style: { font_size: 11.0 },
                color: #a0a0a0
            }
        }
    }

    pub ChartLegend = {{ChartLegend}} {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 16,
        padding: {top: 8, bottom: 8},
        align: {x: 0.5, y: 0.5},

        legend_item = <LegendItem> {}
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawLegendBox {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
}

impl DrawLegendBox {
    pub fn draw_box(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_abs(cx, rect);
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChartLegend {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_box: DrawLegendBox,

    #[rust]
    items: Vec<LegendItemData>,
}

#[derive(Clone, Debug)]
pub struct LegendItemData {
    pub label: String,
    pub color: Vec4,
    pub hidden: bool,
}

impl Widget for ChartLegend {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Use flow layout for items
        cx.begin_turtle(walk, Layout {
            flow: Flow::right(),
            spacing: 16.0,
            padding: Padding { left: 8.0, top: 4.0, right: 8.0, bottom: 4.0 },
            align: Align { x: 0.5, y: 0.5 },
            ..Default::default()
        });

        // Draw each legend item using the template
        for item in &self.items {
            // Create the legend item from template
            let legend_item = self.view.view(ids!(legend_item));

            // Set the color on the color box
            legend_item.view(ids!(color_box)).apply_over(cx, live! {
                draw_bg: { color: (item.color) }
            });

            // Set the label text
            legend_item.label(ids!(label)).set_text(cx, &item.label);

            // Draw the legend item
            let _ = legend_item.draw_walk(cx, scope, Walk::fit());
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl ChartLegend {
    pub fn set_items(&mut self, items: Vec<LegendItemData>) {
        self.items = items;
    }

    pub fn set_items_from_labels(&mut self, labels: &[String]) {
        self.items = labels.iter().enumerate().map(|(i, label)| {
            LegendItemData {
                label: label.clone(),
                color: get_color(i),
                hidden: false,
            }
        }).collect();
    }

    pub fn set_items_from_datasets(&mut self, labels: &[String], colors: &[Option<Vec4>]) {
        self.items = labels.iter().enumerate().map(|(i, label)| {
            LegendItemData {
                label: label.clone(),
                color: colors.get(i).and_then(|c| *c).unwrap_or_else(|| get_color(i)),
                hidden: false,
            }
        }).collect();
    }
}

impl ChartLegendRef {
    pub fn set_items(&self, items: Vec<LegendItemData>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_items(items);
        }
    }

    pub fn set_items_from_labels(&self, labels: &[String]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_items_from_labels(labels);
        }
    }
}
