use std::borrow::Cow;
use std::fmt;
use wasm_bindgen::{prelude::*, JsCast};

/// A guard on a callback that will unregister the callback when it goes out of scope. Makes things
/// like `window.addEventListener` RAII.
pub struct SubscribeGuard {
    // This could actually be FnOnce, since we will call it only once.
    unsubscribe: Option<Box<dyn FnMut()>>,
}

impl SubscribeGuard {
    pub(crate) fn new(unsubscribe: impl FnMut() + 'static) -> Self {
        SubscribeGuard {
            unsubscribe: Some(Box::new(unsubscribe)),
        }
    }
}

impl fmt::Debug for SubscribeGuard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SubscribeGuard")
    }
}

impl Drop for SubscribeGuard {
    fn drop(&mut self) {
        if let Some(mut unsubscribe) = self.unsubscribe.take() {
            unsubscribe()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    inner: web_sys::Event,
}

impl From<EventKind> for Event {
    fn from(kind: EventKind) -> Event {
        let inner = expect!(
            web_sys::Event::new(&Cow::from(kind)),
            "could not create event"
        );
        Event { inner }
    }
}

impl From<web_sys::Event> for Event {
    fn from(inner: web_sys::Event) -> Self {
        Event { inner }
    }
}

impl From<Event> for web_sys::Event {
    fn from(event: Event) -> web_sys::Event {
        event.inner
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EventKind {
    Abort,
    BeforeInput,
    Blur,
    Click,
    CompositionStart,
    CompositionUpdate,
    CompositionEnd,
    DoubleClick,
    Error,
    Focus,
    FocusIn,
    FocusOut,
    Input,
    KeyDown,
    KeyPress,
    KeyUp,
    Load,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOut,
    MouseOver,
    MouseUp,
    Resize,
    Scroll,
    Select,
    Unload,
    Wheel,
    Other(Cow<'static, str>),
}

impl From<String> for EventKind {
    fn from(kind: String) -> EventKind {
        match kind.as_str() {
            "abort" => return EventKind::Abort,
            "beforeinput" => return EventKind::BeforeInput,
            "blur" => return EventKind::Blur,
            "click" => return EventKind::Click,
            "compositionstart" => return EventKind::CompositionStart,
            "compositionupdate" => return EventKind::CompositionUpdate,
            "compositionend" => return EventKind::CompositionEnd,
            "dblclick" => return EventKind::DoubleClick,
            "error" => return EventKind::Error,
            "focus" => return EventKind::Focus,
            "focusin" => return EventKind::FocusIn,
            "focusout" => return EventKind::FocusOut,
            "input" => return EventKind::Input,
            "keydown" => return EventKind::KeyDown,
            "keypress" => return EventKind::KeyPress,
            "keyup" => return EventKind::KeyUp,
            "load" => return EventKind::Load,
            "mousedown" => return EventKind::MouseDown,
            "mouseenter" => return EventKind::MouseEnter,
            "mouseleave" => return EventKind::MouseLeave,
            "mousemove" => return EventKind::MouseMove,
            "mouseout" => return EventKind::MouseOut,
            "mouseover" => return EventKind::MouseOver,
            "mouseup" => return EventKind::MouseUp,
            "resize" => return EventKind::Resize,
            "scroll" => return EventKind::Scroll,
            "select" => return EventKind::Select,
            "unload" => return EventKind::Unload,
            "wheel" => return EventKind::Wheel,
            _ => (),
        }
        EventKind::Other(Cow::Owned(kind))
    }
}

impl From<EventKind> for Cow<'static, str> {
    fn from(event_kind: EventKind) -> Cow<'static, str> {
        match event_kind {
            EventKind::Abort => Cow::Borrowed("abort"),
            EventKind::BeforeInput => Cow::Borrowed("beforeinput"),
            EventKind::Blur => Cow::Borrowed("blur"),
            EventKind::Click => Cow::Borrowed("click"),
            EventKind::CompositionStart => Cow::Borrowed("compositionstart"),
            EventKind::CompositionUpdate => Cow::Borrowed("compositionupdate"),
            EventKind::CompositionEnd => Cow::Borrowed("compositionend"),
            EventKind::DoubleClick => Cow::Borrowed("dblclick"),
            EventKind::Error => Cow::Borrowed("error"),
            EventKind::Focus => Cow::Borrowed("focus"),
            EventKind::FocusIn => Cow::Borrowed("focusin"),
            EventKind::FocusOut => Cow::Borrowed("focusout"),
            EventKind::Input => Cow::Borrowed("input"),
            EventKind::KeyDown => Cow::Borrowed("keydown"),
            EventKind::KeyPress => Cow::Borrowed("keypress"),
            EventKind::KeyUp => Cow::Borrowed("keyup"),
            EventKind::Load => Cow::Borrowed("load"),
            EventKind::MouseDown => Cow::Borrowed("mousedown"),
            EventKind::MouseEnter => Cow::Borrowed("mouseenter"),
            EventKind::MouseLeave => Cow::Borrowed("mouseleave"),
            EventKind::MouseMove => Cow::Borrowed("mousemove"),
            EventKind::MouseOut => Cow::Borrowed("mouseout"),
            EventKind::MouseOver => Cow::Borrowed("mouseover"),
            EventKind::MouseUp => Cow::Borrowed("mouseup"),
            EventKind::Resize => Cow::Borrowed("resize"),
            EventKind::Scroll => Cow::Borrowed("scroll"),
            EventKind::Select => Cow::Borrowed("select"),
            EventKind::Unload => Cow::Borrowed("unload"),
            EventKind::Wheel => Cow::Borrowed("wheel"),
            EventKind::Other(other) => other,
        }
    }
}

dict! {
    /// Options for the `add_event_listener` function.
    pub struct AddEventListenerOptions {
        pub capture: bool,
        pub once: bool,
        pub passive: bool,
    }
}

impl AddEventListenerOptions {
    /// Convert into the corresponding web_sys type.
    pub(crate) fn into_web_sys_remove(&self) -> Option<web_sys::EventListenerOptions> {
        match *self {
            AddEventListenerOptions {
                capture: Some(capture),
                ..
            } => {
                let mut opts = web_sys::EventListenerOptions::new();
                opts.capture(capture);
                Some(opts)
            }
            _ => None,
        }
    }
}

/// The `EventTarget` interface in rust.
pub trait IEventTarget {
    /// Run the function `listener` when the event type fires on this object.
    ///
    /// Unlike the javascript equivalent, this function returns a guard that unregisters the
    /// callback when it is dropped.
    fn add_event_listener(
        &self,
        event_kind: EventKind,
        listener: impl Fn(Event) + 'static,
    ) -> SubscribeGuard {
        self.add_event_listener_opts(event_kind, listener, Default::default())
    }
    fn add_event_listener_opts(
        &self,
        event_kind: EventKind,
        listener: impl Fn(Event) + 'static,
        options: AddEventListenerOptions,
    ) -> SubscribeGuard;
    fn dispatch_event(&self, event: impl Into<Event>) -> bool;
}

#[repr(transparent)]
pub struct EventTarget {
    pub(crate) inner: web_sys::EventTarget,
}

impl EventTarget {
    pub fn new() -> Self {
        let inner = expect!(
            web_sys::EventTarget::new(),
            "failed to create an `EventTarget`"
        );
        Self { inner }
    }
}

macro_rules! impl_IEventTarget {
    ($name:ident, $as_ref:expr) => {
        impl IEventTarget for $name {
            fn add_event_listener_opts(
                &self,
                event_kind: EventKind,
                listener: impl Fn(Event) + 'static,
                options: AddEventListenerOptions,
            ) -> SubscribeGuard {
                let target = self.inner.clone();
                let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    listener(event.into());
                }) as Box<dyn Fn(web_sys::Event)>);
                let event_kind = Cow::from(event_kind);
                if let Some(opts) = options.into_web_sys() {
                    expect!(
                        target.add_event_listener_with_callback_and_add_event_listener_options(
                            &event_kind.clone(),
                            closure.as_ref().unchecked_ref(),
                            &opts,
                        ),
                        "failed to add event listener to EventTarget"
                    );
                } else {
                    expect!(
                        target.add_event_listener_with_callback(
                            &event_kind.clone(),
                            closure.as_ref().unchecked_ref(),
                        ),
                        "failed to add event listener to EventTarget"
                    );
                }
                if let Some(opts) = options.into_web_sys_remove() {
                    SubscribeGuard::new(move || {
                        let _ = closure;
                        expect!(
                            target.remove_event_listener_with_callback_and_event_listener_options(
                                &event_kind,
                                closure.as_ref().unchecked_ref(),
                                &opts
                            ),
                            "unable to remove event listener"
                        );
                    })
                } else {
                    SubscribeGuard::new(move || {
                        let _ = closure;
                        expect!(
                            target.remove_event_listener_with_callback(
                                &event_kind,
                                closure.as_ref().unchecked_ref()
                            ),
                            "unable to remove event listener"
                        );
                    })
                }
            }

            fn dispatch_event(&self, event: impl Into<Event>) -> bool {
                let event = event.into();
                expect!(
                    self.inner.dispatch_event(&event.clone().into()),
                    "could not dispatch event {:?}",
                    event
                )
            }
        }
    }
}

impl IEventTarget for EventTarget {
    fn add_event_listener_opts(
        &self,
        event_kind: EventKind,
        listener: impl Fn(Event) + 'static,
        options: AddEventListenerOptions,
    ) -> SubscribeGuard {
        let target = self.inner.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            listener(event.into());
        }) as Box<dyn Fn(web_sys::Event)>);
        let event_kind = Cow::from(event_kind);
        if let Some(opts) = options.into_web_sys() {
            expect!(
                target.add_event_listener_with_callback_and_add_event_listener_options(
                    &event_kind.clone(),
                    closure.as_ref().unchecked_ref(),
                    &opts,
                ),
                "failed to add event listener to EventTarget"
            );
        } else {
            expect!(
                target.add_event_listener_with_callback(
                    &event_kind.clone(),
                    closure.as_ref().unchecked_ref(),
                ),
                "failed to add event listener to EventTarget"
            );
        }
        if let Some(opts) = options.into_web_sys_remove() {
            SubscribeGuard::new(move || {
                let _ = closure;
                expect!(
                    target.remove_event_listener_with_callback_and_event_listener_options(
                        &event_kind,
                        closure.as_ref().unchecked_ref(),
                        &opts
                    ),
                    "unable to remove event listener"
                );
            })
        } else {
            SubscribeGuard::new(move || {
                let _ = closure;
                expect!(
                    target.remove_event_listener_with_callback(
                        &event_kind,
                        closure.as_ref().unchecked_ref()
                    ),
                    "unable to remove event listener"
                );
            })
        }
    }

    fn dispatch_event(&self, event: impl Into<Event>) -> bool {
        let event = event.into();
        expect!(
            self.inner.dispatch_event(&event.clone().into()),
            "could not dispatch event {:?}",
            event
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn event_target() {
        let counter = Rc::new(RefCell::new(0u32));
        let target = EventTarget::new();
        let counter_copy = counter.clone();
        let _ = target.add_event_listener(EventKind::Click, move |_| {
            *counter_copy.borrow_mut() += 1;
        });
        target.dispatch_event(EventKind::Click.into());
        target.dispatch_event(EventKind::Click.into());
        assert_eq!(*counter.borrow(), 2);
    }
}
