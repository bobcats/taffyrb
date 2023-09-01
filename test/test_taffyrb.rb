# frozen_string_literal: true

require "test_helper"

class TestTaffyrb < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Taffyrb::VERSION
  end

  def test_it_does_something_useful
    taffy = Taffy.new
    root_style = Taffy::Style.new(
      display: :flex,
      size: Taffy::Size.dimension(
        Taffy::Dimension.length(100),
        Taffy::Dimension.length(100)
      )
    )

    root_node = taffy.new_leaf(root_style)
    another_node = taffy.new_leaf(root_style)
    grandchild_node = taffy.new_leaf(root_style)
    taffy.add_child(root_node, grandchild_node)
    taffy.add_child(root_node, another_node)
    assert_equal 2, taffy.child_count(root_node)

    taffy.compute_layout(root_node, {width: :auto, height: :auto})
    layout = taffy.layout(root_node)
    assert_equal [100, 100], layout.size

    layout = taffy.layout(another_node)
    assert_equal [50, 100.00], layout.size
  end

  def test_readme
    taffy = Taffy.new
    header_node = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(
          Taffy::Dimension.length(800),
          Taffy::Dimension.length(100)
        )
      )
    )

    body_node = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(
          Taffy::Dimension.length(800),
          Taffy::Dimension.auto
        ),
        flex_grow: 1.0
      )
    )

    root_node = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(
          Taffy::Dimension.length(800),
          Taffy::Dimension.length(600)
        ),
        flex_direction: :column,
        display: :flex
      )
    )

    taffy.add_child(root_node, header_node)
    taffy.add_child(root_node, body_node)

    taffy.compute_layout(root_node, {width: 800, height: :auto})

    assert_equal(taffy.layout(root_node).size, [800, 600])
    assert_equal(taffy.layout(header_node).size, [800, 100])
    assert_equal(taffy.layout(body_node).size, [800, 500])
  end

  def test_basic
    # let mut taffy = Taffy::new();
    #
    # let child = taffy.new_leaf(Style {
    #     size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
    #     ..Default::default()
    # })?;
    #
    # let node = taffy.new_with_children(
    #     Style {
    #         size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
    #         justify_content: Some(JustifyContent::Center),
    #         ..Default::default()
    #     },
    #     &[child],
    # )?;
    #
    # println!("Compute layout with 100x100 viewport:");
    # taffy.compute_layout(
    #     node,
    #     Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    # )?;
    # println!("node: {:#?}", taffy.layout(node)?);
    # println!("child: {:#?}", taffy.layout(child)?);
    #
    # println!("Compute layout with undefined (infinite) viewport:");
    # taffy.compute_layout(node, Size::MAX_CONTENT)?;
    # println!("node: {:#?}", taffy.layout(node)?);
    # println!("child: {:#?}", taffy.layout(child)?);
    # Ok(())

    taffy = Taffy.new
    child = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(Taffy::Dimension.percent(0.5), Taffy::Dimension.auto)
      )
    )
    node = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(Taffy::Dimension.length(100), Taffy::Dimension.length(100)),
        justify_content: :center
      )
    )
    taffy.add_child(node, child)

    taffy.compute_layout(node, {width: 100, height: 100})
    assert_equal [100, 100], taffy.layout(node).size
    assert_equal [50, 100], taffy.layout(child).size

    taffy = Taffy.new
    child = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(Taffy::Dimension.percent(0.5), Taffy::Dimension.auto)
      )
    )
    node = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(Taffy::Dimension.length(100), Taffy::Dimension.length(100)),
        justify_content: :center
      )
    )
    taffy.add_child(node, child)

    taffy.compute_layout(node, {width: :auto, height: :auto})
    assert_equal [100, 100], taffy.layout(node).size
    assert_equal [50, 100], taffy.layout(child).size
  end

  def test_flex_box_gap
    # let mut taffy = Taffy::new();
    #
    # let child_style = Style { size: Size { width: points(20.0), height: points(20.0) }, ..Default::default() };
    # let child0 = taffy.new_leaf(child_style.clone())?;
    # let child1 = taffy.new_leaf(child_style.clone())?;
    # let child2 = taffy.new_leaf(child_style.clone())?;
    #
    # let root = taffy.new_with_children(
    #     Style { gap: Size { width: points(10.0), height: zero() }, ..Default::default() },
    #     &[child0, child1, child2],
    # )?;
    #
    # // Compute layout and print result
    # taffy.compute_layout(root, Size::MAX_CONTENT)?;
    # taffy::debug::print_tree(&taffy, root);
    #
    # Ok(())

    taffy = Taffy.new
    child_style = Taffy::Style.new(
      size: Taffy::Size.dimension(Taffy::Dimension.length(20), Taffy::Dimension.length(20))
    )
    child0 = taffy.new_leaf(child_style)
    child1 = taffy.new_leaf(child_style)
    child2 = taffy.new_leaf(child_style)

    root = taffy.new_leaf(
      Taffy::Style.new(
        gap: Taffy::Size.length_percentage(
          Taffy::LengthPercentage.length(10),
          Taffy::LengthPercentage.length(0)
        )
      )
    )

    taffy.add_child(root, child0)
    taffy.add_child(root, child1)
    taffy.add_child(root, child2)

    taffy.compute_layout(root, {width: :auto, height: :auto})
    assert_equal [80, 20], taffy.layout(root).size
    assert_equal [20, 20], taffy.layout(child0).size
    assert_equal [20, 20], taffy.layout(child1).size
    assert_equal [20, 20], taffy.layout(child2).size
  end
end
