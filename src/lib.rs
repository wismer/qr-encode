pub mod grid;

#[cfg(test)]
mod tests {
    use super::grid;

    #[test]
    fn test_qr_grid_size() {
        grid::main();
    }
}
