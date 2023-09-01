use magnus::scan_args::{get_kwargs, scan_args};
use magnus::{class, define_class, exception, function, method, prelude::*, Error, Value};
use magnus::{Symbol, TryConvert};
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

    fn compute_layout(&self, node: &TaffyRBNode, size: Value) {
        let size = TaffyRBSize::<AvailableSpace>::try_convert(size).unwrap();
        let inner = &self.0;
        inner.borrow_mut().compute_layout(node.0, size.0).unwrap();
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

struct TaffyRBSize<T>(Size<T>);

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

fn dimension_auto() -> TaffyRBDimension {
    TaffyRBDimension(Dimension::Auto)
}

impl TryConvert for TaffyRBSize<Dimension> {
    fn try_convert(value: magnus::Value) -> Result<Self, Error> {
        let value = magnus::RHash::from_value(value).unwrap();

        let mut size = Size {
            width: Dimension::Auto,
            height: Dimension::Auto,
        };

        if let Ok(width) = value.fetch::<_, f32>(Symbol::new("width_pts")) {
            size.width = points(width);
        }

        if let Ok(height) = value.fetch::<_, f32>(Symbol::new("height_pts")) {
            size.height = points(height);
        }

        if let Ok(width) = value.fetch::<_, f32>(Symbol::new("width_pct")) {
            size.width = percent(width);
        }

        if let Ok(height) = value.fetch::<_, f32>(Symbol::new("height_pct")) {
            size.height = percent(height);
        }

        if value
            .fetch::<_, bool>(Symbol::new("width_auto"))
            .ok()
            .is_some()
        {
            size.width = Dimension::Auto;
        }

        if value
            .fetch::<_, bool>(Symbol::new("height_auto"))
            .ok()
            .is_some()
        {
            size.height = Dimension::Auto;
        }

        Ok(Self(size))
    }
}

impl TryConvert for TaffyRBSize<LengthPercentage> {
    fn try_convert(value: magnus::Value) -> Result<Self, Error> {
        let value = magnus::RHash::from_value(value).unwrap();

        let mut size = Size {
            width: zero(),
            height: zero(),
        };

        if let Ok(width) = value.fetch::<_, f32>(Symbol::new("width_pts")) {
            size.width = points(width);
        }

        if let Ok(height) = value.fetch::<_, f32>(Symbol::new("height_pts")) {
            size.height = points(height);
        }

        if let Ok(width) = value.fetch::<_, f32>(Symbol::new("width_pct")) {
            size.width = percent(width);
        }

        if let Ok(height) = value.fetch::<_, f32>(Symbol::new("height_pct")) {
            size.height = percent(height);
        }

        Ok(Self(size))
    }
}

impl TryConvert for TaffyRBSize<AvailableSpace> {
    fn try_convert(value: magnus::Value) -> Result<Self, Error> {
        let value = magnus::RHash::from_value(value).unwrap();

        let mut size = Size {
            width: AvailableSpace::MaxContent,
            height: AvailableSpace::MaxContent,
        };

        if let Ok(width) = value.fetch::<_, f32>(Symbol::new("width")) {
            size.width = AvailableSpace::Definite(width);
        }

        if let Ok(height) = value.fetch::<_, f32>(Symbol::new("height")) {
            size.height = AvailableSpace::Definite(height);
        }

        Ok(Self(size))
    }
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
                Option<TaffyRBDisplay>,
                Option<&TaffyRBSizeDimension>,
                Option<TaffyRBFlexDirection>,
                Option<f32>,
                Option<TaffyRBJustifyContent>,
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

impl TryConvert for TaffyRBDisplay {
    fn try_convert(value: magnus::Value) -> Result<Self, Error> {
        let value = Symbol::from_value(value).unwrap().name()?;

        let display = match value.into_owned().as_str() {
            "flex" => Display::Flex,
            "grid" => Display::Grid,
            "none" => Display::None,
            _ => return Err(Error::new(exception::arg_error(), "no good")),
        };
        Ok(Self(display))
    }
}

#[magnus::wrap(class = "Taffy::FlexDirection", free_immediately, size)]
struct TaffyRBFlexDirection(FlexDirection);

impl TryConvert for TaffyRBFlexDirection {
    fn try_convert(value: magnus::Value) -> Result<Self, Error> {
        let value = Symbol::from_value(value).unwrap().name()?;

        let direction = match value.into_owned().as_str() {
            "row" => FlexDirection::Row,
            "row-reverse" => FlexDirection::RowReverse,
            "column" => FlexDirection::Column,
            "column-reverse" => FlexDirection::ColumnReverse,
            _ => return Err(Error::new(exception::arg_error(), "no good")),
        };
        Ok(Self(direction))
    }
}

#[magnus::wrap(class = "Taffy::JustifyContent", free_immediately, size)]
struct TaffyRBJustifyContent(Option<JustifyContent>);

impl TryConvert for TaffyRBJustifyContent {
    fn try_convert(value: magnus::Value) -> Result<Self, Error> {
        let value = Symbol::from_value(value).unwrap().name()?;

        let justify = match value.into_owned().as_str() {
            "flex-start" => Some(JustifyContent::FlexStart),
            "flex-end" => Some(JustifyContent::FlexEnd),
            "center" => Some(JustifyContent::Center),
            "space-between" => Some(JustifyContent::SpaceBetween),
            "space-around" => Some(JustifyContent::SpaceAround),
            "space-evenly" => Some(JustifyContent::SpaceEvenly),
            "none" => None,
            _ => return Err(Error::new(exception::arg_error(), "no good")),
        };
        Ok(Self(justify))
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let klass = define_class("Taffy", class::object())?;
    klass.define_singleton_method("new", function!(TaffyRB::new, 0))?;
    klass.define_method("new_leaf", method!(TaffyRB::new_leaf, 1))?;
    klass.define_method("child_count", method!(TaffyRB::child_count, 1))?;
    klass.define_method("add_child", method!(TaffyRB::add_child, 2))?;
    klass.define_method("compute_layout", method!(TaffyRB::compute_layout, 2))?;
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

    let dimension_mod = klass.define_class("Dimension", class::object())?;
    dimension_mod.define_singleton_method("percent", function!(dimension_percent, 1))?;
    dimension_mod.define_singleton_method("length", function!(dimension_points, 1))?;
    dimension_mod.define_singleton_method("auto", function!(dimension_auto, 0))?;

    let length_percentage_mod = klass.define_class("LengthPercentage", class::object())?;
    length_percentage_mod
        .define_singleton_method("percent", function!(length_percentage_percent, 1))?;
    length_percentage_mod
        .define_singleton_method("length", function!(length_percentage_points, 1))?;

    Ok(())
}
