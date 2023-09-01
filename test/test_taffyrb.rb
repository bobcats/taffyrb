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
      size: {
        width_pts: 100,
        height_pts: 100
      }
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
        size: {width_pts: 800, height_pts: 100}
      )
    )

    body_node = taffy.new_leaf(
      Taffy::Style.new(
        size: {width_pts: 800, height_auto: true},
        flex_grow: 1.0
      )
    )

    root_node = taffy.new_leaf(
      Taffy::Style.new(
        size: {width_pts: 800, height_pts: 600},
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
end
