// use crate::{
//     event::{Key, KeyDownHandler},
//     properties::*,
//     structs::{Position, Size, Spacer},
//     styling::{colors, fonts},
//     theme::Selector,
//     widget::{
//         Container, Context, Cursor, Grid, Property, ScrollViewer, State, Template,
//         WaterMarkTextBox, Widget,
//     },
// };

// /// The `TextBoxState` handles the text processing of the `TextBox` widget.
// #[derive(Default)]
// pub struct TextBoxState {
//     text: RefCell<String>,
//     focused: Cell<bool>,
//     updated: Cell<bool>,
//     selection_start: Cell<usize>,
//     selection_length: Cell<usize>,
//     cursor_x: Cell<f64>,
// }

// impl Into<Rc<dyn State>> for TextBoxState {
//     fn into(self) -> Rc<dyn State> {
//         Rc::new(self)
//     }
// }

// impl TextBoxState {
//     // fn click(&self, point: Point) {
//     //     println!("Clicked text box point: ({}, {})", point.x, point.y);
//     // }

//     fn update_selection_start(&self, selection: i32) {
//         self.selection_start
//             .set(selection.max(0).min(self.text.borrow().len() as i32) as usize);
//     }

//     fn update_text(&self, key: Key) -> bool {
//         if !self.focused.get() {
//             return false;
//         }

//         match <Option<u8>>::from(key) {
//             Some(byte) => {
//                 (*self.text.borrow_mut()).insert(self.selection_start.get(), byte as char);
//                 self.update_selection_start(self.selection_start.get() as i32 + 1);
//             }
//             None => match key {
//                 Key::Left => {
//                     self.update_selection_start(self.selection_start.get() as i32 - 1);
//                     self.selection_length.set(0);
//                 }
//                 Key::Right => {
//                     self.update_selection_start(self.selection_start.get() as i32 + 1);
//                     self.selection_length.set(0);
//                 }
//                 Key::Backspace => {
//                     if self.text.borrow().len() > 0 {
//                         if self.selection_start.get() > 0 {
//                             for _ in 0..(self.selection_length.get() + 1) {
//                                 (*self.text.borrow_mut()).remove(self.selection_start.get() - 1);
//                             }
//                             self.update_selection_start(self.selection_start.get() as i32 - 1);
//                         }
//                     }
//                 }
//                 _ => {}
//             },
//         }

//         self.updated.set(true);

//         true
//     }
// }

// impl State for TextBoxState {
//     fn update(&self, context: &mut Context<'_>) {
//         let mut widget = context.widget();

//         if let Ok(focused) = widget.get_mut::<Focused>() {
//             self.focused.set(focused.0);
//         }

//         if let Ok(text) = widget.borrow_mut::<Text>() {
//             if text.0 != *self.text.borrow() {
//                 if self.updated.get() {
//                     text.0 = self.text.borrow().clone();
//                 } else {
//                     let text_length = self.text.borrow().len();
//                     let origin_text_length = text.0.len();
//                     let delta = text_length as i32 - origin_text_length as i32;

//                     *self.text.borrow_mut() = text.0.clone();

//                     // adjust cursor position after label is changed from outside
//                     if text_length < origin_text_length {
//                         self.update_selection_start(self.selection_start.get() as i32 - delta);
//                     } else {
//                         self.update_selection_start(self.selection_start.get() as i32 + delta);
//                     }
//                 }

//                 self.updated.set(false);
//             }
//         }

//         if let Ok(selection) = widget.borrow_mut::<TextSelection>() {
//             selection.start_index = self.selection_start.get();
//             selection.length = self.selection_length.get();
//         }
//     }

//     fn update_post_layout(&self, context: &mut Context<'_>) {
//         let mut cursor_x_delta = 0.0;
//         let mut scroll_viewer_width = 0.0;

//         {
//             let scroll_viewer = context.child_by_id("TextBoxScrollViewer");

//             if let Ok(bounds) = scroll_viewer.unwrap().get_mut::<Bounds>() {
//                 scroll_viewer_width = bounds.width();
//             }
//         }

//         // maybe not use scroll viewer here

//         // Adjust offset of text and cursor if cursor position is out of bounds

//         {
//             let mut cursor = context.child_by_id("TextBoxCursor").unwrap();

//             if let Ok(margin) = cursor.borrow_mut::<Margin>() {
//                 if margin.left() < 0.0 || margin.left() > scroll_viewer_width {
//                     cursor_x_delta = self.cursor_x.get() - margin.left();
//                     margin.set_left(self.cursor_x.get());
//                 }
//                 self.cursor_x.set(margin.left());
//             }

//             if let Ok(bounds) = cursor.borrow_mut::<Bounds>() {
//                 bounds.set_x(self.cursor_x.get());
//             }
//         }

//         if cursor_x_delta != 0.0 {
//             {
//                 let text_block = context.child_by_id("TextBoxTextBox");

//                 if let Ok(bounds) = text_block.unwrap().borrow_mut::<Bounds>() {
//                     bounds.set_x(bounds.x() + cursor_x_delta);
//                 }
//             }

//             if let Ok(offset) = context.widget().borrow_mut::<Offset>() {
//                 offset.0 += cursor_x_delta;
//             }
//         }
//     }
// }

// widget!(
//     /// The `TextBox` represents a single line text input widget.
//     TextBox
//     (
//         BackgroundProperty,
//         BorderRadiusProperty,
//         BorderThicknessProperty,
//         BorderBrushProperty,
//         FontProperty,
//         FontSizeProperty,
//         TextProperty,
//         FocusedProperty,
//         WaterMarkProperty,
//         TextSelectionProperty,
//         PaddingProperty,
//         KeyDownHandler
//     )
// );

// impl Widget for TextBox {
//     fn create() -> Self {
//         // text properties
//         let text: Property = Text::default().into();
//         let foreground: Property = Foreground::from(colors::LINK_WATER_COLOR).into();
//         let font: Property = Font::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT)).into();
//         let font_size: Property = FontSize::from(fonts::FONT_SIZE_12).into();
//         let water_mark: Property = WaterMark::default().into();

//         // state properties
//         let selector = Selector::from("textbox");
//         let selection: Property = TextSelection::default().into();
//         let offset: Property = Offset::default().into();
//         let focused: Property = Focused(false).into();
//         let padding: Property = Padding::from(4.0).into();

//         // container properties
//         let background: Property = Background::from(colors::LYNCH_COLOR).into();
//         let border_radius: Property = BorderRadius::from(2.0).into();
//         let border_thickness: Property = BorderThickness::from(0.0).into();
//         let border_brush: Property = BorderBrush::from("transparent").into();
//         let _padding: Property = Padding::from((8.0, 0.0, 8.0, 0.0)).into();
//         let _opacity: Property = Opacity::from(1.0).into();

//         // states
//         let state = Rc::new(TextBoxState::default());
//         let _click_state = state.clone();

//         TextBox::new()
//             .size(128.0, 32.0)
//             .state(state.clone())
//             .debug_name("TextBox")
//             .child(
//                 Container::create()
//                     .child(
//                         Grid::create()
//                             .child(
//                                 ScrollViewer::create()
//                                     .child(
//                                         WaterMarkTextBox::create()
//                                             .vertical_alignment("Center")
//                                             .foreground_prop(foreground.share())
//                                             .text_prop(text.share())
//                                             .font_prop(font.share())
//                                             .font_size_prop(font_size.share())
//                                             .shared_water_mark(water_mark.share())
//                                             .attach(focused.share())
//                                             .selector(selector.clone().id("TextBoxTextBox")),
//                                     )
//                                     .shared_offset(offset.share())
//                                     .scroll_viewer_mode(ScrollViewerMode::new(
//                                         ScrollMode::None,
//                                         ScrollMode::None,
//                                     ))
//                                     .selector(Selector::new().id("TextBoxScrollViewer")),
//                             )
//                             .child(
//                                 Cursor::create()
//                                     .margin(0.0)
//                                     .horizontal_alignment("Start")
//                                     .text_prop(text.share())
//                                     .font_prop(font.share())
//                                     .font_size_prop(font_size.share())
//                                     .shared_text_selection(selection.share())
//                                     .shared_offset(offset.share())
//                                     .shared_focused(focused.share())
//                                     .selector(Selector::from("cursor").id("TextBoxCursor")),
//                             )
//                             // .event_handler(MouseEventHandler::default().on_mouse_down(Rc::new(
//                             //     move |pos: Point| -> bool {
//                             //         click_state.click(pos);
//                             //         false
//                             //     },
//                             // ))),
//                     )
//                     .attach(selector.clone())
//                     .attach(focused.share())
//                     .padding_prop(padding.share())
//                     .background_prop(background.share())
//                     .border_radius_prop(border_radius.share())
//                     .border_thickness_prop(border_thickness.share())
//                     .border_brush_prop(border_brush.share()),
//             )
//             .text_prop(text)
//             .font_prop(font)
//             .font_size_prop(font_size)
//             .selector(selector)
//             .shared_water_mark(water_mark)
//             .shared_text_selection(selection)
//             .attach(offset)
//             .shared_focused(focused)
//             .padding_prop(padding)
//             .background_prop(background)
//             .border_radius_prop(border_radius)
//             .border_thickness_prop(border_thickness)
//             .border_brush_prop(border_brush)
//             .on_key_down(move |key: Key| -> bool { state.update_text(key) })
//     }
// }

use dces::prelude::Entity;

use std::cell::Cell;

use crate::{
    event::{Key, KeyDownHandler},
    properties::*,
    structs::*,
    styling::{colors, fonts},
    widget::*,
};

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
#[derive(Default)]
pub struct TextBoxState {
    text: RefCell<String>,
    focused: Cell<bool>,
    updated: Cell<bool>,
    selection_start: Cell<usize>,
    selection_length: Cell<usize>,
    cursor_x: Cell<f64>,
}

impl Into<Rc<dyn State>> for TextBoxState {
    fn into(self) -> Rc<dyn State> {
        Rc::new(self)
    }
}

impl TextBoxState {
    // fn click(&self, point: Point) {
    //     println!("Clicked text box point: ({}, {})", point.x, point.y);
    // }

    fn update_selection_start(&self, selection: i32) {
        self.selection_start
            .set(selection.max(0).min(self.text.borrow().len() as i32) as usize);
    }

    fn update_text(&self, key: Key) -> bool {
        if !self.focused.get() {
            return false;
        }

        match <Option<u8>>::from(key) {
            Some(byte) => {
                (*self.text.borrow_mut()).insert(self.selection_start.get(), byte as char);
                self.update_selection_start(self.selection_start.get() as i32 + 1);
            }
            None => match key {
                Key::Left => {
                    self.update_selection_start(self.selection_start.get() as i32 - 1);
                    self.selection_length.set(0);
                }
                Key::Right => {
                    self.update_selection_start(self.selection_start.get() as i32 + 1);
                    self.selection_length.set(0);
                }
                Key::Backspace => {
                    if self.text.borrow().len() > 0 {
                        if self.selection_start.get() > 0 {
                            for _ in 0..(self.selection_length.get() + 1) {
                                (*self.text.borrow_mut()).remove(self.selection_start.get() - 1);
                            }
                            self.update_selection_start(self.selection_start.get() as i32 - 1);
                        }
                    }
                }
                _ => {}
            },
        }

        self.updated.set(true);

        true
    }
}

impl State for TextBoxState {
    fn update(&self, context: &mut Context<'_>) {
        let mut widget = context.widget();

        self.focused.set(widget.get::<Focused>().0);

        if let Ok(text) = widget.borrow_mut::<Text>() {
            if text.0 != *self.text.borrow() {
                if self.updated.get() {
                    text.0 = self.text.borrow().clone();
                } else {
                    let text_length = self.text.borrow().len();
                    let origin_text_length = text.0.len();
                    let delta = text_length as i32 - origin_text_length as i32;

                    *self.text.borrow_mut() = text.0.clone();

                    // adjust cursor position after label is changed from outside
                    if text_length < origin_text_length {
                        self.update_selection_start(self.selection_start.get() as i32 - delta);
                    } else {
                        self.update_selection_start(self.selection_start.get() as i32 + delta);
                    }
                }

                self.updated.set(false);
            }
        }

        if let Ok(selection) = widget.borrow_mut::<TextSelection>() {
            selection.0.start_index = self.selection_start.get();
            selection.0.length = self.selection_length.get();
        }
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        let mut cursor_x_delta = 0.0;
        let mut scroll_viewer_width = 0.0;

        {
            let scroll_viewer = context.child_by_id("scroll_viewer");

            if let Ok(bounds) = scroll_viewer.unwrap().borrow_mut::<Bounds>() {
                scroll_viewer_width = bounds.width();
            }
        }

        // maybe not use scroll viewer here

        // Adjust offset of text and cursor if cursor position is out of bounds

        {
            let mut cursor = context.child_by_id("cursor").unwrap();

            if let Ok(margin) = cursor.borrow_mut::<Margin>() {
                if margin.left() < 0.0 || margin.left() > scroll_viewer_width {
                    cursor_x_delta = self.cursor_x.get() - margin.left();
                    margin.set_left(self.cursor_x.get());
                }
                self.cursor_x.set(margin.left());
            }

            if let Ok(bounds) = cursor.borrow_mut::<Bounds>() {
                bounds.set_x(self.cursor_x.get());
            }
        }

        if cursor_x_delta != 0.0 {
            {
                let text_block = context.child_by_id("text_block");

                if let Ok(bounds) = text_block.unwrap().borrow_mut::<Bounds>() {
                    bounds.set_x(bounds.x() + cursor_x_delta);
                }
            }

            if let Ok(offset) = context.widget().borrow_mut::<Offset>() {
                (offset.0).0 += cursor_x_delta;
            }
        }
    }
}

widget!(
    /// The `TextBox` widget represents a single line text input widget.
    /// 
    /// * CSS element: `text-box`
    TextBox<TextBoxState>: KeyDownHandler {
        /// Sets or shares the text property.
        text: Text,

        /// Sets or shares the placeholder text property.
        placeholder: WaterMark,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// Sets or shares the foreground property.
        foreground: Foreground,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the text offset property.
        offset: Offset,

         /// Sets or shares the focused property.
        focused: Focused,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for TextBox {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("TextBox")
            .selector("text-box")
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
            .selection(TextSelectionValue::default())
            .offset(0.0)
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_thickness(0.0)
            .border_radius(2.0)
            .size(128.0, 32.0)
            .focused(false)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Grid::create()
                            .child(
                                ScrollViewer::create()
                                    .selector(Selector::default().id("scroll_viewer"))
                                    .offset(id)
                                    .scroll_mode(("None", "None"))
                                    .child(
                                        TextBlock::create()
                                            .selector(Selector::default().clone().id("text_block"))
                                            .vertical_alignment("Center")
                                            .foreground(id)
                                            .text(id)
                                            .font(id)
                                            .font_size(id)
                                            .build(context),
                                    )
                                    .build(context),
                            )
                            .child(
                                Cursor::create()
                                    .selector(Selector::from("cursor").id("cursor"))
                                    .margin(0.0)
                                    .horizontal_alignment("Start")
                                    .text(id)
                                    .font(id)
                                    .font_size(id)
                                    .offset(id)
                                    .focused(id)
                                    .selection(id)
                                    .build(context),
                            )
                            .build(context),
                    )
                    .build(context),
            )
            .on_key_down(move |key: Key| -> bool { state.update_text(key) })
    }
}
