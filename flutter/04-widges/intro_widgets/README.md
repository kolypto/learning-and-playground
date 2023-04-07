# Widgets

## Basic Widgets

* `Text`: styled text
* `Center`: centers an element in the middle of the screen
* `Row`, `Column`: flexbox: horizontal and vertical layout directions.
* `Stack`: lets you place widgets on top of one another. Use `Positioned` to move them
* `Container`: rectangular visual element, with margins and padding, with constraints, with `BoxDecoration` to color, border, shadow.
  Matrix 3D transformations are possible too.

## Layout

### Row, Column

`Row` and `Column` occupy different main axes. A `Row`'s main axis is horizontal, and a `Column`'s main axis is vertical.

The `mainAxisSize` determines how much space a `Row`/`Column` can occupy on their main axis.
Two possible values: `min` (just enough) and `max` (default; all of it).

The `mainAxisAlignment` determines how children are positioned in extra space:
`start`, `end`, `center`, `spaceBetween` (evenly between), `spaceEvenly` (evenly, before and after too), `spaceAround` (evenly, reduce space before/after by half).

The `crossAxisAlignment` determins positioning on the cross axis:
`start`, `end`, `center`, `stretch` (stretch children across), `baseline` (aligns by baseline).

### Flexible, Expanded

`Flexible` wraps a widget so it becomes resizable.
Widgets are resized according to:

* `flex`: the fraction of remaining space it gets
* `fit`: whether the widget fills all of its extra space: `loose` or `tight`

`Expanded` wraps a widget and fills extra space.

### SizedBox, Spacer

`SizedBox` resizes a widget to a width and height.
When no widget is provided, it creates empty space.

`Spacer` can also create empty space: it uses "flex", not px.

## Layout Widgets

Standard widgets:

* `Container`: adds paddings/margins, borders, background color, ...
* `GridView`: scrollable grid (table)
* `ListView`: scrollable list. Use `ListTile` for children
* `Stack`: overlaps widgets on top of another, use `alignment:` to specify how
