use cosmic::iced::{
    advanced::{
        graphics::core::{Element, Layout, Rectangle, Size},
        layout::{Limits, Node},
        renderer,
        widget::{Operation, Tree, Widget},
        Shell,
    },
    mouse, Color, Length, Theme,
};
use iced_tiny_skia;
use std::sync::{Arc, Mutex};
use wry::WebView;

pub struct WebViewWidget {
    width: Length,
    height: Length,
    url: String,
    webview: Arc<Mutex<Option<WebView>>>,
}

impl WebViewWidget {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            url: url.into(),
            webview: Arc::new(Mutex::new(None)),
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<Message> Widget<Message, Theme, iced_tiny_skia::Renderer> for WebViewWidget
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &iced_tiny_skia::Renderer,
        limits: &Limits,
    ) -> Node {
        Node::new(limits.resolve(self.width, self.height, Size::ZERO))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced_tiny_skia::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // Initialize webview if not already done
        if self.webview.lock().unwrap().is_none() {
            // This is a simplified approach - in reality you'd need access to the window handle
            // which requires deeper integration with iced's windowing system
            // For now, we'll draw a placeholder
        }

        // Draw placeholder background
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Default::default(),
                shadow: Default::default(),
            },
            Color::from_rgb(0.2, 0.2, 0.2),
        );
    }

    fn operate(
        &self,
        _tree: &mut Tree,
        _layout: Layout<'_>,
        _renderer: &iced_tiny_skia::Renderer,
        _operation: &mut dyn Operation,
    ) {
    }

    fn on_event(
        &mut self,
        _tree: &mut Tree,
        _event: cosmic::iced::Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced_tiny_skia::Renderer,
        _clipboard: &mut dyn cosmic::iced::advanced::Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> cosmic::iced::advanced::graphics::core::event::Status {
        cosmic::iced::advanced::graphics::core::event::Status::Ignored
    }
}

impl<'a, Message> From<WebViewWidget> for Element<'a, Message, Theme, iced_tiny_skia::Renderer>
{
    fn from(widget: WebViewWidget) -> Self {
        Self::new(widget)
    }
}

pub fn webview<Message>(url: impl Into<String>) -> WebViewWidget {
    WebViewWidget::new(url)
}