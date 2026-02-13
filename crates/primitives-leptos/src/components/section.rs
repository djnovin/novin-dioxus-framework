use leptos::prelude::*;
use primitives_core::section::SectionType;

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
/// view! {
///     <Section section_type=SectionType::Full>
///         <h1>"Full-width hero section"</h1>
///     </Section>
///
///     <Section section_type=SectionType::Grid>
///         <div>"Card 1"</div>
///         <div>"Card 2"</div>
///         <div>"Card 3"</div>
///     </Section>
/// }
/// ```
#[component]
pub fn Section(
    /// The layout type of the section.
    #[prop(optional, into)]
    section_type: SectionType,
    /// Additional Tailwind classes merged onto the root element.
    #[prop(optional, into)]
    class: String,
    /// Section content (headings, cards, grids, etc.).
    children: Children,
) -> impl IntoView {
    let classes = format!("{} {}", section_type.classes(), class);

    let data_type = section_type.as_str();

    view! {
        <section
            class={classes}
            data-slot="section"
            data-type={data_type}
        >
            {children()}
        </section>
    }
}
