use primitives_core::section::SectionType;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Properties, Clone, PartialEq)]
pub struct SectionProps {
    /// The layout type of the section.
    #[prop_or_default]
    pub section_type: SectionType,

    /// Additional Tailwind classes merged onto the root element.
    #[prop_or_default]
    pub class: String,

    /// Section content (headings, cards, grids, etc.).
    pub children: Children,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// A section component that renders a `<section>` element with a layout type.
///
/// Uses `primitives_core::section::SectionType` for shared class and type
/// mapping logic.
///
/// ## Types
///
/// `Full` (default) · `Grid`
///
/// ## Example
///
/// ```rust
/// html! {
///     <Section section_type={SectionType::Full}>
///         <h1>{"Full-width hero section"}</h1>
///     </Section>
///
///     <Section section_type={SectionType::Grid}>
///         <div>{"Card 1"}</div>
///         <div>{"Card 2"}</div>
///         <div>{"Card 3"}</div>
///     </Section>
/// }
/// ```
#[component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let classes = format!("{} {}", props.section_type.classes(), props.class);

    let data_type = props.section_type.as_str();

    html! {
        <section
            class={classes}
            data-slot="section"
            data-type={data_type}
        >
            { for props.children.iter() }
        </section>
    }
}
