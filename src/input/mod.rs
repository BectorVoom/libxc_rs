use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::model::Spin;

/// LDA input bundle: only electron density (rho).
///
/// Polarized data uses interleaved SoA layout:
/// `[rho_a_0, rho_b_0, rho_a_1, rho_b_1, ...]`.
/// The kernel indexes via `ip * dims.rho + component`.
/// The input bundle does not reorder data; it validates the caller
/// provided the correct total length.
#[derive(Debug)]
pub struct LdaInput<'a> {
    rho: &'a [f64],
    np: usize,
    spin: Spin,
}

/// GGA input bundle: electron density (rho) and density gradient (sigma).
///
/// Polarized data uses interleaved SoA layout (see [`LdaInput`]).
#[derive(Debug)]
pub struct GgaInput<'a> {
    rho: &'a [f64],
    sigma: &'a [f64],
    np: usize,
    spin: Spin,
}

/// MGGA input bundle: electron density (rho), density gradient (sigma),
/// laplacian of density (lapl), and kinetic energy density (tau).
///
/// Polarized data uses interleaved SoA layout (see [`LdaInput`]).
#[derive(Debug)]
pub struct MggaInput<'a> {
    rho: &'a [f64],
    sigma: &'a [f64],
    lapl: &'a [f64],
    tau: &'a [f64],
    np: usize,
    spin: Spin,
}

/// Validate a single input buffer against its expected size.
fn validate_input_buffer(
    buf: &[f64],
    field: &'static str,
    np: usize,
    dim: usize,
) -> Result<(), LibxcRsError> {
    let expected = np * dim;
    if buf.len() != expected {
        return Err(LibxcRsError::InputBufferSizeMismatch {
            field,
            expected,
            actual: buf.len(),
        });
    }
    Ok(())
}

impl<'a> LdaInput<'a> {
    /// Create a new LDA input bundle, validating buffer sizes.
    ///
    /// # Errors
    /// Returns [`LibxcRsError::InputBufferSizeMismatch`] if `rho.len() != np * dims.rho`.
    pub fn new(rho: &'a [f64], np: usize, spin: Spin) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::lda(spin);
        validate_input_buffer(rho, "rho", np, dims.rho as usize)?;
        Ok(Self { rho, np, spin })
    }

    /// Number of grid points.
    pub fn np(&self) -> usize {
        self.np
    }

    /// Spin polarization mode.
    pub fn spin(&self) -> Spin {
        self.spin
    }

    /// Electron density buffer.
    pub fn rho(&self) -> &[f64] {
        self.rho
    }
}

impl<'a> GgaInput<'a> {
    /// Create a new GGA input bundle, validating buffer sizes.
    ///
    /// # Errors
    /// Returns [`LibxcRsError::InputBufferSizeMismatch`] if any buffer has wrong length.
    pub fn new(
        rho: &'a [f64],
        sigma: &'a [f64],
        np: usize,
        spin: Spin,
    ) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::gga(spin);
        validate_input_buffer(rho, "rho", np, dims.rho as usize)?;
        validate_input_buffer(sigma, "sigma", np, dims.sigma as usize)?;
        Ok(Self { rho, sigma, np, spin })
    }

    /// Number of grid points.
    pub fn np(&self) -> usize {
        self.np
    }

    /// Spin polarization mode.
    pub fn spin(&self) -> Spin {
        self.spin
    }

    /// Electron density buffer.
    pub fn rho(&self) -> &[f64] {
        self.rho
    }

    /// Density gradient buffer.
    pub fn sigma(&self) -> &[f64] {
        self.sigma
    }
}

impl<'a> MggaInput<'a> {
    /// Create a new MGGA input bundle, validating buffer sizes.
    ///
    /// # Errors
    /// Returns [`LibxcRsError::InputBufferSizeMismatch`] if any buffer has wrong length.
    pub fn new(
        rho: &'a [f64],
        sigma: &'a [f64],
        lapl: &'a [f64],
        tau: &'a [f64],
        np: usize,
        spin: Spin,
    ) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::mgga(spin);
        validate_input_buffer(rho, "rho", np, dims.rho as usize)?;
        validate_input_buffer(sigma, "sigma", np, dims.sigma as usize)?;
        validate_input_buffer(lapl, "lapl", np, dims.lapl as usize)?;
        validate_input_buffer(tau, "tau", np, dims.tau as usize)?;
        Ok(Self { rho, sigma, lapl, tau, np, spin })
    }

    /// Number of grid points.
    pub fn np(&self) -> usize {
        self.np
    }

    /// Spin polarization mode.
    pub fn spin(&self) -> Spin {
        self.spin
    }

    /// Electron density buffer.
    pub fn rho(&self) -> &[f64] {
        self.rho
    }

    /// Density gradient buffer.
    pub fn sigma(&self) -> &[f64] {
        self.sigma
    }

    /// Laplacian of density buffer.
    pub fn lapl(&self) -> &[f64] {
        self.lapl
    }

    /// Kinetic energy density buffer.
    pub fn tau(&self) -> &[f64] {
        self.tau
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── LdaInput tests ─────────────────────────────────────────────

    #[test]
    fn lda_unpolarized_ok() {
        let rho = vec![0.1; 10];
        let input = LdaInput::new(&rho, 10, Spin::Unpolarized).unwrap();
        assert_eq!(input.np(), 10);
        assert_eq!(input.spin(), Spin::Unpolarized);
        assert_eq!(input.rho().len(), 10);
    }

    #[test]
    fn lda_polarized_ok() {
        let rho = vec![0.1; 20];
        let input = LdaInput::new(&rho, 10, Spin::Polarized).unwrap();
        assert_eq!(input.np(), 10);
        assert_eq!(input.spin(), Spin::Polarized);
        assert_eq!(input.rho().len(), 20);
    }

    #[test]
    fn lda_unpolarized_wrong_size() {
        let rho = vec![0.1; 5];
        let err = LdaInput::new(&rho, 10, Spin::Unpolarized).unwrap_err();
        match err {
            LibxcRsError::InputBufferSizeMismatch { field, expected, actual } => {
                assert_eq!(field, "rho");
                assert_eq!(expected, 10);
                assert_eq!(actual, 5);
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn lda_polarized_wrong_size() {
        let rho = vec![0.1; 10]; // should be 20 for polarized
        let err = LdaInput::new(&rho, 10, Spin::Polarized).unwrap_err();
        match err {
            LibxcRsError::InputBufferSizeMismatch { field, expected, actual } => {
                assert_eq!(field, "rho");
                assert_eq!(expected, 20);
                assert_eq!(actual, 10);
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    // ── GgaInput tests ─────────────────────────────────────────────

    #[test]
    fn gga_unpolarized_ok() {
        let rho = vec![0.1; 10];
        let sigma = vec![0.01; 10];
        let input = GgaInput::new(&rho, &sigma, 10, Spin::Unpolarized).unwrap();
        assert_eq!(input.np(), 10);
        assert_eq!(input.rho().len(), 10);
        assert_eq!(input.sigma().len(), 10);
    }

    #[test]
    fn gga_polarized_ok() {
        let rho = vec![0.1; 20];   // np*2
        let sigma = vec![0.01; 30]; // np*3
        let input = GgaInput::new(&rho, &sigma, 10, Spin::Polarized).unwrap();
        assert_eq!(input.np(), 10);
        assert_eq!(input.spin(), Spin::Polarized);
    }

    #[test]
    fn gga_bad_rho() {
        let rho = vec![0.1; 5];
        let sigma = vec![0.01; 10];
        let err = GgaInput::new(&rho, &sigma, 10, Spin::Unpolarized).unwrap_err();
        match err {
            LibxcRsError::InputBufferSizeMismatch { field, .. } => {
                assert_eq!(field, "rho");
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn gga_bad_sigma() {
        let rho = vec![0.1; 10];
        let sigma = vec![0.01; 5];
        let err = GgaInput::new(&rho, &sigma, 10, Spin::Unpolarized).unwrap_err();
        match err {
            LibxcRsError::InputBufferSizeMismatch { field, expected, actual } => {
                assert_eq!(field, "sigma");
                assert_eq!(expected, 10);
                assert_eq!(actual, 5);
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    // ── MggaInput tests ────────────────────────────────────────────

    #[test]
    fn mgga_unpolarized_ok() {
        let np = 10;
        let rho = vec![0.1; np];
        let sigma = vec![0.01; np];
        let lapl = vec![0.001; np];
        let tau = vec![0.002; np];
        let input = MggaInput::new(&rho, &sigma, &lapl, &tau, np, Spin::Unpolarized).unwrap();
        assert_eq!(input.np(), np);
        assert_eq!(input.rho().len(), np);
        assert_eq!(input.sigma().len(), np);
        assert_eq!(input.lapl().len(), np);
        assert_eq!(input.tau().len(), np);
    }

    #[test]
    fn mgga_polarized_ok() {
        let np = 10;
        let rho = vec![0.1; np * 2];
        let sigma = vec![0.01; np * 3];
        let lapl = vec![0.001; np * 2];
        let tau = vec![0.002; np * 2];
        let input = MggaInput::new(&rho, &sigma, &lapl, &tau, np, Spin::Polarized).unwrap();
        assert_eq!(input.np(), np);
        assert_eq!(input.spin(), Spin::Polarized);
    }

    #[test]
    fn mgga_bad_lapl() {
        let np = 10;
        let rho = vec![0.1; np * 2];
        let sigma = vec![0.01; np * 3];
        let lapl = vec![0.001; 5]; // wrong
        let tau = vec![0.002; np * 2];
        let err = MggaInput::new(&rho, &sigma, &lapl, &tau, np, Spin::Polarized).unwrap_err();
        match err {
            LibxcRsError::InputBufferSizeMismatch { field, expected, actual } => {
                assert_eq!(field, "lapl");
                assert_eq!(expected, 20);
                assert_eq!(actual, 5);
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn mgga_bad_tau() {
        let np = 10;
        let rho = vec![0.1; np * 2];
        let sigma = vec![0.01; np * 3];
        let lapl = vec![0.001; np * 2];
        let tau = vec![0.002; 3]; // wrong
        let err = MggaInput::new(&rho, &sigma, &lapl, &tau, np, Spin::Polarized).unwrap_err();
        match err {
            LibxcRsError::InputBufferSizeMismatch { field, .. } => {
                assert_eq!(field, "tau");
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    // ── Edge case tests ────────────────────────────────────────────

    #[test]
    fn lda_zero_np() {
        let rho: Vec<f64> = vec![];
        let input = LdaInput::new(&rho, 0, Spin::Unpolarized).unwrap();
        assert_eq!(input.np(), 0);
        assert_eq!(input.rho().len(), 0);
    }
}
