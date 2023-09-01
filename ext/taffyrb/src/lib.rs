use magnus::scan_args::{get_kwargs, scan_args};
use magnus::{class, define_class, function, method, prelude::*, Error, Value};
use taffy::prelude::*;
use taffy::style::Dimension;

#[magnus::wrap(class = "Taffy", free_immediately, size)]
struct TaffyRB(std::cell::RefCell<Taffy>);

impl TaffyRB {
    fn new() -> Self {
        Self(std::cell::RefCell::new(Taffy::new()))
    }

    fn new_leaf(&self, style: &TaffyRBStyle) -> TaffyRBNode {
        let mut inner = self.0.borrow_mut();
        let node = inner.new_leaf(style.0.borrow().clone()).unwrap();
        TaffyRBNode(node)
    }

    fn add_child(&self, parent: &TaffyRBNode, child: &TaffyRBNode) {
        let mut inner = self.0.borrow_mut();
        inner.add_child(parent.0, child.0).unwrap();
    }

    fn child_count(&self, parent: &TaffyRBNode) -> usize {
        let inner = self.0.borrow();
        inner.child_count(parent.0).unwrap()
    }

    fn compute_layout(&self, node: &TaffyRBNode, width: isize, height: isize) {
        let size = Size {
            width: Self::available_space(width),
            height: Self::available_space(height),
        };
        let inner = &self.0;
        inner.borrow_mut().compute_layout(node.0, size).unwrap();
    }

    fn available_space(num: isize) -> AvailableSpace {
        if num == -1 {
            AvailableSpace::MaxContent
        } else {
            AvailableSpace::Definite(num as f32)
        }
    }

    fn layout(&self, node: &TaffyRBNode) -> TaffyRBLayout {
        let inner = self.0.borrow();
        TaffyRBLayout(*inner.layout(node.0).unwrap())
    }
}

#[magnus::wrap(class = "Taffy::Layout", free_immediately, size)]
struct TaffyRBLayout(Layout);

impl TaffyRBLayout {
    fn size(&self) -> (f32, f32) {
        (self.0.size.width, self.0.size.height)
    }
}

#[magnus::wrap(class = "Taffy::SizeDimension", free_immediately, size)]
struct TaffyRBSizeDimension(Size<Dimension>);

impl TaffyRBSizeDimension {
    fn new(width: &TaffyRBDimension, height: &TaffyRBDimension) -> Self {
        Self(Size {
            width: width.0,
            height: height.0,
        })
    }
}

#[magnus::wrap(class = "Taffy::SizeLengthPercentage", free_immediately, size)]
struct TaffyRBSizeLengthPercentage(Size<LengthPercentage>);

impl TaffyRBSizeLengthPercentage {
    fn new(width: &TaffyRBLengthPercentage, height: &TaffyRBLengthPercentage) -> Self {
        Self(Size {
            width: width.0,
            height: height.0,
        })
    }
}

#[magnus::wrap(class = "Taffy::LengthPercentage", free_immediately, size)]
struct TaffyRBLengthPercentage(LengthPercentage);

fn length_percentage_percent(value: f32) -> TaffyRBLengthPercentage {
    TaffyRBLengthPercentage(LengthPercentage::Percent(value))
}

fn length_percentage_points(value: f32) -> TaffyRBLengthPercentage {
    TaffyRBLengthPercentage(LengthPercentage::Points(value))
}

#[magnus::wrap(class = "Taffy::Dimension", free_immediately, size)]
struct TaffyRBDimension(Dimension);

fn dimension_percent(value: f32) -> TaffyRBDimension {
    TaffyRBDimension(Dimension::Percent(value))
}

fn dimension_points(value: f32) -> TaffyRBDimension {
    TaffyRBDimension(Dimension::Points(value))
}

fn auto() -> TaffyRBDimension {
    TaffyRBDimension(Dimension::Auto)
}

#[magnus::wrap(class = "Taffy::Style", free_immediately, size)]
#[derive(Clone, Debug, Default)]
struct TaffyRBStyle(std::cell::RefCell<Style>);

impl TaffyRBStyle {
    fn initialize(rb_self: magnus::typed_data::Obj<Self>, args: &[Value]) -> Result<(), Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let kwargs = get_kwargs::<
            _,
            (),
            (
                Option<&TaffyRBDisplay>,
                Option<&TaffyRBSizeDimension>,
                Option<&TaffyRBFlexDirection>,
                Option<f32>,
                Option<&TaffyRBJustifyContent>,
                Option<&TaffyRBSizeLengthPercentage>,
            ),
            (),
        >(
            args.keywords,
            &[],
            &[
                "display",
                "size",
                "flex_direction",
                "flex_grow",
                "justify_content",
                "gap",
            ],
        )?;

        if let Some(display) = kwargs.optional.0 {
            rb_self.0.borrow_mut().display = display.0;
        }

        if let Some(size) = kwargs.optional.1 {
            rb_self.0.borrow_mut().size = size.0;
        }

        if let Some(flex_direction) = kwargs.optional.2 {
            rb_self.0.borrow_mut().flex_direction = flex_direction.0;
        }

        if let Some(flex_grow) = kwargs.optional.3 {
            rb_self.0.borrow_mut().flex_grow = flex_grow;
        }

        if let Some(justify_content) = kwargs.optional.4 {
            rb_self.0.borrow_mut().justify_content = justify_content.0;
        }

        if let Some(gap) = kwargs.optional.5 {
            rb_self.0.borrow_mut().gap = gap.0;
        }

        Ok(())
    }
}

#[magnus::wrap(class = "Taffy::Node", free_immediately, size)]
struct TaffyRBNode(Node);

#[magnus::wrap(class = "Taffy::Display", free_immediately, size)]
struct TaffyRBDisplay(Display);

fn display_flex() -> TaffyRBDisplay {
    TaffyRBDisplay(Display::Flex)
}

#[magnus::wrap(class = "Taffy::FlexDirection", free_immediately, size)]
struct TaffyRBFlexDirection(FlexDirection);

fn flex_direction_row() -> TaffyRBFlexDirection {
    TaffyRBFlexDirection(FlexDirection::Row)
}
fn flex_direction_row_reverse() -> TaffyRBFlexDirection {
    TaffyRBFlexDirection(FlexDirection::RowReverse)
}
fn flex_direction_column() -> TaffyRBFlexDirection {
    TaffyRBFlexDirection(FlexDirection::Column)
}
fn flex_direction_column_reverse() -> TaffyRBFlexDirection {
    TaffyRBFlexDirection(FlexDirection::ColumnReverse)
}

#[magnus::wrap(class = "Taffy::JustifyContent", free_immediately, size)]
struct TaffyRBJustifyContent(Option<JustifyContent>);

fn justify_content_flex_start() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(Some(JustifyContent::FlexStart))
}
fn justify_content_flex_end() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(Some(JustifyContent::FlexEnd))
}
fn justify_content_center() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(Some(JustifyContent::Center))
}
fn justify_content_space_between() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(Some(JustifyContent::SpaceBetween))
}
fn justify_content_space_around() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(Some(JustifyContent::SpaceAround))
}
fn justify_content_space_evenly() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(Some(JustifyContent::SpaceEvenly))
}
fn justify_content_none() -> TaffyRBJustifyContent {
    TaffyRBJustifyContent(None)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let klass = define_class("Taffy", class::object())?;
    klass.define_singleton_method("new", function!(TaffyRB::new, 0))?;
    klass.define_method("new_leaf", method!(TaffyRB::new_leaf, 1))?;
    klass.define_method("child_count", method!(TaffyRB::child_count, 1))?;
    klass.define_method("add_child", method!(TaffyRB::add_child, 2))?;
    klass.define_method("compute_layout", method!(TaffyRB::compute_layout, 3))?;
    klass.define_method("layout", method!(TaffyRB::layout, 1))?;

    let style_klass = klass.define_class("Style", class::object())?;
    style_klass.define_alloc_func::<TaffyRBStyle>();
    style_klass.define_method("initialize", method!(TaffyRBStyle::initialize, -1))?;

    klass.define_class("Node", class::object())?;
    let layout_klass = klass.define_class("Layout", class::object())?;
    layout_klass.define_method("size", method!(TaffyRBLayout::size, 0))?;

    let size_klass = klass.define_class("Size", class::object())?;
    size_klass.define_singleton_method("dimension", function!(TaffyRBSizeDimension::new, 2))?;
    klass.define_class("SizeDimension", class::object())?;

    size_klass.define_singleton_method(
        "length_percentage",
        function!(TaffyRBSizeLengthPercentage::new, 2),
    )?;
    klass.define_class("SizeLengthPercentage", class::object())?;

    let dimension_class = klass.define_class("Dimension", class::object())?;
    dimension_class.define_singleton_method("percent", function!(dimension_percent, 1))?;
    dimension_class.define_singleton_method("length", function!(dimension_points, 1))?;
    dimension_class.define_singleton_method("auto", function!(auto, 0))?;

    let length_percentage_class = klass.define_class("LengthPercentage", class::object())?;
    length_percentage_class
        .define_singleton_method("percent", function!(length_percentage_percent, 1))?;
    length_percentage_class
        .define_singleton_method("length", function!(length_percentage_points, 1))?;

    let display_klass = klass.define_class("Display", class::object())?;
    display_klass.define_singleton_method("flex", function!(display_flex, 0))?;

    let flex_direction_klass = klass.define_class("FlexDirection", class::object())?;
    flex_direction_klass.define_singleton_method("row", function!(flex_direction_row, 0))?;
    flex_direction_klass
        .define_singleton_method("row_reverse", function!(flex_direction_row_reverse, 0))?;
    flex_direction_klass.define_singleton_method("column", function!(flex_direction_column, 0))?;
    flex_direction_klass.define_singleton_method(
        "column_reverse",
        function!(flex_direction_column_reverse, 0),
    )?;

    let justify_content_klass = klass.define_class("JustifyContent", class::object())?;
    justify_content_klass
        .define_singleton_method("flex_start", function!(justify_content_flex_start, 0))?;
    justify_content_klass
        .define_singleton_method("flex_end", function!(justify_content_flex_end, 0))?;
    justify_content_klass
        .define_singleton_method("center", function!(justify_content_center, 0))?;
    justify_content_klass
        .define_singleton_method("space_between", function!(justify_content_space_between, 0))?;
    justify_content_klass
        .define_singleton_method("space_around", function!(justify_content_space_around, 0))?;
    justify_content_klass
        .define_singleton_method("space_evenly", function!(justify_content_space_evenly, 0))?;
    justify_content_klass.define_singleton_method("none", function!(justify_content_none, 0))?;

    Ok(())
}
