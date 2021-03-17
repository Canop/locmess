use {
    crate::*,
    minimad::OwningTemplateExpander,
    std::{
        ops::AddAssign,
    },
    termimad::*,
};

/// number of bars max in histograms
pub const N: usize = 32;

static MD: &str = r#"
|:-:|:-:|:-:|:-
|**line len**|**count**|**%**|**${scale}**
|:-:|-:|-:|:-
${bars
|${line_len}|${count}|${percent}|*${bar}*
}
|-:
"#;

#[derive(Clone, Default)]
pub struct Histogram {
    pub bars: [usize; N],
}

impl Histogram {
    pub fn bar_min_max(idx: usize) -> String {
        if idx == 0 {
            "0".to_string()
        } else {
            format!(
                "{} to {}",
                (idx-1)*10+1,
                idx*10,
            )
        }
    }

    pub fn print(
        &self,
        printer: &Printer,
    ) {
        let mut expander = OwningTemplateExpander::new();
        let mut max_count = 0;
        let mut max_idx = 0;
        let mut sum = 0;
        for (idx, &count) in self.bars.iter().enumerate() {
            if count > 0 {
                max_idx = idx;
                if count > max_count {
                    max_count = count;
                }
                sum += count;
            }
        }
        if max_count == 0 {
            eprintln!("nothing to display");
        }
        expander.set(
            "scale",
            format!("0               {:>4}", max_count),
        );
        for idx in 0..=max_idx {
            let bar = self.bars[idx];
            let part = (bar as f32) / (max_count as f32);
            expander.sub("bars")
                .set("line_len", Self::bar_min_max(idx))
                .set_md("count", bar.to_string())
                .set_md("percent", to_percent(bar, sum))
                .set("bar", ProgressBar::new(part, 20));
        }
        printer.print(expander, MD);
    }

}

impl AddAssign for Histogram {
    //#[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, other: Self) {
        for idx in 0..N {
            self.bars[idx] += other.bars[idx];
        }
    }
}
fn to_percent(count: usize, total: usize) -> String {
    if count == 0 {
        " ".to_string()
    } else {
        let percent = 100f32 * (count as f32) / (total as f32);
        format!("{:.0}%", percent)
    }
}
