use std::path::Path;

use cursive::{
    view::SizeConstraint,
    views::{LinearLayout, Panel, ResizedView, TextView},
    CursiveRunnable,
};

use crate::evtx_view::EvtxView;

pub struct UIMain {
    siv: CursiveRunnable,
}

impl UIMain {
    pub fn new(evtx_file: &Path) -> anyhow::Result<Self> {
        let mut siv = cursive::default();
        siv.add_global_callback('q', |s| s.quit());

        let evtx_table = EvtxView::new(&mut siv, evtx_file)?;

        let details_view = TextView::new("");
        let root_view = LinearLayout::vertical()
            .child(evtx_table)
            .child(details_view);

        siv.add_layer(
            Panel::new(ResizedView::new(
                SizeConstraint::Full,
                SizeConstraint::Full,
                root_view,
            ))
            .title(format!(
                "{} v{}",
                env!("CARGO_BIN_NAME"),
                env!("CARGO_PKG_VERSION")
            )),
        );
        Ok(Self { siv })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.siv.run();
        Ok(())
    }
}
