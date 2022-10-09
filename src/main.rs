use polars::prelude::*;
fn main() -> PolarsResult<()> {
    let df = LazyCsvReader::new("./dataset/foods1.csv")
        .finish()?
        .select([
            // select all columns
            all(),
            // and do some aggregations
            cols(["fats_g", "sugars_g"]).sum().suffix("_summed"),
        ])
        .collect()?;

    dbg!(&df);

    Ok(())
}
