pub fn human_readable(seconds: f64) -> String {
    return if seconds < 2. * 60. {
        // Less than two minutes, use seconds.
        format!("{:.2} sec", seconds)
    } else if seconds < 2. * 60. * 60. {
        // Less than two hours, use minutes.
        format!("{:.2} min", seconds / 60.)
    } else if seconds < 2. * 60. * 60. * 24. {
        format!("{:.2} hr", seconds / (60. * 60.))
    } else {
        // Use days.
        format!("{:.2} hr", seconds / (60. * 60. * 24.))
    }
}
