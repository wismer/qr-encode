/*

    type for qr options

  let qr_opt: QROptions = QROptions {
    version: 21,
    encoding: 13,
    format: 'i dont know'
  }

  from http://www.thonky.com/qr-code-tutorial/module-placement-matrix ->

    The size of a QR code can be calculated with the formula (((V-1)*4)+21), where V is the QR code version.
    For example, version 32 is (((32-1)*4)+21) or 145 modules by 145 modules.
    Therefore, the positions of the finder patterns can be generalized as follows:

        The top-left finder pattern's is always placed at (0,0).
        The top-right finder pattern's is always placed at ([(((V-1)*4)+21) - 7], 0)
        The bottom-left finder pattern's is always placed at (0,[(((V-1)*4)+21) - 7])
*/

pub struct QROptions {
    pub version: usize,
    pub encoding: u8, // for now - should be its own sub-type.
    pub requires_alignment: bool,
    pub finder_points: [(usize, usize); 3]
}

/*
    apply_finder_patterns();
    apply_separators();
    apply_alignment_patterns();

*/
