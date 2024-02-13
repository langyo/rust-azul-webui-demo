use azul::prelude::*;

struct DataModel {
    counter: usize,
}

extern "C" fn layout_func(data: &mut RefAny, _: &mut LayoutCallbackInfo) -> StyledDom {
    let counter = match data.downcast_ref::<DataModel>() {
        Some(d) => format!("{}", d.counter),
        None => return StyledDom::default(),
    };

    let label =
        Dom::text(counter).with_inline_css_props(NodeDataInlineCssPropertyVec::from_vec(vec![
            NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(
                StyleFontSize {
                    inner: PixelValue::em(2.0),
                },
            ))),
            NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(
                StyleTextColor {
                    inner: ColorU {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                },
            ))),
        ]));

    let button = Dom::div()
        .with_inline_css_props(NodeDataInlineCssPropertyVec::from_vec(vec![
            NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(
                StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_vec(vec![
                    StyleBackgroundContent::Color(ColorU {
                        r: 0,
                        g: 0,
                        b: 255,
                        a: 255,
                    }),
                ])),
            )),
            NodeDataInlineCssProperty::Normal(CssProperty::MarginTop(LayoutMarginTopValue::Exact(
                LayoutMarginTop {
                    inner: PixelValue::px(8.0),
                },
            ))),
            NodeDataInlineCssProperty::Normal(CssProperty::MarginLeft(
                LayoutMarginLeftValue::Exact(LayoutMarginLeft {
                    inner: PixelValue::px(8.0),
                }),
            )),
            NodeDataInlineCssProperty::Normal(CssProperty::Width(LayoutWidthValue::Exact(
                LayoutWidth {
                    inner: PixelValue::px(64.0),
                },
            ))),
            NodeDataInlineCssProperty::Normal(CssProperty::Height(LayoutHeightValue::Exact(
                LayoutHeight {
                    inner: PixelValue::px(64.0),
                },
            ))),
        ]))
        .with_callback(
            EventFilter::Hover(HoverEventFilter::MouseUp),
            data.clone(),
            handle_on_click,
        )
        .with_children(DomVec::from_vec(vec![label]));

    Dom::body()
        .with_child(button)
        .with_callback(
            EventFilter::Window(WindowEventFilter::TextInput),
            data.clone(),
            handle_on_click,
        )
        .style(Css::empty())
}

extern "C" fn handle_on_click(data: &mut RefAny, _: &mut CallbackInfo) -> Update {
    println!("+1");
    let mut data = match data.downcast_mut::<DataModel>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    data.counter += 1;

    Update::RefreshDom
}

fn main() {
    let data = DataModel { counter: 0 };
    let config = AppConfig::new(LayoutSolver::Default);
    let app = App::new(RefAny::new(data), config);
    let window = WindowCreateOptions::new(layout_func);
    app.run(window);
}
