use std::{ fs::File, io::{ Write, BufWriter } };

fn main() -> anyhow::Result<()> {
    let context = rune::default_context()?;

    let mut paths = Vec::new();
    for (_hash, sig) in context.iter_functions() {
        let mut cursor: &mut Vec<ApiStructure> = &mut paths;
        for segment in sig.path() {
            let segment = segment.to_string();
            if let Some(idx) = cursor
                .iter()
                .enumerate()
                .find(|(_idx, x)| x.name == segment)
                .map(|(idx, _)| idx)
            {
                cursor = &mut cursor[idx].children;
            } else {
                cursor.push(ApiStructure {
                    name: segment,
                    children: Vec::new(),
                });
            }
        }
    }

    let mut output = BufWriter::new(File::create("docs.adoc")?);
    writeln!(&mut output, "= Rune API docs")?;
    writeln!(&mut output)?;
    for item in &paths {
        item.to_asciidoc(&mut output, 1)?;
    }

    Ok(())
}

struct ApiStructure {
    name: String,
    children: Vec<ApiStructure>,
}

impl ApiStructure {
    fn to_asciidoc(&self, w: &mut dyn Write, level: usize) -> anyhow::Result<()> {
        writeln!(w, "{} {}", "*".repeat(level), self.name)?;

        for child in &self.children {
            child.to_asciidoc(w, level + 1)?;
        }

        Ok(())
    }
}
