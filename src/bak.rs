use libc::{c_char, c_int, c_uchar, c_ulong, c_long, c_uint};

// boolean values
pub const False: Bool = 0;
pub const True: Bool = 1;

// allocate colormap
pub const AllocNone: c_int = 0;
pub const AllocAll: c_int = 1;

// window classes
pub const InputOutput: c_int = 1;
pub const InputOnly: c_int = 2;

// window attributes
pub const CWBackPixmap: c_ulong = 0x0001;
pub const CWBackPixel: c_ulong = 0x0002;
pub const CWBorderPixmap: c_ulong = 0x0004;
pub const CWBorderPixel: c_ulong = 0x0008;
pub const CWBitGravity: c_ulong = 0x0010;
pub const CWWinGravity: c_ulong = 0x0020;
pub const CWBackingStore: c_ulong = 0x0040;
pub const CWBackingPlanes: c_ulong = 0x0080;
pub const CWBackingPixel: c_ulong = 0x0100;
pub const CWOverrideRedirect: c_ulong = 0x0200;
pub const CWSaveUnder: c_ulong = 0x0400;
pub const CWEventMask: c_ulong = 0x0800;
pub const CWDontPropagate: c_ulong = 0x1000;
pub const CWColormap: c_ulong = 0x2000;
pub const CWCursor: c_ulong = 0x4000;

// event kinds
pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;
pub const ButtonPress: c_int = 4;
pub const ButtonRelease: c_int = 5;
pub const MotionNotify: c_int = 6;
pub const EnterNotify: c_int = 7;
pub const LeaveNotify: c_int = 8;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const KeymapNotify: c_int = 11;
pub const Expose: c_int = 12;
pub const GraphicsExpose: c_int = 13;
pub const NoExpose: c_int = 14;
pub const VisibilityNotify: c_int = 15;
pub const CreateNotify: c_int = 16;
pub const DestroyNotify: c_int = 17;
pub const UnmapNotify: c_int = 18;
pub const MapNotify: c_int = 19;
pub const MapRequest: c_int = 20;
pub const ReparentNotify: c_int = 21;
pub const ConfigureNotify: c_int = 22;
pub const ConfigureRequest: c_int = 23;
pub const GravityNotify: c_int = 24;
pub const ResizeRequest: c_int = 25;
pub const CirculateNotify: c_int = 26;
pub const CirculateRequest: c_int = 27;
pub const PropertyNotify: c_int = 28;
pub const SelectionClear: c_int = 29;
pub const SelectionRequest: c_int = 30;
pub const SelectionNotify: c_int = 31;
pub const ColormapNotify: c_int = 32;
pub const ClientMessage: c_int = 33;
pub const MappingNotify: c_int = 34;
pub const GenericEvent: c_int = 35;
pub const LASTEvent: c_int = 36;

// event mask
pub const NoEventMask: c_long = 0;
pub const KeyPressMask: c_long = 0x0000_0001;
pub const KeyReleaseMask: c_long = 0x0000_0002;
pub const ButtonPressMask: c_long = 0x0000_0004;
pub const ButtonReleaseMask: c_long = 0x0000_0008;
pub const EnterWindowMask: c_long = 0x0000_0010;
pub const LeaveWindowMask: c_long = 0x0000_0020;
pub const PointerMotionMask: c_long = 0x0000_0040;
pub const PointerMotionHintMask: c_long = 0x0000_0080;
pub const Button1MotionMask: c_long = 0x0000_0100;
pub const Button2MotionMask: c_long = 0x0000_0200;
pub const Button3MotionMask: c_long = 0x0000_0400;
pub const Button4MotionMask: c_long = 0x0000_0800;
pub const Button5MotionMask: c_long = 0x0000_1000;
pub const ButtonMotionMask: c_long = 0x0000_2000;
pub const KeymapStateMask: c_long = 0x0000_4000;
pub const ExposureMask: c_long = 0x0000_8000;
pub const VisibilityChangeMask: c_long = 0x0001_0000;
pub const StructureNotifyMask: c_long = 0x0002_0000;
pub const ResizeRedirectMask: c_long = 0x0004_0000;
pub const SubstructureNotifyMask: c_long = 0x0008_0000;
pub const SubstructureRedirectMask: c_long = 0x0010_0000;
pub const FocusChangeMask: c_long = 0x0020_0000;
pub const PropertyChangeMask: c_long = 0x0040_0000;
pub const ColormapChangeMask: c_long = 0x0080_0000;
pub const OwnerGrabButtonMask: c_long = 0x0100_0000;

// common types
pub type Atom = XID;
pub type Bool = c_int;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type GContext = XID;
pub type KeyCode = c_uchar;
pub type KeySym = XID;
pub type Mask = c_ulong;
pub type Pixmap = XID;
pub type Status = Bool;
pub type Time = c_ulong;
pub type VisualID = XID;
pub type Window = XID;
pub type XID = c_ulong;
pub type XPointer = *mut c_char;

// opaque structures
pub enum _XDisplay {}
pub enum _XGC {}

// misc typedefs
pub type Display = _XDisplay;
pub type GC = *mut _XGC;

#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub bits_per_rgb: c_int,
    pub map_entries: c_int,
}

#[repr(C)]
pub struct XExtData {
    pub number: c_int,
    pub next: *mut XExtData,
    pub free_private: Option<unsafe extern "C" fn() -> c_int>,
    pub private_data: XPointer,
}

#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: Bool,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: Bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

#[repr(C)]
pub struct XWindowAttributes {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub depth: c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: c_int,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: Bool,
    pub colormap: Colormap,
    pub map_installed: Bool,
    pub map_state: c_int,
    pub all_event_masks: c_long,
    pub your_event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: Bool,
    pub screen: *mut Screen,
}

#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut Display,
    pub root: Window,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub ndepths: c_int,
    pub depths: *mut Depth,
    pub root_depth: c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: c_ulong,
    pub black_pixel: c_ulong,
    pub max_maps: c_int,
    pub min_maps: c_int,
    pub backing_store: c_int,
    pub save_unders: Bool,
    pub root_input_mask: c_long,
}

#[repr(C)]
pub struct Depth {
    pub depth: c_int,
    pub nvisuals: c_int,
    pub visuals: *mut Visual,
}

#[link(name = "X11")]
extern "C" {
    pub fn XCreateColormap(_4: *mut Display, _3: c_ulong, _2: *mut Visual, _1: c_int) -> c_ulong;
    pub fn XCreateWindow (_12: *mut Display, _11: c_ulong, _10: c_int, _9: c_int, _8: c_uint, _7: c_uint, _6: c_uint, _5: c_int, _4: c_uint, _3: *mut Visual, _2: c_ulong, _1: *mut XSetWindowAttributes) -> c_ulong;
    pub fn XDefaultScreen(_1: *mut Display) -> c_int;
    pub fn XInternAtom (_3: *mut Display, _2: *const c_char, _1: c_int) -> c_ulong;
    pub fn XMapWindow (_2: *mut Display, _1: c_ulong) -> c_int;
    pub fn XOpenDisplay(_1: *const c_char) -> *mut Display;
    pub fn XRootWindow(_2: *mut Display, _1: c_int) -> c_ulong;
    pub fn XSetWMProtocols (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int;
    pub fn XStoreName (_3: *mut Display, _2: c_ulong, _1: *const c_char) -> c_int;
    pub fn XWhitePixel (_2: *mut Display, _1: c_int) -> c_ulong;
    pub fn XNextEvent (_2: *mut Display, _1: *mut libc::c_void) -> c_int; // TODO: Wrong cvoid here
    pub fn XDestroyWindow (_2: *mut Display, _1: c_ulong) -> c_int;
    pub fn XCloseDisplay (_1: *mut Display) -> c_int;
    pub fn XGetWindowAttributes (_3: *mut Display, _2: c_ulong, _1: *mut XWindowAttributes) -> c_int;
}


//
// event structures
//


#[derive(Clone, Copy)]
#[repr(C)]
pub union XEvent {
    pub type_: c_int,
    pub any: XAnyEvent,
    pub button: XButtonEvent,
    pub circulate: XCirculateEvent,
    pub circulate_request: XCirculateRequestEvent,
    pub client_message: XClientMessageEvent,
    pub colormap: XColormapEvent,
    pub configure: XConfigureEvent,
    pub configure_request: XConfigureRequestEvent,
    pub create_window: XCreateWindowEvent,
    pub crossing: XCrossingEvent,
    pub destroy_window: XDestroyWindowEvent,
    pub error: XErrorEvent,
    pub expose: XExposeEvent,
    pub focus_change: XFocusChangeEvent,
    pub generic_event_cookie: XGenericEventCookie,
    pub graphics_expose: XGraphicsExposeEvent,
    pub gravity: XGravityEvent,
    pub key: XKeyEvent,
    pub keymap: XKeymapEvent,
    pub map: XMapEvent,
    pub mapping: XMappingEvent,
    pub map_request: XMapRequestEvent,
    pub motion: XMotionEvent,
    pub no_expose: XNoExposeEvent,
    pub property: XPropertyEvent,
    pub reparent: XReparentEvent,
    pub resize_request: XResizeRequestEvent,
    pub selection_clear: XSelectionClearEvent,
    pub selection: XSelectionEvent,
    pub selection_request: XSelectionRequestEvent,
    pub unmap: XUnmapEvent,
    pub visibility: XVisibilityEvent,
    pub pad: [c_long; 24],
    // xf86vidmode
    pub xf86vm_notify: xf86vmode::XF86VidModeNotifyEvent,
    // xrandr
    pub xrr_screen_change_notify: xrandr::XRRScreenChangeNotifyEvent,
    pub xrr_notify: xrandr::XRRNotifyEvent,
    pub xrr_output_change_notify: xrandr::XRROutputChangeNotifyEvent,
    pub xrr_crtc_change_notify: xrandr::XRRCrtcChangeNotifyEvent,
    pub xrr_output_property_notify: xrandr::XRROutputPropertyNotifyEvent,
    pub xrr_provider_change_notify: xrandr::XRRProviderChangeNotifyEvent,
    pub xrr_provider_property_notify: xrandr::XRRProviderPropertyNotifyEvent,
    pub xrr_resource_change_notify: xrandr::XRRResourceChangeNotifyEvent,
    // xscreensaver
    pub xss_notify: xss::XScreenSaverNotifyEvent,
}

impl XEvent {
    pub fn get_type (&self) -> c_int {
        unsafe {
            self.type_
        }
    }
}

impl fmt::Debug for XEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("XEvent");
        unsafe {
            match self.type_ {
                KeyPress => d.field("key", &self.key),
                KeyRelease => d.field("key", &self.key),
                ButtonPress => d.field("button", &self.button),
                ButtonRelease => d.field("button", &self.button),
                MotionNotify => d.field("motion", &self.motion),
                EnterNotify => d.field("crossing", &self.crossing),
                LeaveNotify => d.field("crossing", &self.crossing),
                FocusIn => d.field("focus_change", &self.focus_change),
                FocusOut => d.field("focus_change", &self.focus_change),
                KeymapNotify => d.field("keymap", &self.keymap),
                Expose => d.field("expose", &self.expose),
                GraphicsExpose => d.field("graphics_expose", &self.graphics_expose),
                NoExpose => d.field("no_expose", &self.no_expose),
                VisibilityNotify => d.field("visibility", &self.visibility),
                CreateNotify => d.field("create_window", &self.create_window),
                DestroyNotify => d.field("destroy_window", &self.destroy_window),
                UnmapNotify => d.field("unmap", &self.unmap),
                MapNotify => d.field("map", &self.map),
                MapRequest => d.field("map_request", &self.map_request),
                ReparentNotify => d.field("reparent", &self.reparent),
                ConfigureNotify => d.field("configure", &self.configure),
                ConfigureRequest => d.field("configure_request", &self.configure_request),
                GravityNotify => d.field("gravity", &self.gravity),
                ResizeRequest => d.field("resize_request", &self.resize_request),
                CirculateNotify => d.field("circulate", &self.circulate),
                CirculateRequest => d.field("circulate_request", &self.circulate_request),
                PropertyNotify => d.field("property", &self.property),
                SelectionClear => d.field("selection_clear", &self.selection_clear),
                SelectionRequest => d.field("selection_request", &self.selection_request),
                SelectionNotify => d.field("selection", &self.selection),
                ColormapNotify => d.field("colormap", &self.colormap),
                ClientMessage => d.field("client_message", &self.client_message),
                MappingNotify => d.field("mapping", &self.mapping),
                GenericEvent => d.field("generic_event_cookie", &self.generic_event_cookie),
                _ => d.field("any", &self.any),
            }
        }.finish()
    }
}

macro_rules! event_conversions_and_tests {
  { $($field:ident: $ty:ty,)* } => {
    #[test]
    fn xevent_size_test () {
      use std::mem::size_of;
      let xevent_size = size_of::<XEvent>();
      $(assert!(xevent_size >= size_of::<$ty>());)*
    }

    $(
      impl AsMut<$ty> for XEvent {
        fn as_mut (&mut self) -> &mut $ty {
          unsafe { &mut self.$field }
        }
      }

      impl AsRef<$ty> for XEvent {
        fn as_ref (&self) -> &$ty {
          unsafe { &self.$field }
        }
      }

      impl From<$ty> for XEvent {
        fn from (other: $ty) -> XEvent {
          XEvent{ $field: other }
        }
      }

      impl<'a> From<&'a $ty> for XEvent {
        fn from (other: &'a $ty) -> XEvent {
          XEvent{ $field: other.clone() }
        }
      }

      impl From<XEvent> for $ty {
        fn from (xevent: XEvent) -> $ty {
          unsafe { xevent.$field }
        }
      }

      impl<'a> From<&'a XEvent> for $ty {
        fn from (xevent: &'a XEvent) -> $ty {
          unsafe { xevent.$field }
        }
      }
    )*
  };
}

event_conversions_and_tests! {
  any: XAnyEvent,
  button: XButtonEvent,
  circulate: XCirculateEvent,
  circulate_request: XCirculateRequestEvent,
  client_message: XClientMessageEvent,
  colormap: XColormapEvent,
  configure: XConfigureEvent,
  configure_request: XConfigureRequestEvent,
  create_window: XCreateWindowEvent,
  crossing: XCrossingEvent,
  destroy_window: XDestroyWindowEvent,
  error: XErrorEvent,
  expose: XExposeEvent,
  focus_change: XFocusChangeEvent,
  generic_event_cookie: XGenericEventCookie,
  graphics_expose: XGraphicsExposeEvent,
  gravity: XGravityEvent,
  key: XKeyEvent,
  keymap: XKeymapEvent,
  map: XMapEvent,
  mapping: XMappingEvent,
  map_request: XMapRequestEvent,
  motion: XMotionEvent,
  no_expose: XNoExposeEvent,
  property: XPropertyEvent,
  reparent: XReparentEvent,
  resize_request: XResizeRequestEvent,
  selection_clear: XSelectionClearEvent,
  selection: XSelectionEvent,
  selection_request: XSelectionRequestEvent,
  unmap: XUnmapEvent,
  visibility: XVisibilityEvent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: Bool,
}
pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: c_int,
    pub data: ClientMessageData,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: Bool,
    pub state: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub override_redirect: Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub detail: c_int,
    pub value_mask: c_ulong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub override_redirect: Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub mode: c_int,
    pub detail: c_int,
    pub same_screen: Bool,
    pub focus: Bool,
    pub state: c_uint,
}
pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_: c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: c_ulong,
    pub error_code: c_uchar,
    pub request_code: c_uchar,
    pub minor_code: c_uchar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub mode: c_int,
    pub detail: c_int,
}
pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
    pub major_code: c_int,
    pub minor_code: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: Bool,
}
pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [c_char; 32],
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub request: c_int,
    pub first_keycode: c_int,
    pub count: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub is_hint: c_char,
    pub same_screen: Bool,
}
pub type XPointerMovedEvent = XMotionEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: c_int,
    pub minor_code: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: c_int,
    pub y: c_int,
    pub override_redirect: Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub width: c_int,
    pub height: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub state: c_int,
}
