#![allow(warnings)]

use crate::Xlib::{Atom, Time};
use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};

pub type Bool = c_int;

//
// Constants shamelessly ripped from https://github.com/erlepereira/x11-rs/blob/master/src/xlib.rs#L2635
//

// allocate colormap
pub const AllocNone: c_int = 0;
pub const AllocAll: c_int = 1;

// array sizes
pub const XkbKeyNameLength: usize = 4;
pub const XkbNumIndicators: usize = 32;
pub const XkbNumKbdGroups: usize = 4;
pub const XkbNumVirtualMods: usize = 16;

// atoms
pub const XA_PRIMARY: Atom = 1;
pub const XA_SECONDARY: Atom = 2;
pub const XA_ARC: Atom = 3;
pub const XA_ATOM: Atom = 4;
pub const XA_BITMAP: Atom = 5;
pub const XA_CARDINAL: Atom = 6;
pub const XA_COLORMAP: Atom = 7;
pub const XA_CURSOR: Atom = 8;
pub const XA_CUT_BUFFER0: Atom = 9;
pub const XA_CUT_BUFFER1: Atom = 10;
pub const XA_CUT_BUFFER2: Atom = 11;
pub const XA_CUT_BUFFER3: Atom = 12;
pub const XA_CUT_BUFFER4: Atom = 13;
pub const XA_CUT_BUFFER5: Atom = 14;
pub const XA_CUT_BUFFER6: Atom = 15;
pub const XA_CUT_BUFFER7: Atom = 16;
pub const XA_DRAWABLE: Atom = 17;
pub const XA_FONT: Atom = 18;
pub const XA_INTEGER: Atom = 19;
pub const XA_PIXMAP: Atom = 20;
pub const XA_POINT: Atom = 21;
pub const XA_RECTANGLE: Atom = 22;
pub const XA_RESOURCE_MANAGER: Atom = 23;
pub const XA_RGB_COLOR_MAP: Atom = 24;
pub const XA_RGB_BEST_MAP: Atom = 25;
pub const XA_RGB_BLUE_MAP: Atom = 26;
pub const XA_RGB_DEFAULT_MAP: Atom = 27;
pub const XA_RGB_GRAY_MAP: Atom = 28;
pub const XA_RGB_GREEN_MAP: Atom = 29;
pub const XA_RGB_RED_MAP: Atom = 30;
pub const XA_STRING: Atom = 31;
pub const XA_VISUALID: Atom = 32;
pub const XA_WINDOW: Atom = 33;
pub const XA_WM_COMMAND: Atom = 34;
pub const XA_WM_HINTS: Atom = 35;
pub const XA_WM_CLIENT_MACHINE: Atom = 36;
pub const XA_WM_ICON_NAME: Atom = 37;
pub const XA_WM_ICON_SIZE: Atom = 38;
pub const XA_WM_NAME: Atom = 39;
pub const XA_WM_NORMAL_HINTS: Atom = 40;
pub const XA_WM_SIZE_HINTS: Atom = 41;
pub const XA_WM_ZOOM_HINTS: Atom = 42;
pub const XA_MIN_SPACE: Atom = 43;
pub const XA_NORM_SPACE: Atom = 44;
pub const XA_MAX_SPACE: Atom = 45;
pub const XA_END_SPACE: Atom = 46;
pub const XA_SUPERSCRIPT_X: Atom = 47;
pub const XA_SUPERSCRIPT_Y: Atom = 48;
pub const XA_SUBSCRIPT_X: Atom = 49;
pub const XA_SUBSCRIPT_Y: Atom = 50;
pub const XA_UNDERLINE_POSITION: Atom = 51;
pub const XA_UNDERLINE_THICKNESS: Atom = 52;
pub const XA_STRIKEOUT_ASCENT: Atom = 53;
pub const XA_STRIKEOUT_DESCENT: Atom = 54;
pub const XA_ITALIC_ANGLE: Atom = 55;
pub const XA_X_HEIGHT: Atom = 56;
pub const XA_QUAD_WIDTH: Atom = 57;
pub const XA_WEIGHT: Atom = 58;
pub const XA_POINT_SIZE: Atom = 59;
pub const XA_RESOLUTION: Atom = 60;
pub const XA_COPYRIGHT: Atom = 61;
pub const XA_NOTICE: Atom = 62;
pub const XA_FONT_NAME: Atom = 63;
pub const XA_FAMILY_NAME: Atom = 64;
pub const XA_FULL_NAME: Atom = 65;
pub const XA_CAP_HEIGHT: Atom = 66;
pub const XA_WM_CLASS: Atom = 67;
pub const XA_WM_TRANSIENT_FOR: Atom = 68;

// boolean values
pub const False: Bool = 0;
pub const True: Bool = 1;

// clip rect ordering
pub const Unsorted: c_int = 0;
pub const YSorted: c_int = 1;
pub const YXSorted: c_int = 2;
pub const YXBanded: c_int = 3;

// color component mask
pub const DoRed: c_char = 1;
pub const DoGreen: c_char = 2;
pub const DoBlue: c_char = 4;

// error codes
pub const Success: c_uchar = 0;
pub const BadRequest: c_uchar = 1;
pub const BadValue: c_uchar = 2;
pub const BadWindow: c_uchar = 3;
pub const BadPixmap: c_uchar = 4;
pub const BadAtom: c_uchar = 5;
pub const BadCursor: c_uchar = 6;
pub const BadFont: c_uchar = 7;
pub const BadMatch: c_uchar = 8;
pub const BadDrawable: c_uchar = 9;
pub const BadAccess: c_uchar = 10;
pub const BadAlloc: c_uchar = 11;
pub const BadColor: c_uchar = 12;
pub const BadGC: c_uchar = 13;
pub const BadIDChoice: c_uchar = 14;
pub const BadName: c_uchar = 15;
pub const BadLength: c_uchar = 16;
pub const BadImplementation: c_uchar = 17;
pub const FirstExtensionError: c_uchar = 128;
pub const LastExtensionError: c_uchar = 255;

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

// property modes
pub const PropModeReplace: c_int = 0;
pub const PropModePrepend: c_int = 1;
pub const PropModeAppend: c_int = 2;

// modifier names
pub const ShiftMapIndex: c_int = 0;
pub const LockMapIndex: c_int = 1;
pub const ControlMapIndex: c_int = 2;
pub const Mod1MapIndex: c_int = 3;
pub const Mod2MapIndex: c_int = 4;
pub const Mod3MapIndex: c_int = 5;
pub const Mod4MapIndex: c_int = 6;
pub const Mod5MapIndex: c_int = 7;

// button masks
pub const Button1Mask: c_uint = (1 << 8);
pub const Button2Mask: c_uint = (1 << 9);
pub const Button3Mask: c_uint = (1 << 10);
pub const Button4Mask: c_uint = (1 << 11);
pub const Button5Mask: c_uint = (1 << 12);
pub const AnyModifier: c_uint = (1 << 15);

// Notify modes
pub const NotifyNormal: c_int = 0;
pub const NotifyGrab: c_int = 1;
pub const NotifyUngrab: c_int = 2;
pub const NotifyWhileGrabbed: c_int = 3;

pub const NotifyHint: c_int = 1;

// Notify detail
pub const NotifyAncestor: c_int = 0;
pub const NotifyVirtual: c_int = 1;
pub const NotifyInferior: c_int = 2;
pub const NotifyNonlinear: c_int = 3;
pub const NotifyNonlinearVirtual: c_int = 4;
pub const NotifyPointer: c_int = 5;
pub const NotifyPointerRoot: c_int = 6;
pub const NotifyDetailNone: c_int = 7;

// Visibility notify
pub const VisibilityUnobscured: c_int = 0;
pub const VisibilityPartiallyObscured: c_int = 1;
pub const VisibilityFullyObscured: c_int = 2;

// Circulation request
pub const PlaceOnTop: c_int = 0;
pub const PlaceOnBottom: c_int = 1;

// protocol families
pub const FamilyInternet: c_int = 0;
pub const FamilyDECnet: c_int = 1;
pub const FamilyChaos: c_int = 2;
pub const FamilyInternet6: c_int = 6;

// authentication families not tied to a specific protocol
pub const FamilyServerInterpreted: c_int = 5;

// property notification
pub const PropertyNewValue: c_int = 0;
pub const PropertyDelete: c_int = 1;

// Color Map notification
pub const ColormapUninstalled: c_int = 0;
pub const ColormapInstalled: c_int = 1;

// grab modes
pub const GrabModeSync: c_int = 0;
pub const GrabModeAsync: c_int = 1;

// grab status
pub const GrabSuccess: c_int = 0;
pub const AlreadyGrabbed: c_int = 1;
pub const GrabInvalidTime: c_int = 2;
pub const GrabNotViewable: c_int = 3;
pub const GrabFrozen: c_int = 4;

// AllowEvents modes
pub const AsyncPointer: c_int = 0;
pub const SyncPointer: c_int = 1;
pub const ReplayPointer: c_int = 2;
pub const AsyncKeyboard: c_int = 3;
pub const SyncKeyboard: c_int = 4;
pub const ReplayKeyboard: c_int = 5;
pub const AsyncBoth: c_int = 6;
pub const SyncBoth: c_int = 7;

// Used in SetInputFocus, GetInputFocus
pub const RevertToNone: c_int = 0;
pub const RevertToPointerRoot: c_int = 1;
pub const RevertToParent: c_int = 2;

// ConfigureWindow structure
pub const CWX: c_ushort = (1 << 0);
pub const CWY: c_ushort = (1 << 1);
pub const CWWidth: c_ushort = (1 << 2);
pub const CWHeight: c_ushort = (1 << 3);
pub const CWBorderWidth: c_ushort = (1 << 4);
pub const CWSibling: c_ushort = (1 << 5);
pub const CWStackMode: c_ushort = (1 << 6);

// gravity
pub const ForgetGravity: c_int = 0;
pub const UnmapGravity: c_int = 0;
pub const NorthWestGravity: c_int = 1;
pub const NorthGravity: c_int = 2;
pub const NorthEastGravity: c_int = 3;
pub const WestGravity: c_int = 4;
pub const CenterGravity: c_int = 5;
pub const EastGravity: c_int = 6;
pub const SouthWestGravity: c_int = 7;
pub const SouthGravity: c_int = 8;
pub const SouthEastGravity: c_int = 9;
pub const StaticGravity: c_int = 10;

// image format
pub const XYBitmap: c_int = 0;
pub const XYPixmap: c_int = 1;
pub const ZPixmap: c_int = 2;

// Used in CreateWindow for backing-store hint
pub const NotUseful: c_int = 0;
pub const WhenMapped: c_int = 1;
pub const Always: c_int = 2;

// map state
pub const IsUnmapped: c_int = 0;
pub const IsUnviewable: c_int = 1;
pub const IsViewable: c_int = 2;

// modifier keys mask
pub const ShiftMask: c_uint = 0x01;
pub const LockMask: c_uint = 0x02;
pub const ControlMask: c_uint = 0x04;
pub const Mod1Mask: c_uint = 0x08;
pub const Mod2Mask: c_uint = 0x10;
pub const Mod3Mask: c_uint = 0x20;
pub const Mod4Mask: c_uint = 0x40;
pub const Mod5Mask: c_uint = 0x80;

// mouse buttons
pub const Button1: c_uint = 1;
pub const Button2: c_uint = 2;
pub const Button3: c_uint = 3;
pub const Button4: c_uint = 4;
pub const Button5: c_uint = 5;

// size hints mask
pub const USPosition: c_long = 0x0001;
pub const USSize: c_long = 0x0002;
pub const PPosition: c_long = 0x0004;
pub const PSize: c_long = 0x0008;
pub const PMinSize: c_long = 0x0010;
pub const PMaxSize: c_long = 0x0020;
pub const PResizeInc: c_long = 0x0040;
pub const PAspect: c_long = 0x0080;
pub const PBaseSize: c_long = 0x0100;
pub const PWinGravity: c_long = 0x0200;
pub const PAllHints: c_long = PPosition | PSize | PMinSize | PMaxSize | PResizeInc | PAspect;

// Used in ChangeSaveSet
pub const SetModeInsert: c_int = 0;
pub const SetModeDelete: c_int = 1;

// Used in ChangeCloseDownMode
pub const DestroyAll: c_int = 0;
pub const RetainPermanent: c_int = 1;
pub const RetainTemporary: c_int = 2;

// Window stacking method (in configureWindow)
pub const Above: c_int = 0;
pub const Below: c_int = 1;
pub const TopIf: c_int = 2;
pub const BottomIf: c_int = 3;
pub const Opposite: c_int = 4;

// Circulation direction
pub const RaiseLowest: c_int = 0;
pub const LowerHighest: c_int = 1;

// graphics functions
pub const GXclear: c_int = 0x0;
pub const GXand: c_int = 0x1;
pub const GXandReverse: c_int = 0x2;
pub const GXcopy: c_int = 0x3;
pub const GXandInverted: c_int = 0x4;
pub const GXnoop: c_int = 0x5;
pub const GXxor: c_int = 0x6;
pub const GXor: c_int = 0x7;
pub const GXnor: c_int = 0x8;
pub const GXequiv: c_int = 0x9;
pub const GXinvert: c_int = 0xa;
pub const GXorReverse: c_int = 0xb;
pub const GXcopyInverted: c_int = 0xc;
pub const GXorInverted: c_int = 0xd;
pub const GXnand: c_int = 0xe;
pub const GXset: c_int = 0xf;

// LineStyle
pub const LineSolid: c_int = 0;
pub const LineOnOffDash: c_int = 1;
pub const LineDoubleDash: c_int = 2;

// capStyle
pub const CapNotLast: c_int = 0;
pub const CapButt: c_int = 1;
pub const CapRound: c_int = 2;
pub const CapProjecting: c_int = 3;

// joinStyle
pub const JoinMiter: c_int = 0;
pub const JoinRound: c_int = 1;
pub const JoinBevel: c_int = 2;

// fillStyle
pub const FillSolid: c_int = 0;
pub const FillTiled: c_int = 1;
pub const FillStippled: c_int = 2;
pub const FillOpaqueStippled: c_int = 3;

// fillRule
pub const EvenOddRule: c_int = 0;
pub const WindingRule: c_int = 1;

// subwindow mode
pub const ClipByChildren: c_int = 0;
pub const IncludeInferiors: c_int = 1;

// CoordinateMode for drawing routines
pub const CoordModeOrigin: c_int = 0;
pub const CoordModePrevious: c_int = 1;

// Polygon shapes
pub const Complex: c_int = 0;
pub const Nonconvex: c_int = 1;
pub const Convex: c_int = 2;

// Arc modes for PolyFillArc
pub const ArcChord: c_int = 0;
pub const ArcPieSlice: c_int = 1;

// GC components
pub const GCFunction: c_uint = (1 << 0);
pub const GCPlaneMask: c_uint = (1 << 1);
pub const GCForeground: c_uint = (1 << 2);
pub const GCBackground: c_uint = (1 << 3);
pub const GCLineWidth: c_uint = (1 << 4);
pub const GCLineStyle: c_uint = (1 << 5);
pub const GCCapStyle: c_uint = (1 << 6);
pub const GCJoinStyle: c_uint = (1 << 7);
pub const GCFillStyle: c_uint = (1 << 8);
pub const GCFillRule: c_uint = (1 << 9);
pub const GCTile: c_uint = (1 << 10);
pub const GCStipple: c_uint = (1 << 11);
pub const GCTileStipXOrigin: c_uint = (1 << 12);
pub const GCTileStipYOrigin: c_uint = (1 << 13);
pub const GCFont: c_uint = (1 << 14);
pub const GCSubwindowMode: c_uint = (1 << 15);
pub const GCGraphicsExposures: c_uint = (1 << 16);
pub const GCClipXOrigin: c_uint = (1 << 17);
pub const GCClipYOrigin: c_uint = (1 << 18);
pub const GCClipMask: c_uint = (1 << 19);
pub const GCDashOffset: c_uint = (1 << 20);
pub const GCDashList: c_uint = (1 << 21);
pub const GCArcMode: c_uint = (1 << 22);

pub const GCLastBit: c_uint = 22;

// draw direction
pub const FontLeftToRight: c_int = 0;
pub const FontRightToLeft: c_int = 1;

pub const FontChange: c_uchar = 255;

// QueryBestSize Class
pub const CursorShape: c_int = 0;
pub const TileShape: c_int = 1;
pub const StippleShape: c_int = 2;

// keyboard autorepeat
pub const AutoRepeatModeOff: c_int = 0;
pub const AutoRepeatModeOn: c_int = 1;
pub const AutoRepeatModeDefault: c_int = 2;

pub const LedModeOff: c_int = 0;
pub const LedModeOn: c_int = 1;

// masks for ChangeKeyboardControl
pub const KBKeyClickPercent: c_ulong = (1 << 0);
pub const KBBellPercent: c_ulong = (1 << 1);
pub const KBBellPitch: c_ulong = (1 << 2);
pub const KBBellDuration: c_ulong = (1 << 3);
pub const KBLed: c_ulong = (1 << 4);
pub const KBLedMode: c_ulong = (1 << 5);
pub const KBKey: c_ulong = (1 << 6);
pub const KBAutoRepeatMode: c_ulong = (1 << 7);

pub const MappingSuccess: c_uchar = 0;
pub const MappingBusy: c_uchar = 1;
pub const MappingFailed: c_uchar = 2;

pub const MappingModifier: c_int = 0;
pub const MappingKeyboard: c_int = 1;
pub const MappingPointer: c_int = 2;

// screensaver
pub const DontPreferBlanking: c_int = 0;
pub const PreferBlanking: c_int = 1;
pub const DefaultBlanking: c_int = 2;

pub const DisableScreenSaver: c_int = 0;
pub const DisableScreenInterval: c_int = 0;

pub const DontAllowExposures: c_int = 0;
pub const AllowExposures: c_int = 1;
pub const DefaultExposures: c_int = 2;

pub const ScreenSaverReset: c_int = 0;
pub const ScreenSaverActive: c_int = 1;

// hosts and connections
pub const HostInsert: c_uchar = 0;
pub const HostDelete: c_uchar = 1;

pub const EnableAccess: c_int = 1;
pub const DisableAccess: c_int = 0;

// visual class
pub const StaticGray: c_int = 0;
pub const GrayScale: c_int = 1;
pub const StaticColor: c_int = 2;
pub const PseudoColor: c_int = 3;
pub const TrueColor: c_int = 4;
pub const DirectColor: c_int = 5;

// visual info mask
pub const VisualNoMask: c_long = 0x0000;
pub const VisualIDMask: c_long = 0x0001;
pub const VisualScreenMask: c_long = 0x0002;
pub const VisualDepthMask: c_long = 0x0004;
pub const VisualClassMask: c_long = 0x0008;
pub const VisualRedMaskMask: c_long = 0x0010;
pub const VisualGreenMaskMask: c_long = 0x0020;
pub const VisualBlueMaskMask: c_long = 0x0040;
pub const VisualColormapSizeMask: c_long = 0x0080;
pub const VisualBitsPerRGBMask: c_long = 0x0100;
pub const VisualAllMask: c_long = 0x01ff;

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

// window classes
pub const InputOutput: c_int = 1;
pub const InputOnly: c_int = 2;

// XCreateIC values
pub const XIMPreeditArea: c_int = 0x0001;
pub const XIMPreeditCallbacks: c_int = 0x0002;
pub const XIMPreeditPosition: c_int = 0x0004;
pub const XIMPreeditNothing: c_int = 0x0008;
pub const XIMPreeditNone: c_int = 0x0010;
pub const XIMStatusArea: c_int = 0x0100;
pub const XIMStatusCallbacks: c_int = 0x0200;
pub const XIMStatusNothing: c_int = 0x0400;
pub const XIMStatusNone: c_int = 0x0800;

// Byte order  used in imageByteOrder and bitmapBitOrder
pub const LSBFirst: c_int = 0;
pub const MSBFirst: c_int = 1;

// Reserved resource and constant definitions
//pub const None: c_int = 0;
pub const ParentRelative: c_int = 1;
pub const CopyFromParent: c_int = 0;
pub const PointerWindow: c_int = 0;
pub const InputFocus: c_int = 1;
pub const PointerRoot: c_int = 1;
pub const AnyPropertyType: c_int = 0;
pub const AnyKey: c_int = 0;
pub const AnyButton: c_int = 0;
pub const AllTemporary: c_int = 0;
pub const CurrentTime: Time = 0;
pub const NoSymbol: c_int = 0;

/* Definitions for the X window system likely to be used by applications */
pub const X_PROTOCOL: c_int = 11;
pub const X_PROTOCOL_REVISION: c_int = 0;

pub const XNVaNestedList: &'static str = "XNVaNestedList";
pub const XNQueryInputStyle: &'static str = "queryInputStyle";
pub const XNClientWindow: &'static str = "clientWindow";
pub const XNInputStyle: &'static str = "inputStyle";
pub const XNFocusWindow: &'static str = "focusWindow";
pub const XNResourceName: &'static str = "resourceName";
pub const XNResourceClass: &'static str = "resourceClass";
pub const XNGeometryCallback: &'static str = "geometryCallback";
pub const XNDestroyCallback: &'static str = "destroyCallback";
pub const XNFilterEvents: &'static str = "filterEvents";
pub const XNPreeditStartCallback: &'static str = "preeditStartCallback";
pub const XNPreeditDoneCallback: &'static str = "preeditDoneCallback";
pub const XNPreeditDrawCallback: &'static str = "preeditDrawCallback";
pub const XNPreeditCaretCallback: &'static str = "preeditCaretCallback";
pub const XNPreeditStateNotifyCallback: &'static str = "preeditStateNotifyCallback";
pub const XNPreeditAttributes: &'static str = "preeditAttributes";
pub const XNStatusStartCallback: &'static str = "statusStartCallback";
pub const XNStatusDoneCallback: &'static str = "statusDoneCallback";
pub const XNStatusDrawCallback: &'static str = "statusDrawCallback";
pub const XNStatusAttributes: &'static str = "statusAttributes";
pub const XNArea: &'static str = "area";
pub const XNAreaNeeded: &'static str = "areaNeeded";
pub const XNSpotLocation: &'static str = "spotLocation";
pub const XNColormap: &'static str = "colorMap";
pub const XNStdColormap: &'static str = "stdColorMap";
pub const XNForeground: &'static str = "foreground";
pub const XNBackground: &'static str = "background";
pub const XNBackgroundPixmap: &'static str = "backgroundPixmap";
pub const XNFontSet: &'static str = "fontSet";
pub const XNLineSpace: &'static str = "lineSpace";
pub const XNCursor: &'static str = "cursor";

pub const XNVaNestedList_0: &'static [u8] = b"XNVaNestedList\0";
pub const XNQueryInputStyle_0: &'static [u8] = b"queryInputStyle\0";
pub const XNClientWindow_0: &'static [u8] = b"clientWindow\0";
pub const XNInputStyle_0: &'static [u8] = b"inputStyle\0";
pub const XNFocusWindow_0: &'static [u8] = b"focusWindow\0";
pub const XNResourceName_0: &'static [u8] = b"resourceName\0";
pub const XNResourceClass_0: &'static [u8] = b"resourceClass\0";
pub const XNGeometryCallback_0: &'static [u8] = b"geometryCallback\0";
pub const XNDestroyCallback_0: &'static [u8] = b"destroyCallback\0";
pub const XNFilterEvents_0: &'static [u8] = b"filterEvents\0";
pub const XNPreeditStartCallback_0: &'static [u8] = b"preeditStartCallback\0";
pub const XNPreeditDoneCallback_0: &'static [u8] = b"preeditDoneCallback\0";
pub const XNPreeditDrawCallback_0: &'static [u8] = b"preeditDrawCallback\0";
pub const XNPreeditCaretCallback_0: &'static [u8] = b"preeditCaretCallback\0";
pub const XNPreeditStateNotifyCallback_0: &'static [u8] = b"preeditStateNotifyCallback\0";
pub const XNPreeditAttributes_0: &'static [u8] = b"preeditAttributes\0";
pub const XNStatusStartCallback_0: &'static [u8] = b"statusStartCallback\0";
pub const XNStatusDoneCallback_0: &'static [u8] = b"statusDoneCallback\0";
pub const XNStatusDrawCallback_0: &'static [u8] = b"statusDrawCallback\0";
pub const XNStatusAttributes_0: &'static [u8] = b"statusAttributes\0";
pub const XNArea_0: &'static [u8] = b"area\0";
pub const XNAreaNeeded_0: &'static [u8] = b"areaNeeded\0";
pub const XNSpotLocation_0: &'static [u8] = b"spotLocation\0";
pub const XNColormap_0: &'static [u8] = b"colorMap\0";
pub const XNStdColormap_0: &'static [u8] = b"stdColorMap\0";
pub const XNForeground_0: &'static [u8] = b"foreground\0";
pub const XNBackground_0: &'static [u8] = b"background\0";
pub const XNBackgroundPixmap_0: &'static [u8] = b"backgroundPixmap\0";
pub const XNFontSet_0: &'static [u8] = b"fontSet\0";
pub const XNLineSpace_0: &'static [u8] = b"lineSpace\0";
pub const XNCursor_0: &'static [u8] = b"cursor\0";

pub const XNQueryIMValuesList: &'static str = "queryIMValuesList";
pub const XNQueryICValuesList: &'static str = "queryICValuesList";
pub const XNVisiblePosition: &'static str = "visiblePosition";
pub const XNR6PreeditCallback: &'static str = "r6PreeditCallback";
pub const XNStringConversionCallback: &'static str = "stringConversionCallback";
pub const XNStringConversion: &'static str = "stringConversion";
pub const XNResetState: &'static str = "resetState";
pub const XNHotKey: &'static str = "hotKey";
pub const XNHotKeyState: &'static str = "hotKeyState";
pub const XNPreeditState: &'static str = "preeditState";
pub const XNSeparatorofNestedList: &'static str = "separatorofNestedList";

pub const XNQueryIMValuesList_0: &'static [u8] = b"queryIMValuesList\0";
pub const XNQueryICValuesList_0: &'static [u8] = b"queryICValuesList\0";
pub const XNVisiblePosition_0: &'static [u8] = b"visiblePosition\0";
pub const XNR6PreeditCallback_0: &'static [u8] = b"r6PreeditCallback\0";
pub const XNStringConversionCallback_0: &'static [u8] = b"stringConversionCallback\0";
pub const XNStringConversion_0: &'static [u8] = b"stringConversion\0";
pub const XNResetState_0: &'static [u8] = b"resetState\0";
pub const XNHotKey_0: &'static [u8] = b"hotKey\0";
pub const XNHotKeyState_0: &'static [u8] = b"hotKeyState\0";
pub const XNPreeditState_0: &'static [u8] = b"preeditState\0";
pub const XNSeparatorofNestedList_0: &'static [u8] = b"separatorofNestedList\0";

pub const XBufferOverflow: i32 = -1;
pub const XLookupNone: i32 = 1;
pub const XLookupChars: i32 = 2;
pub const XLookupKeySym: i32 = 3;
pub const XLookupBoth: i32 = 4;

// Xkb constants
pub const XkbActionMessageLength: usize = 6;

pub const XkbOD_Success: c_int = 0;
pub const XkbOD_BadLibraryVersion: c_int = 1;
pub const XkbOD_ConnectionRefused: c_int = 2;
pub const XkbOD_NonXkbServer: c_int = 3;
pub const XkbOD_BadServerVersion: c_int = 4;

pub const XkbLC_ForceLatinLookup: c_uint = 1 << 0;
pub const XkbLC_ConsumeLookupMods: c_uint = 1 << 1;
pub const XkbLC_AlwaysConsumeShiftAndLock: c_uint = 1 << 2;
pub const XkbLC_IgnoreNewKeyboards: c_uint = 1 << 3;
pub const XkbLC_ControlFallback: c_uint = 1 << 4;
pub const XkbLC_ConsumeKeysOnComposeFail: c_uint = 1 << 29;
pub const XkbLC_ComposeLED: c_uint = 1 << 30;
pub const XkbLC_BeepOnComposeFail: c_uint = 1 << 31;

pub const XkbLC_AllComposeControls: c_uint = 0xc000_0000;
pub const XkbLC_AllControls: c_uint = 0xc000_001f;

pub const XkbNewKeyboardNotify: c_int = 0;
pub const XkbMapNotify: c_int = 1;
pub const XkbStateNotify: c_int = 2;
pub const XkbControlsNotify: c_int = 3;
pub const XkbIndicatorStateNotify: c_int = 4;
pub const XkbIndicatorMapNotify: c_int = 5;
pub const XkbNamesNotify: c_int = 6;
pub const XkbCompatMapNotify: c_int = 7;
pub const XkbBellNotify: c_int = 8;
pub const XkbActionMessage: c_int = 9;
pub const XkbAccessXNotify: c_int = 10;
pub const XkbExtensionDeviceNotify: c_int = 11;

pub const XkbNewKeyboardNotifyMask: c_ulong = 1 << 0;
pub const XkbMapNotifyMask: c_ulong = 1 << 1;
pub const XkbStateNotifyMask: c_ulong = 1 << 2;
pub const XkbControlsNotifyMask: c_ulong = 1 << 3;
pub const XkbIndicatorStateNotifyMask: c_ulong = 1 << 4;
pub const XkbIndicatorMapNotifyMask: c_ulong = 1 << 5;
pub const XkbNamesNotifyMask: c_ulong = 1 << 6;
pub const XkbCompatMapNotifyMask: c_ulong = 1 << 7;
pub const XkbBellNotifyMask: c_ulong = 1 << 8;
pub const XkbActionMessageMask: c_ulong = 1 << 9;
pub const XkbAccessXNotifyMask: c_ulong = 1 << 10;
pub const XkbExtensionDeviceNotifyMask: c_ulong = 1 << 11;
pub const XkbAllEventsMask: c_ulong = 0xfff;

// Bitmask returned by XParseGeometry
pub const NoValue: c_int = 0x0000;
pub const XValue: c_int = 0x0001;
pub const YValue: c_int = 0x0002;
pub const WidthValue: c_int = 0x0004;
pub const HeightValue: c_int = 0x0008;
pub const AllValues: c_int = 0x000f;
pub const XNegative: c_int = 0x0010;
pub const YNegative: c_int = 0x0020;

// Definition for flags of XWMHints
pub const InputHint: c_long = 1 << 0;
pub const StateHint: c_long = 1 << 1;
pub const IconPixmapHint: c_long = 1 << 2;
pub const IconWindowHint: c_long = 1 << 3;
pub const IconPositionHint: c_long = 1 << 4;
pub const IconMaskHint: c_long = 1 << 5;
pub const WindowGroupHint: c_long = 1 << 6;
pub const AllHints: c_long = InputHint
    | StateHint
    | IconPixmapHint
    | IconWindowHint
    | IconPositionHint
    | IconMaskHint
    | WindowGroupHint;
pub const XUrgencyHint: c_long = 1 << 8;

// XICCEncodingStyle
pub const XStringStyle: c_int = 0;
pub const XCompoundTextStyle: c_int = 1;
pub const XTextStyle: c_int = 2;
pub const XStdICCTextStyle: c_int = 3;
pub const XUTF8StringStyle: c_int = 4;
