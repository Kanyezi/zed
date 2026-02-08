use anyhow::Result;
use gpui::{
    actions, div, prelude::*, App, AsyncWindowContext, Context, EventEmitter, Entity, Focusable,
    FocusHandle, IntoElement, Render, WeakEntity, Window,
};
use i18n::t;
use ui::{prelude::*, IconName};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

actions!(custom_panel, [ToggleFocus]);

const CUSTOM_PANEL_KEY: &str = "CustomPanel";

/// 自定义面板结构体，显示简单文本
pub struct CustomPanel {
    // 焦点句柄，用于管理面板的键盘焦点
    focus_handle: FocusHandle,
    // Workspace 的弱引用，避免循环引用（下划线前缀表示未使用）
    _workspace: WeakEntity<Workspace>,
    // 面板宽度，None 表示使用默认宽度
    width: Option<Pixels>,
    // 订阅列表，用于存储事件订阅（下划线前缀表示未使用）
    _subscriptions: Vec<gpui::Subscription>,
}

// CustomPanel 的实现块
impl CustomPanel {
    /// 创建新的自定义面板实例
    pub fn new(
        // 可变引用 workspace，用于获取工作区信息
        workspace: &mut Workspace,
        // 可变引用 window，用于创建窗口相关的组件（未使用）
        _window: &mut Window,
        // Context，用于创建实体和访问应用状态
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        // 从上下文中获取焦点句柄
        let focus_handle = cx.focus_handle();
        // 获取 workspace 的弱引用，避免循环引用
        let workspace_handle = workspace.weak_handle();

        // 创建新的 CustomPanel 实体
        cx.new(|_| CustomPanel {
            // 设置焦点句柄
            focus_handle,
            // 设置 workspace 的弱引用
            _workspace: workspace_handle,
            // 初始化宽度为 None（使用默认值）
            width: None,
            // 初始化订阅列表为空
            _subscriptions: Vec::new(),
        })
    }

    /// 异步加载自定义面板
    pub async fn load(
        // Workspace 的弱引用，用于在异步上下文中访问 workspace
        workspace: WeakEntity<Workspace>,
        // 异步窗口上下文，用于在异步操作中更新 UI
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        // 在异步上下文中更新 workspace，调用 new 方法创建面板
        workspace
            .update_in(&mut cx, |workspace, window, cx| {
                Self::new(workspace, window, cx)
            })
    }
}

/// 初始化自定义面板，注册切换焦点动作
pub fn init(cx: &mut App) {
    // 观察新创建的 workspace 实例
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        // 在 workspace 中注册 ToggleFocus 动作
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            // 切换 CustomPanel 的焦点
            workspace.toggle_panel_focus::<CustomPanel>(window, cx);
        });
    })
    // 分离任务，使其在后台运行
    .detach();
}

// 实现 Panel trait，使 CustomPanel 成为可用的面板
impl Panel for CustomPanel {
    // 返回面板的持久化名称，用于序列化和反序列化
    fn persistent_name() -> &'static str {
        "Custom Panel"
    }

    // 返回面板的唯一键
    fn panel_key() -> &'static str {
        CUSTOM_PANEL_KEY
    }

    // 返回面板在 dock 中的位置
    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Left
    }

    // 检查给定的位置是否有效
    fn position_is_valid(&self, position: DockPosition) -> bool {
        // 只允许左侧或右侧位置
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    // 设置面板位置（当前为空实现）
    fn set_position(&mut self, _position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {
        // 预留方法，可以在此更新设置或状态
    }

    // 返回面板的宽度
    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        // 如果有设置宽度则使用设置值，否则使用默认值 280 像素
        self.width.unwrap_or(px(280.))
    }

    // 设置面板宽度
    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, cx: &mut Context<Self>) {
        // 更新宽度
        self.width = size;
        // 通知视图需要重新渲染
        cx.notify();
    }

    // 返回面板的图标
    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Star)
    }

    // 返回图标的提示文本
    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some(i18n::t_static("panel.custom_panel"))
    }

    // 返回切换面板的动作
    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    // 返回面板的激活优先级，用于确定面板在状态栏中的顺序
    fn activation_priority(&self) -> u32 {
        10
    }
}

// 实现 Focusable trait，使面板可以接收键盘焦点
impl Focusable for CustomPanel {
    // 返回焦点句柄
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        // 克隆焦点句柄
        self.focus_handle.clone()
    }
}

// 实现 EventEmitter trait，使面板可以发出事件
impl EventEmitter<PanelEvent> for CustomPanel {}

// 实现 Render trait，定义面板的渲染逻辑
impl Render for CustomPanel {
    // 渲染面板内容
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 使用 i18n 翻译函数获取本地化文本
        let title = t("custom_panel.title");
        let hello_text = t("custom_panel.hello");

        // 创建一个占满整个空间的 div 容器
        div()
            // 设置容器大小为全屏
            .size_full()
            // 设置为 flex 布局
            .flex()
            // 设置为垂直布局（列方向）
            .flex_col()
            // 设置子元素间距为 2（默认单位）
            .gap_2()
            // 设置内边距为 2（默认单位）
            .p_2()
            // 设置背景色为面板背景色
            .bg(cx.theme().colors().panel_background)
            // 添加面板标题子元素
            .child(
                // 创建标题 div
                div()
                    // 设置文本颜色为静音色
                    .text_color(cx.theme().colors().text_muted)
                    // 设置标题文本内容（使用翻译）
                    .child(title),
            )
            // 添加内容文本子元素
            .child(
                // 创建内容 div
                div()
                    // 设置文本颜色为静音色
                    .text_color(cx.theme().colors().text_muted)
                    // 设置文本内容（使用翻译）
                    .child(hello_text),
            )
    }
}