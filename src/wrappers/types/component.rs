use twilight_model::{
    channel::message::component::{
        ActionRow as TwilightActionRow, Button as TwilightButton, Component as TwilightComponent,
        Container as TwilightContainer, SelectDefaultValue as TwilightSelectDefaultValue,
        SelectMenu as TwilightSelectMenu, SelectMenuOption as TwilightSelectMenuOption,
        SelectMenuType as TwilightSelectMenuType, Separator as TwilightSeparator,
        TextDisplay as TwilightTextDisplay,
    },
    id::{
        Id,
        marker::{RoleMarker, UserMarker},
    },
};

pub use twilight_model::channel::message::component::{ButtonStyle, SeparatorSpacingSize};

pub trait IntoTwilightMessageComponent {
    fn into_component(self) -> TwilightComponent;
}

pub trait IntoTwilightButtonComponent {
    fn into_button_component(self) -> TwilightComponent;
}

pub trait IntoTwilightSelectComponent {
    fn into_select_component(self) -> TwilightComponent;
}

#[derive(Default, Clone)]
pub struct ActionRow {
    components: Vec<TwilightComponent>,
}

impl ActionRow {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn button(mut self, component: impl IntoTwilightButtonComponent) -> Self {
        assert!(
            self.components.len() < 5,
            "An action row can only have up to 5 button components"
        );
        assert!(
            self.components
                .first()
                .is_none_or(|c| matches!(c, TwilightComponent::Button(_))),
            "An action row can only contain one select component, or up to five button components"
        );

        self.components.push(component.into_button_component());
        self
    }

    pub fn select(mut self, component: impl IntoTwilightSelectComponent) -> Self {
        assert!(
            self.components.is_empty(),
            "An action row can only contain one select component, or up to five button components"
        );

        self.components.push(component.into_select_component());
        self
    }
}

impl IntoTwilightMessageComponent for ActionRow {
    fn into_component(self) -> TwilightComponent {
        TwilightComponent::ActionRow(TwilightActionRow {
            id: None,
            components: self.components,
        })
    }
}

impl From<ActionRow> for TwilightComponent {
    fn from(action_row: ActionRow) -> Self {
        action_row.into_component()
    }
}

#[derive(Clone)]
pub struct PrimaryButton {
    label: String,
    custom_id: String,
    disabled: bool,
}

impl PrimaryButton {
    pub fn new(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            custom_id: custom_id.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl IntoTwilightButtonComponent for PrimaryButton {
    fn into_button_component(self) -> TwilightComponent {
        TwilightComponent::Button(TwilightButton {
            id: None,
            custom_id: Some(self.custom_id),
            disabled: self.disabled,
            emoji: None,
            label: Some(self.label),
            style: ButtonStyle::Primary,
            url: None,
            sku_id: None,
        })
    }
}

impl From<PrimaryButton> for TwilightComponent {
    fn from(button: PrimaryButton) -> Self {
        button.into_button_component()
    }
}

#[derive(Clone)]
pub struct SecondaryButton {
    label: String,
    custom_id: String,
    disabled: bool,
}

impl SecondaryButton {
    pub fn new(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            custom_id: custom_id.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl IntoTwilightButtonComponent for SecondaryButton {
    fn into_button_component(self) -> TwilightComponent {
        TwilightComponent::Button(TwilightButton {
            id: None,
            custom_id: Some(self.custom_id),
            disabled: self.disabled,
            emoji: None,
            label: Some(self.label),
            style: ButtonStyle::Secondary,
            url: None,
            sku_id: None,
        })
    }
}

impl From<SecondaryButton> for TwilightComponent {
    fn from(button: SecondaryButton) -> Self {
        button.into_button_component()
    }
}

#[derive(Clone)]
pub struct SuccessButton {
    label: String,
    custom_id: String,
    disabled: bool,
}

impl SuccessButton {
    pub fn new(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            custom_id: custom_id.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl IntoTwilightButtonComponent for SuccessButton {
    fn into_button_component(self) -> TwilightComponent {
        TwilightComponent::Button(TwilightButton {
            id: None,
            custom_id: Some(self.custom_id),
            disabled: self.disabled,
            emoji: None,
            label: Some(self.label),
            style: ButtonStyle::Success,
            url: None,
            sku_id: None,
        })
    }
}

impl From<SuccessButton> for TwilightComponent {
    fn from(button: SuccessButton) -> Self {
        button.into_button_component()
    }
}

#[derive(Clone)]
pub struct DangerButton {
    label: String,
    custom_id: String,
    disabled: bool,
}

impl DangerButton {
    pub fn new(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            custom_id: custom_id.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl IntoTwilightButtonComponent for DangerButton {
    fn into_button_component(self) -> TwilightComponent {
        TwilightComponent::Button(TwilightButton {
            id: None,
            custom_id: Some(self.custom_id),
            disabled: self.disabled,
            emoji: None,
            label: Some(self.label),
            style: ButtonStyle::Danger,
            url: None,
            sku_id: None,
        })
    }
}

impl From<DangerButton> for TwilightComponent {
    fn from(button: DangerButton) -> Self {
        button.into_button_component()
    }
}

pub trait IntoTwilightSelectOption {
    fn into_select_option(self) -> TwilightSelectMenuOption;
}

#[derive(Clone)]

pub struct TextSelect {
    custom_id: String,
    disabled: bool,
    max_values: Option<u8>,
    min_values: Option<u8>,
    options: Vec<TwilightSelectMenuOption>,
    placeholder: Option<String>,
    required: bool,
}

impl TextSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            disabled: false,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: None,
            required: false,
        }
    }

    pub fn option(mut self, option: impl IntoTwilightSelectOption) -> Self {
        self.options.push(option.into_select_option());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl IntoTwilightSelectComponent for TextSelect {
    fn into_select_component(self) -> TwilightComponent {
        TwilightComponent::SelectMenu(TwilightSelectMenu {
            id: None,
            channel_types: None,
            custom_id: self.custom_id,
            default_values: None,
            disabled: self.disabled,
            kind: TwilightSelectMenuType::Text,
            max_values: self.max_values,
            min_values: self.min_values,
            options: Some(self.options),
            placeholder: self.placeholder,
            required: Some(self.required),
        })
    }
}

impl From<TextSelect> for TwilightComponent {
    fn from(select: TextSelect) -> Self {
        select.into_select_component()
    }
}

#[derive(Clone)]
pub struct TextSelectOption {
    label: String,
    value: String,
    description: Option<String>,
    default: bool,
}

impl TextSelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            default: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn default(mut self) -> Self {
        self.default = true;
        self
    }
}

impl IntoTwilightSelectOption for TextSelectOption {
    fn into_select_option(self) -> TwilightSelectMenuOption {
        TwilightSelectMenuOption {
            default: self.default,
            description: self.description,
            emoji: None,
            label: self.label,
            value: self.value,
        }
    }
}

impl IntoTwilightSelectOption for String {
    fn into_select_option(self) -> TwilightSelectMenuOption {
        TwilightSelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: self.clone(),
            value: self,
        }
    }
}

impl IntoTwilightSelectOption for &'_ str {
    fn into_select_option(self) -> TwilightSelectMenuOption {
        TwilightSelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: self.to_string(),
            value: self.to_string(),
        }
    }
}

impl<Label, Value> IntoTwilightSelectOption for (Label, Value)
where
    Label: Into<String>,
    Value: Into<String>,
{
    fn into_select_option(self) -> TwilightSelectMenuOption {
        TwilightSelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: self.0.into(),
            value: self.1.into(),
        }
    }
}

impl From<TextSelectOption> for TwilightSelectMenuOption {
    fn from(option: TextSelectOption) -> Self {
        option.into_select_option()
    }
}

#[derive(Clone)]
pub struct UserSelect {
    custom_id: String,
    default_values: Vec<TwilightSelectDefaultValue>,
    disabled: bool,
    max_values: Option<u8>,
    min_values: Option<u8>,
    placeholder: Option<String>,
    required: bool,
}

impl UserSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            default_values: Vec::new(),
            disabled: false,
            max_values: None,
            min_values: None,
            placeholder: None,
            required: false,
        }
    }

    pub fn default_user(mut self, default: Id<UserMarker>) -> Self {
        self.default_values
            .push(TwilightSelectDefaultValue::User(default));
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl IntoTwilightSelectComponent for UserSelect {
    fn into_select_component(self) -> TwilightComponent {
        TwilightComponent::SelectMenu(TwilightSelectMenu {
            id: None,
            channel_types: None,
            custom_id: self.custom_id,
            default_values: Some(self.default_values),
            disabled: self.disabled,
            kind: TwilightSelectMenuType::User,
            max_values: self.max_values,
            min_values: self.min_values,
            options: None,
            placeholder: self.placeholder,
            required: Some(self.required),
        })
    }
}

impl From<UserSelect> for TwilightComponent {
    fn from(select: UserSelect) -> Self {
        select.into_select_component()
    }
}

#[derive(Clone)]
pub struct RoleSelect {
    custom_id: String,
    default_values: Vec<TwilightSelectDefaultValue>,
    disabled: bool,
    max_values: Option<u8>,
    min_values: Option<u8>,
    placeholder: Option<String>,
    required: bool,
}

impl RoleSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            default_values: Vec::new(),
            disabled: false,
            max_values: None,
            min_values: None,
            placeholder: None,
            required: false,
        }
    }

    pub fn default_role(mut self, default: Id<RoleMarker>) -> Self {
        self.default_values
            .push(TwilightSelectDefaultValue::Role(default));
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl IntoTwilightSelectComponent for RoleSelect {
    fn into_select_component(self) -> TwilightComponent {
        TwilightComponent::SelectMenu(TwilightSelectMenu {
            id: None,
            channel_types: None,
            custom_id: self.custom_id,
            default_values: Some(self.default_values),
            disabled: self.disabled,
            kind: TwilightSelectMenuType::Role,
            max_values: self.max_values,
            min_values: self.min_values,
            options: None,
            placeholder: self.placeholder,
            required: Some(self.required),
        })
    }
}

impl From<RoleSelect> for TwilightComponent {
    fn from(select: RoleSelect) -> Self {
        select.into_select_component()
    }
}

#[derive(Clone)]
pub struct MentionableSelect {
    custom_id: String,
    default_values: Vec<TwilightSelectDefaultValue>,
    disabled: bool,
    max_values: Option<u8>,
    min_values: Option<u8>,
    placeholder: Option<String>,
    required: bool,
}

impl MentionableSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            default_values: Vec::new(),
            disabled: false,
            max_values: None,
            min_values: None,
            placeholder: None,
            required: false,
        }
    }

    pub fn default_user(mut self, default: Id<UserMarker>) -> Self {
        self.default_values
            .push(TwilightSelectDefaultValue::User(default));
        self
    }

    pub fn default_role(mut self, default: Id<RoleMarker>) -> Self {
        self.default_values
            .push(TwilightSelectDefaultValue::Role(default));
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl IntoTwilightSelectComponent for MentionableSelect {
    fn into_select_component(self) -> TwilightComponent {
        TwilightComponent::SelectMenu(TwilightSelectMenu {
            id: None,
            channel_types: None,
            custom_id: self.custom_id,
            default_values: Some(self.default_values),
            disabled: self.disabled,
            kind: TwilightSelectMenuType::Mentionable,
            max_values: self.max_values,
            min_values: self.min_values,
            options: None,
            placeholder: self.placeholder,
            required: Some(self.required),
        })
    }
}

impl From<MentionableSelect> for TwilightComponent {
    fn from(select: MentionableSelect) -> Self {
        select.into_select_component()
    }
}

#[derive(Default, Clone)]
pub struct TextDisplay {
    content: String,
}

impl TextDisplay {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn push(mut self, content: impl Into<String>) -> Self {
        self.content.push_str(&content.into());
        self
    }
}

impl IntoTwilightMessageComponent for TextDisplay {
    fn into_component(self) -> TwilightComponent {
        TwilightComponent::TextDisplay(TwilightTextDisplay {
            id: None,
            content: self.content,
        })
    }
}

impl From<TextDisplay> for TwilightComponent {
    fn from(text_display: TextDisplay) -> Self {
        text_display.into_component()
    }
}

#[derive(Clone)]
pub struct Separator {
    divider: bool,
    spacing: SeparatorSpacingSize,
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            divider: true,
            spacing: SeparatorSpacingSize::Small,
        }
    }
}

impl Separator {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = divider;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<SeparatorSpacingSize>) -> Self {
        self.spacing = spacing.into();
        self
    }
}

impl IntoTwilightMessageComponent for Separator {
    fn into_component(self) -> TwilightComponent {
        TwilightComponent::Separator(TwilightSeparator {
            id: None,
            divider: Some(self.divider),
            spacing: Some(self.spacing),
        })
    }
}

impl From<Separator> for TwilightComponent {
    fn from(separator: Separator) -> Self {
        separator.into_component()
    }
}

#[derive(Default, Clone)]
pub struct Container {
    accent_color: Option<u32>,
    spoiler: bool,
    components: Vec<TwilightComponent>,
}

impl Container {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn accent_color(mut self, accent_color: u32) -> Self {
        self.accent_color = Some(accent_color);
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }

    pub fn component(mut self, component: impl IntoTwilightMessageComponent) -> Self {
        self.components.push(component.into_component());
        self
    }
}

impl IntoTwilightMessageComponent for Container {
    fn into_component(self) -> TwilightComponent {
        TwilightComponent::Container(TwilightContainer {
            id: None,
            accent_color: Some(self.accent_color),
            spoiler: Some(self.spoiler),
            components: self.components,
        })
    }
}

impl From<Container> for TwilightComponent {
    fn from(container: Container) -> Self {
        container.into_component()
    }
}
