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

/// 鸿蒙系统的输入事件
#[derive(Copy,Clone)]
#[repr(C)]
pub enum OHOS_Input_Event{
    MouseEvent(OH_NativeXComponent_MouseEvent),
    TouchEvent(OH_NativeXComponent_TouchEvent),
    NoEvent,
}

unsafe impl Send for OHOS_Input_Event {}
unsafe impl Sync for OHOS_Input_Event {}
