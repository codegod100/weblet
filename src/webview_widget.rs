use cosmic::iced::{
    advanced::{
        graphics::core::{Element, Layout, Rectangle, Size},
        layout::{Limits, Node},
        renderer::{self, Renderer},
        widget::{Operation, Tree, Widget},
        Shell,
        text,
    },
    mouse, Color, Length, Font, Pixels,
};
use cosmic::iced_core::text::Renderer as TextRenderer;
use std::sync::{Arc, Mutex};
use wry::WebView;

// Import the exact renderer type from cosmic's dependency tree
use iced_tiny_skia;

pub struct WebViewWidget {
    width: Length,
    height: Length,
    url: String,
    webview: Arc<Mutex<Option<WebView>>>,
    initialized: Arc<Mutex<bool>>,
}

impl WebViewWidget {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            url: url.into(),
            webview: Arc::new(Mutex::new(None)),
            initialized: Arc::new(Mutex::new(false)),
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

impl<Message> Widget<Message, cosmic::Theme, iced_tiny_skia::Renderer> for WebViewWidget
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
        _theme: &cosmic::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // For now, let's render HTML-like content using text and shapes
        // This simulates what the HTML would look like
        
        // Draw dark background (simulating the CSS background:#333)
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Default::default(),
                shadow: Default::default(),
            },
            Color::from_rgb(0.2, 0.2, 0.2), // #333 background
        );

        // Parse the HTML content and render text
        let html_content = &self.url;
        if html_content.starts_with("data:text/html,") {
            let html = &html_content[15..]; // Remove "data:text/html," prefix
            
            // Simple HTML parsing - extract text from <body>
            if let Some(body_start) = html.find("<body") {
                if let Some(body_content_start) = html[body_start..].find('>') {
                    let body_content = &html[body_start + body_content_start + 1..];
                    if let Some(body_end) = body_content.find("</body>") {
                        let text_content = &body_content[..body_end];
                        
                        // Render the text content
                        let text_bounds = Rectangle {
                            x: layout.bounds().x + 8.0, // padding from CSS
                            y: layout.bounds().y + 8.0,
                            width: layout.bounds().width - 16.0,
                            height: layout.bounds().height - 16.0,
                        };
                        
                        // Draw white text (from CSS color:white)
                        renderer.fill_text(text::Text {
                            content: text_content.to_string(),
                            bounds: text_bounds.size(),
                            size: Pixels(12.0), // font-size:12px from CSS
                            line_height: text::LineHeight::default(),
                            font: Font::default(),
                            horizontal_alignment: cosmic::iced::alignment::Horizontal::Left,
                            vertical_alignment: cosmic::iced::alignment::Vertical::Top,
                            shaping: text::Shaping::default(),
                            wrapping: text::Wrapping::default(),
                        }, text_bounds.position(), Color::WHITE, text_bounds); // color:white from CSS
                    }
                }
            }
        }
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

impl<'a, Message> From<WebViewWidget> for Element<'a, Message, cosmic::Theme, iced_tiny_skia::Renderer>
{
    fn from(widget: WebViewWidget) -> Self {
        Self::new(widget)
    }
}

pub fn webview<Message>(url: impl Into<String>) -> WebViewWidget {
    WebViewWidget::new(url)
}