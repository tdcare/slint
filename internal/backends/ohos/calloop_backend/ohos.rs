/// 枚举API访问状态
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_Result {
    /// 成功
    OH_NATIVEXCOMPONENT_RESULT_SUCCESS = 0,
    /// 失败
    OH_NATIVEXCOMPONENT_RESULT_FAILED = -1,
    /// 无效参数
    OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER = -2,
}

/// 触摸事件类型枚举
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_TouchEventType {
    /// 当手指按下时触发触摸事件
    OH_NATIVEXCOMPONENT_DOWN = 0,
    /// 当手指抬起时触发触摸事件
    OH_NATIVEXCOMPONENT_UP,
    /// 当手指在屏幕上移动时触发触摸事件
    OH_NATIVEXCOMPONENT_MOVE,
    /// 当触摸事件被取消时触发事件
    OH_NATIVEXCOMPONENT_CANCEL,
    /// 无效的触摸类型
    OH_NATIVEXCOMPONENT_UNKNOWN,
}

/// 触摸点工具类型枚举
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_TouchPointToolType {
    /// 无效的工具类型
    OH_NATIVEXCOMPONENT_TOOL_TYPE_UNKNOWN = 0,
    /// 手指
    OH_NATIVEXCOMPONENT_TOOL_TYPE_FINGER,
    /// 笔
    OH_NATIVEXCOMPONENT_TOOL_TYPE_PEN,
    /// 橡皮擦
    OH_NATIVEXCOMPONENT_TOOL_TYPE_RUBBER,
    /// 画笔
    OH_NATIVEXCOMPONENT_TOOL_TYPE_BRUSH,
    /// 铅笔
    OH_NATIVEXCOMPONENT_TOOL_TYPE_PENCIL,
    /// 气刷
    OH_NATIVEXCOMPONENT_TOOL_TYPE_AIRBRUSH,
    /// 鼠标
    OH_NATIVEXCOMPONENT_TOOL_TYPE_MOUSE,
    /// 镜头
    OH_NATIVEXCOMPONENT_TOOL_TYPE_LENS,
}

/// 事件源类型枚举
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_EventSourceType {
    /// 未知的输入源类型
    OH_NATIVEXCOMPONENT_SOURCE_TYPE_UNKNOWN = 0,
    /// 鼠标多点触控事件输入源
    OH_NATIVEXCOMPONENT_SOURCE_TYPE_MOUSE,
    /// 触摸屏多点触控事件输入源
    OH_NATIVEXCOMPONENT_SOURCE_TYPE_TOUCHSCREEN,
    /// 触摸板多点触控事件输入源
    OH_NATIVEXCOMPONENT_SOURCE_TYPE_TOUCHPAD,
    /// 游戏手柄多点触控事件输入源
    OH_NATIVEXCOMPONENT_SOURCE_TYPE_JOYSTICK,
    /**
     * @brief Indicates that the input source generates a keyboard event.
     *
     * @since 10
     * @version 1.0
     */
    OH_NATIVEXCOMPONENT_SOURCE_TYPE_KEYBOARD,
}

/// 鼠标事件动作枚举
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_MouseEventAction {
    OH_NATIVEXCOMPONENT_MOUSE_NONE = 0,
    OH_NATIVEXCOMPONENT_MOUSE_PRESS,
    OH_NATIVEXCOMPONENT_MOUSE_RELEASE,
    OH_NATIVEXCOMPONENT_MOUSE_MOVE,
}
/// 按键动作
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_KeyAction{
    OH_NATIVEXCOMPONENT_KEY_ACTION_UNKNOWN = -1,
    OH_NATIVEXCOMPONENT_KEY_ACTION_DOWN = 0,
    OH_NATIVEXCOMPONENT_KEY_ACTION_UP,
}
/// 按键码
#[derive(Copy,Clone)]
#[repr(C)]
pub  enum OH_NativeXComponent_KeyCode {
    KEY_UNKNOWN = -1,
    KEY_FN = 0,
    KEY_HOME = 1,
    KEY_BACK = 2,
    KEY_MEDIA_PLAY_PAUSE = 10,
    KEY_MEDIA_STOP = 11,
    KEY_MEDIA_NEXT = 12,
    KEY_MEDIA_PREVIOUS = 13,
    KEY_MEDIA_REWIND = 14,
    KEY_MEDIA_FAST_FORWARD = 15,
    KEY_VOLUME_UP = 16,
    KEY_VOLUME_DOWN = 17,
    KEY_POWER = 18,
    KEY_CAMERA = 19,
    KEY_VOLUME_MUTE = 22,
    KEY_MUTE = 23,
    KEY_BRIGHTNESS_UP = 40,
    KEY_BRIGHTNESS_DOWN = 41,
    KEY_0 = 2000,
    KEY_1 = 2001,
    KEY_2 = 2002,
    KEY_3 = 2003,
    KEY_4 = 2004,
    KEY_5 = 2005,
    KEY_6 = 2006,
    KEY_7 = 2007,
    KEY_8 = 2008,
    KEY_9 = 2009,
    KEY_STAR = 2010,
    KEY_POUND = 2011,
    KEY_DPAD_UP = 2012,
    KEY_DPAD_DOWN = 2013,
    KEY_DPAD_LEFT = 2014,
    KEY_DPAD_RIGHT = 2015,
    KEY_DPAD_CENTER = 2016,
    KEY_A = 2017,
    KEY_B = 2018,
    KEY_C = 2019,
    KEY_D = 2020,
    KEY_E = 2021,
    KEY_F = 2022,
    KEY_G = 2023,
    KEY_H = 2024,
    KEY_I = 2025,
    KEY_J = 2026,
    KEY_K = 2027,
    KEY_L = 2028,
    KEY_M = 2029,
    KEY_N = 2030,
    KEY_O = 2031,
    KEY_P = 2032,
    KEY_Q = 2033,
    KEY_R = 2034,
    KEY_S = 2035,
    KEY_T = 2036,
    KEY_U = 2037,
    KEY_V = 2038,
    KEY_W = 2039,
    KEY_X = 2040,
    KEY_Y = 2041,
    KEY_Z = 2042,
    KEY_COMMA = 2043,
    KEY_PERIOD = 2044,
    KEY_ALT_LEFT = 2045,
    KEY_ALT_RIGHT = 2046,
    KEY_SHIFT_LEFT = 2047,
    KEY_SHIFT_RIGHT = 2048,
    KEY_TAB = 2049,
    KEY_SPACE = 2050,
    KEY_SYM = 2051,
    KEY_EXPLORER = 2052,
    KEY_ENVELOPE = 2053,
    KEY_ENTER = 2054,
    KEY_DEL = 2055,
    KEY_GRAVE = 2056,
    KEY_MINUS = 2057,
    KEY_EQUALS = 2058,
    KEY_LEFT_BRACKET = 2059,
    KEY_RIGHT_BRACKET = 2060,
    KEY_BACKSLASH = 2061,
    KEY_SEMICOLON = 2062,
    KEY_APOSTROPHE = 2063,
    KEY_SLASH = 2064,
    KEY_AT = 2065,
    KEY_PLUS = 2066,
    KEY_MENU = 2067,
    KEY_PAGE_UP = 2068,
    KEY_PAGE_DOWN = 2069,
    KEY_ESCAPE = 2070,
    KEY_FORWARD_DEL = 2071,
    KEY_CTRL_LEFT = 2072,
    KEY_CTRL_RIGHT = 2073,
    KEY_CAPS_LOCK = 2074,
    KEY_SCROLL_LOCK = 2075,
    KEY_META_LEFT = 2076,
    KEY_META_RIGHT = 2077,
    KEY_FUNCTION = 2078,
    KEY_SYSRQ = 2079,
    KEY_BREAK = 2080,
    KEY_MOVE_HOME = 2081,
    KEY_MOVE_END = 2082,
    KEY_INSERT = 2083,
    KEY_FORWARD = 2084,
    KEY_MEDIA_PLAY = 2085,
    KEY_MEDIA_PAUSE = 2086,
    KEY_MEDIA_CLOSE = 2087,
    KEY_MEDIA_EJECT = 2088,
    KEY_MEDIA_RECORD = 2089,
    KEY_F1 = 2090,
    KEY_F2 = 2091,
    KEY_F3 = 2092,
    KEY_F4 = 2093,
    KEY_F5 = 2094,
    KEY_F6 = 2095,
    KEY_F7 = 2096,
    KEY_F8 = 2097,
    KEY_F9 = 2098,
    KEY_F10 = 2099,
    KEY_F11 = 2100,
    KEY_F12 = 2101,
    KEY_NUM_LOCK = 2102,
    KEY_NUMPAD_0 = 2103,
    KEY_NUMPAD_1 = 2104,
    KEY_NUMPAD_2 = 2105,
    KEY_NUMPAD_3 = 2106,
    KEY_NUMPAD_4 = 2107,
    KEY_NUMPAD_5 = 2108,
    KEY_NUMPAD_6 = 2109,
    KEY_NUMPAD_7 = 2110,
    KEY_NUMPAD_8 = 2111,
    KEY_NUMPAD_9 = 2112,
    KEY_NUMPAD_DIVIDE = 2113,
    KEY_NUMPAD_MULTIPLY = 2114,
    KEY_NUMPAD_SUBTRACT = 2115,
    KEY_NUMPAD_ADD = 2116,
    KEY_NUMPAD_DOT = 2117,
    KEY_NUMPAD_COMMA = 2118,
    KEY_NUMPAD_ENTER = 2119,
    KEY_NUMPAD_EQUALS = 2120,
    KEY_NUMPAD_LEFT_PAREN = 2121,
    KEY_NUMPAD_RIGHT_PAREN = 2122,
    KEY_VIRTUAL_MULTITASK = 2210,
    KEY_SLEEP = 2600,
    KEY_ZENKAKU_HANKAKU = 2601,
    KEY_102ND = 2602,
    KEY_RO = 2603,
    KEY_KATAKANA = 2604,
    KEY_HIRAGANA = 2605,
    KEY_HENKAN = 2606,
    KEY_KATAKANA_HIRAGANA = 2607,
    KEY_MUHENKAN = 2608,
    KEY_LINEFEED = 2609,
    KEY_MACRO = 2610,
    KEY_NUMPAD_PLUSMINUS = 2611,
    KEY_SCALE = 2612,
    KEY_HANGUEL = 2613,
    KEY_HANJA = 2614,
    KEY_YEN = 2615,
    KEY_STOP = 2616,
    KEY_AGAIN = 2617,
    KEY_PROPS = 2618,
    KEY_UNDO = 2619,
    KEY_COPY = 2620,
    KEY_OPEN = 2621,
    KEY_PASTE = 2622,
    KEY_FIND = 2623,
    KEY_CUT = 2624,
    KEY_HELP = 2625,
    KEY_CALC = 2626,
    KEY_FILE = 2627,
    KEY_BOOKMARKS = 2628,
    KEY_NEXT = 2629,
    KEY_PLAYPAUSE = 2630,
    KEY_PREVIOUS = 2631,
    KEY_STOPCD = 2632,
    KEY_CONFIG = 2634,
    KEY_REFRESH = 2635,
    KEY_EXIT = 2636,
    KEY_EDIT = 2637,
    KEY_SCROLLUP = 2638,
    KEY_SCROLLDOWN = 2639,
    KEY_NEW = 2640,
    KEY_REDO = 2641,
    KEY_CLOSE = 2642,
    KEY_PLAY = 2643,
    KEY_BASSBOOST = 2644,
    KEY_PRINT = 2645,
    KEY_CHAT = 2646,
    KEY_FINANCE = 2647,
    KEY_CANCEL = 2648,
    KEY_KBDILLUM_TOGGLE = 2649,
    KEY_KBDILLUM_DOWN = 2650,
    KEY_KBDILLUM_UP = 2651,
    KEY_SEND = 2652,
    KEY_REPLY = 2653,
    KEY_FORWARDMAIL = 2654,
    KEY_SAVE = 2655,
    KEY_DOCUMENTS = 2656,
    KEY_VIDEO_NEXT = 2657,
    KEY_VIDEO_PREV = 2658,
    KEY_BRIGHTNESS_CYCLE = 2659,
    KEY_BRIGHTNESS_ZERO = 2660,
    KEY_DISPLAY_OFF = 2661,
    KEY_BTN_MISC = 2662,
    KEY_GOTO = 2663,
    KEY_INFO = 2664,
    KEY_PROGRAM = 2665,
    KEY_PVR = 2666,
    KEY_SUBTITLE = 2667,
    KEY_FULL_SCREEN = 2668,
    KEY_KEYBOARD = 2669,
    KEY_ASPECT_RATIO = 2670,
    KEY_PC = 2671,
    KEY_TV = 2672,
    KEY_TV2 = 2673,
    KEY_VCR = 2674,
    KEY_VCR2 = 2675,
    KEY_SAT = 2676,
    KEY_CD = 2677,
    KEY_TAPE = 2678,
    KEY_TUNER = 2679,
    KEY_PLAYER = 2680,
    KEY_DVD = 2681,
    KEY_AUDIO = 2682,
    KEY_VIDEO = 2683,
    KEY_MEMO = 2684,
    KEY_CALENDAR = 2685,
    KEY_RED = 2686,
    KEY_GREEN = 2687,
    KEY_YELLOW = 2688,
    KEY_BLUE = 2689,
    KEY_CHANNELUP = 2690,
    KEY_CHANNELDOWN = 2691,
    KEY_LAST = 2692,
    KEY_RESTART = 2693,
    KEY_SLOW = 2694,
    KEY_SHUFFLE = 2695,
    KEY_VIDEOPHONE = 2696,
    KEY_GAMES = 2697,
    KEY_ZOOMIN = 2698,
    KEY_ZOOMOUT = 2699,
    KEY_ZOOMRESET = 2700,
    KEY_WORDPROCESSOR = 2701,
    KEY_EDITOR = 2702,
    KEY_SPREADSHEET = 2703,
    KEY_GRAPHICSEDITOR = 2704,
    KEY_PRESENTATION = 2705,
    KEY_DATABASE = 2706,
    KEY_NEWS = 2707,
    KEY_VOICEMAIL = 2708,
    KEY_ADDRESSBOOK = 2709,
    KEY_MESSENGER = 2710,
    KEY_BRIGHTNESS_TOGGLE = 2711,
    KEY_SPELLCHECK = 2712,
    KEY_COFFEE = 2713,
    KEY_MEDIA_REPEAT = 2714,
    KEY_IMAGES = 2715,
    KEY_BUTTONCONFIG = 2716,
    KEY_TASKMANAGER = 2717,
    KEY_JOURNAL = 2718,
    KEY_CONTROLPANEL = 2719,
    KEY_APPSELECT = 2720,
    KEY_SCREENSAVER = 2721,
    KEY_ASSISTANT = 2722,
    KEY_KBD_LAYOUT_NEXT = 2723,
    KEY_BRIGHTNESS_MIN = 2724,
    KEY_BRIGHTNESS_MAX = 2725,
    KEY_KBDINPUTASSIST_PREV = 2726,
    KEY_KBDINPUTASSIST_NEXT = 2727,
    KEY_KBDINPUTASSIST_PREVGROUP = 2728,
    KEY_KBDINPUTASSIST_NEXTGROUP = 2729,
    KEY_KBDINPUTASSIST_ACCEPT = 2730,
    KEY_KBDINPUTASSIST_CANCEL = 2731,
    KEY_FRONT = 2800,
    KEY_SETUP = 2801,
    KEY_WAKEUP = 2802,
    KEY_SENDFILE = 2803,
    KEY_DELETEFILE = 2804,
    KEY_XFER = 2805,
    KEY_PROG1 = 2806,
    KEY_PROG2 = 2807,
    KEY_MSDOS = 2808,
    KEY_SCREENLOCK = 2809,
    KEY_DIRECTION_ROTATE_DISPLAY = 2810,
    KEY_CYCLEWINDOWS = 2811,
    KEY_COMPUTER = 2812,
    KEY_EJECTCLOSECD = 2813,
    KEY_ISO = 2814,
    KEY_MOVE = 2815,
    KEY_F13 = 2816,
    KEY_F14 = 2817,
    KEY_F15 = 2818,
    KEY_F16 = 2819,
    KEY_F17 = 2820,
    KEY_F18 = 2821,
    KEY_F19 = 2822,
    KEY_F20 = 2823,
    KEY_F21 = 2824,
    KEY_F22 = 2825,
    KEY_F23 = 2826,
    KEY_F24 = 2827,
    KEY_PROG3 = 2828,
    KEY_PROG4 = 2829,
    KEY_DASHBOARD = 2830,
    KEY_SUSPEND = 2831,
    KEY_HP = 2832,
    KEY_SOUND = 2833,
    KEY_QUESTION = 2834,
    KEY_CONNECT = 2836,
    KEY_SPORT = 2837,
    KEY_SHOP = 2838,
    KEY_ALTERASE = 2839,
    KEY_SWITCHVIDEOMODE = 2841,
    KEY_BATTERY = 2842,
    KEY_BLUETOOTH = 2843,
    KEY_WLAN = 2844,
    KEY_UWB = 2845,
    KEY_WWAN_WIMAX = 2846,
    KEY_RFKILL = 2847,
    KEY_CHANNEL = 3001,
    KEY_BTN_0 = 3100,
    KEY_BTN_1 = 3101,
    KEY_BTN_2 = 3102,
    KEY_BTN_3 = 3103,
    KEY_BTN_4 = 3104,
    KEY_BTN_5 = 3105,
    KEY_BTN_6 = 3106,
    KEY_BTN_7 = 3107,
    KEY_BTN_8 = 3108,
    KEY_BTN_9 = 3109,
}


/// 鼠标事件按钮枚举
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OH_NativeXComponent_MouseEventButton {
    OH_NATIVEXCOMPONENT_NONE_BUTTON = 0,
    OH_NATIVEXCOMPONENT_LEFT_BUTTON = 0x01,
    OH_NATIVEXCOMPONENT_RIGHT_BUTTON = 0x02,
    OH_NATIVEXCOMPONENT_MIDDLE_BUTTON = 0x04,
    OH_NATIVEXCOMPONENT_BACK_BUTTON = 0x08,
    OH_NATIVEXCOMPONENT_FORWARD_BUTTON = 0x10,
}

/// 触摸点结构体
#[derive(Copy,Clone)]
#[repr(C)]
pub struct OH_NativeXComponent_TouchPoint {
    /// 手指的唯一标识符
    pub id: i32,
    /// 触摸点相对于屏幕左边缘的X坐标
    pub screenX: f32,
    /// 触摸点相对于屏幕上边缘的Y坐标
    pub screenY: f32,
    /// 触摸点相对于触摸元素左边缘的X坐标
    pub x: f32,
    /// 触摸点相对于触摸元素上边缘的Y坐标
    pub y: f32,
    /// 触摸事件类型
    pub r#type: OH_NativeXComponent_TouchEventType,
    /// 手指与屏幕之间的接触面积
    pub size: f64,
    /// 当前触摸事件的压力
    pub force: f32,
    /// 当前触摸事件的时间戳
    pub timeStamp: i64,
    /// 当前点是否被按下
    pub isPressed: bool,
}

/// 触摸事件结构体
#[derive(Copy,Clone)]
#[repr(C)]
pub struct OH_NativeXComponent_TouchEvent {
    /// 手指的唯一标识符
    pub id: i32,
    /// 触摸点相对于屏幕左边缘的X坐标
    pub screenX: f32,
    /// 触摸点相对于屏幕上边缘的Y坐标
    pub screenY: f32,
    /// 触摸点相对于触摸元素左边缘的X坐标
    pub x: f32,
    /// 触摸点相对于触摸元素上边缘的Y坐标
    pub y: f32,
    /// 触摸事件类型
    pub r#type: OH_NativeXComponent_TouchEventType,
    /// 手指与屏幕之间的接触面积
    pub size: f64,
    /// 当前触摸事件的压力
    pub force: f32,
    /// 生成当前触摸事件的设备ID
    pub deviceId: i64,
    /// 当前触摸事件的时间戳
    pub timeStamp: i64,
    /// 当前触摸点数组
    pub touchPoints: Option<[OH_NativeXComponent_TouchPoint; 10]>,
    /// 当前触摸点数量
    pub numPoints: u32,
}

/// 鼠标事件结构体
#[derive(Copy,Clone)]
#[repr(C)]
pub struct OH_NativeXComponent_MouseEvent {
    /** 鼠标点相对于鼠标元素左边缘的X坐标 */
    pub x: f32,
    /** 鼠标点相对于鼠标元素上边缘的Y坐标 */
    pub y: f32,
    /** 鼠标点相对于屏幕左边缘的X坐标 */
    pub screenX: f32,
    /** 鼠标点相对于屏幕上边缘的Y坐标 */
    pub screenY: f32,
    /** 当前鼠标事件的时间戳 */
    pub timestamp: i64,
    /** 鼠标事件动作 */
    pub action: OH_NativeXComponent_MouseEventAction,
    /** 鼠标事件按钮 */
    pub button: OH_NativeXComponent_MouseEventButton,
}


/// 按键事件结构体
#[derive(Copy,Clone)]
#[repr(C)]
pub struct OH_NativeXComponent_KeyEvent{
    pub deviceId:i64,
    pub timeStamp:i64,
    pub source_type:OH_NativeXComponent_EventSourceType,
    pub key_action:OH_NativeXComponent_KeyAction,
    pub key_code:OH_NativeXComponent_KeyCode,
}


/// 鸿蒙系统的输入事件
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OHOS_Input_Event{
    ///鼠标事件
    MouseEvent(OH_NativeXComponent_MouseEvent),
    ///鼠标悬浮事件
    MouseHoverEvent,
    ///触摸事件
    TouchEvent(OH_NativeXComponent_TouchEvent),
    ///焦点事件
    FocusEvent,
    ///模糊事件
    BlurEvent,
    ///按键事件
    KeyEvent(OH_NativeXComponent_KeyEvent),
    NoEvent,
}

unsafe impl Send for OHOS_Input_Event {}
unsafe impl Sync for OHOS_Input_Event {}
