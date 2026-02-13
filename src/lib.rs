use iced::{
    Border, Element, color,
    widget::{
        self, Float, MouseArea, Scrollable,
        container::{self, Container},
        tooltip::{Position, Tooltip},
    },
};

// sensor
// selector, feature flagged "selector"

pub trait IcedExt<'a, W, M>
where
    W: Into<Element<'a, M>>,
{
    /// consumes and wraps self in a tooltip
    fn tooltip(self, tooltip: impl Into<Element<'a, M>>, pos: Position) -> Tooltip<'a, M>;

    /// consumes and wraps self in a container
    fn container(self) -> Container<'a, M>;

    /// consumes and wraps self in a mousearea
    fn mouse_area(self) -> MouseArea<'a, M>;

    /// consumes and wraps self in a hover element
    fn hover(self, hover: impl Into<Element<'a, M>>) -> iced::Element<'a, M>
    where
        M: 'a;

    /// consumes and wraps self in a scrollable
    fn scrollale(self) -> Scrollable<'a, M>;

    /// consumes and wraps self in a float
    fn float(self) -> Float<'a, M>;

    /// consumes and wraps self in a opaque element
    fn opaque(self) -> Element<'a, M>;

    /// consumes and wraps self in container with a red border.
    fn dbg(self) -> iced::Element<'a, M>
    where
        M: 'a;

    /// consumes self and returns Some(widget) if the predicate is true, else None.
    fn show_if(self, pred: bool) -> Option<W>;
    /// consumes self and returns None if the predicate is true, else Some(widget)
    fn hide_if(self, pred: bool) -> Option<W>;
}

impl<'a, W, M> IcedExt<'a, W, M> for W
where
    W: Into<Element<'a, M>>,
{
    fn tooltip(self, tooltip: impl Into<Element<'a, M>>, pos: Position) -> Tooltip<'a, M> {
        widget::tooltip(self, tooltip, pos)
    }

    fn container(self) -> Container<'a, M> {
        widget::container(self)
    }

    fn mouse_area(self) -> MouseArea<'a, M> {
        widget::mouse_area(self)
    }

    fn hover(self, hover: impl Into<Element<'a, M>>) -> iced::Element<'a, M>
    where
        M: 'a,
    {
        widget::hover(self, hover)
    }

    fn scrollale(self) -> Scrollable<'a, M> {
        widget::scrollable(self)
    }

    fn float(self) -> Float<'a, M> {
        widget::float(self)
    }

    fn opaque(self) -> Element<'a, M> {
        self.into()
    }

    fn dbg(self) -> iced::Element<'a, M>
    where
        M: 'a,
    {
        widget::container(self)
            .style(|_| container::Style {
                border: Border {
                    color: color!(0xff0000),
                    width: 1.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    fn show_if(self, pred: bool) -> Option<W> {
        if pred { Some(self) } else { None }
    }
    fn hide_if(self, pred: bool) -> Option<W> {
        if !pred { Some(self) } else { None }
    }
}
