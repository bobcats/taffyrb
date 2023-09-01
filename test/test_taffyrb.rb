# frozen_string_literal: true

require "test_helper"

class TestTaffyrb < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Taffyrb::VERSION
  end

  def test_it_does_something_useful
    taffy = Taffy.new
    root_style = Taffy::Style.new(
      display: Taffy::Display.flex,
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

    taffy.compute_layout(root_node, -1, -1)
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
        flex_direction: Taffy::FlexDirection.column,
        display: Taffy::Display.flex
      )
    )

    taffy.add_child(root_node, header_node)
    taffy.add_child(root_node, body_node)

    taffy.compute_layout(root_node, 800, -1)

    assert_equal(taffy.layout(root_node).size, [800, 600])
    assert_equal(taffy.layout(header_node).size, [800, 100])
    assert_equal(taffy.layout(body_node).size, [800, 500])
  end

  def test_basic
    taffy = Taffy.new
    child = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(Taffy::Dimension.percent(0.5), Taffy::Dimension.auto)
      )
    )
    node = taffy.new_leaf(
      Taffy::Style.new(
        size: Taffy::Size.dimension(Taffy::Dimension.length(100), Taffy::Dimension.length(100)),
        justify_content: Taffy::JustifyContent.flex_start
      )
    )
    taffy.add_child(node, child)

    taffy.compute_layout(node, 100, 100)
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
        justify_content: Taffy::JustifyContent.center
      )
    )
    taffy.add_child(node, child)

    taffy.compute_layout(node, -1, -1)
    assert_equal [100, 100], taffy.layout(node).size
    assert_equal [50, 100], taffy.layout(child).size
  end

  def test_flex_box_gap
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

    taffy.compute_layout(root, -1, -1)

    assert_equal [80, 20], taffy.layout(root).size
    assert_equal [20, 20], taffy.layout(child0).size
    assert_equal [20, 20], taffy.layout(child1).size
    assert_equal [20, 20], taffy.layout(child2).size
  end
end
