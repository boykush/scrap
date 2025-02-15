use once_cell::sync::Lazy;
use scraps_libs::error::{anyhow::Context, ScrapError, ScrapResult};
use tera::Tera;

static MARKDOWN_TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![(
        "daily_note.md",
        include_str!("builtins/daily_note.md"),
    )])
    .unwrap();
    tera
});

pub fn init(template_dir: &str) -> ScrapResult<(Tera, tera::Context)> {
    let mut tera = Tera::new(template_dir).context(ScrapError::PublicRender)?;
    tera.extend(&MARKDOWN_TERA).unwrap();
    let context = tera::Context::new();

    Ok((tera, context))
}
