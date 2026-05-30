pub(crate) fn frequency_scale_per_mille(
    frequency: u64,
    min_frequency: u64,
    max_frequency: u64,
) -> u64 {
    if min_frequency >= max_frequency {
        return 1000;
    }

    let min = (min_frequency.max(1) as f64).ln();
    let max = (max_frequency.max(1) as f64).ln();
    if min >= max {
        return 1000;
    }

    let current = (frequency.max(1) as f64).ln();
    let highness = ((current - min) / (max - min)).clamp(0.0, 1.0);
    1000 + ((1.0 - highness) * 600.0).round() as u64
}
