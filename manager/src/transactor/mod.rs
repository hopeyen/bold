

// adjust interest rate for troves that are not within the acceptable range
pub fn adjust_trove_interest_rate(troves: &Vec<Trove>, difr: &f32) -> Result<(), anyhow::Error> {
    for trove in troves {
        if trove.interest_rate < difr {
            // adjust interest rate
        }
    }
}
