//! Display a widget over another.
use crate::widget::Tree;
use crate::{Element, Widget};
use iced_native::event::{self, Event};
use iced_native::layout;
use iced_native::mouse;
use iced_native::overlay;
use iced_native::renderer;
use iced_native::text;
use iced_native::widget::tooltip;
use iced_native::widget::Text;
use iced_native::{Clipboard, Layout, Length, Point, Rectangle, Shell};

pub use iced_style::container::{Style, StyleSheet};
pub use tooltip::Position;

/// An element to display a widget over another.
#[allow(missing_debug_implementations)]
pub struct Tooltip<'a, Message, Renderer: text::Renderer> {
    content: Element<'a, Message, Renderer>,
    tooltip: Text<Renderer>,
    position: Position,
    style_sheet: Box<dyn StyleSheet + 'a>,
    gap: u16,
    padding: u16,
}

impl<'a, Message, Renderer> Tooltip<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    /// The default padding of a [`Tooltip`] drawn by this renderer.
    const DEFAULT_PADDING: u16 = 5;

    /// Creates an empty [`Tooltip`].
    ///
    /// [`Tooltip`]: struct.Tooltip.html
    pub fn new(
        content: impl Into<Element<'a, Message, Renderer>>,
        tooltip: impl ToString,
        position: Position,
    ) -> Self {
        Tooltip {
            content: content.into(),
            tooltip: Text::new(tooltip.to_string()),
            position,
            style_sheet: Default::default(),
            gap: 0,
            padding: Self::DEFAULT_PADDING,
        }
    }

    /// Sets the size of the text of the [`Tooltip`].
    pub fn size(mut self, size: u16) -> Self {
        self.tooltip = self.tooltip.size(size);
        self
    }

    /// Sets the font of the [`Tooltip`].
    ///
    /// [`Font`]: Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.tooltip = self.tooltip.font(font);
        self
    }

    /// Sets the gap between the content and its [`Tooltip`].
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the padding of the [`Tooltip`].
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the style of the [`Tooltip`].
    pub fn style(
        mut self,
        style_sheet: impl Into<Box<dyn StyleSheet + 'a>>,
    ) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Tooltip<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            inherited_style,
            layout,
            cursor_position,
            viewport,
        );

        let tooltip = &self.tooltip;

        tooltip::draw(
            renderer,
            inherited_style,
            layout,
            cursor_position,
            viewport,
            self.position,
            self.gap,
            self.padding,
            self.style_sheet.as_ref(),
            |renderer, limits| {
                Widget::<(), Renderer>::layout(tooltip, renderer, limits)
            },
            |renderer, defaults, layout, cursor_position, viewport| {
                Widget::<(), Renderer>::draw(
                    tooltip,
                    &Tree::empty(),
                    renderer,
                    defaults,
                    layout,
                    cursor_position,
                    viewport,
                );
            },
        );
    }

    fn overlay<'b>(
        &'b self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content.as_widget().overlay(
            &mut tree.children[0],
            layout,
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Tooltip<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + text::Renderer,
    Message: 'a,
{
    fn from(
        tooltip: Tooltip<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(tooltip)
    }
}
