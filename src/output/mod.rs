<<<<<<< HEAD
pub mod mask;

pub use mask::OutputMask;

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::model::Spin;

/// Validate an optional output buffer against its expected size.
fn validate_output_field(
    field_opt: &Option<&mut [f64]>,
    field_name: &'static str,
    np: usize,
    dim: usize,
) -> Result<(), LibxcRsError> {
    if let Some(buf) = field_opt {
        let expected = np * dim;
        if buf.len() != expected {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: field_name,
                expected,
                actual: buf.len(),
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// LdaOutput
// ---------------------------------------------------------------------------

/// LDA output bundle: derivative buffers through 4th order for rho-only functionals.
///
/// Each field is `Option<&mut [f64]>` -- `None` means "do not compute this order".
/// Buffers are validated at construction time against [`Dimensions::lda`].
#[derive(Debug, Default)]
pub struct LdaOutput<'a> {
    // Order 0
    pub zk: Option<&'a mut [f64]>,
    // Order 1
    pub vrho: Option<&'a mut [f64]>,
    // Order 2
    pub v2rho2: Option<&'a mut [f64]>,
    // Order 3
    pub v3rho3: Option<&'a mut [f64]>,
    // Order 4
    pub v4rho4: Option<&'a mut [f64]>,
}

impl<'a> LdaOutput<'a> {
    /// Create and validate an LDA output bundle.
    ///
    /// Each `Some` buffer must have length `np * dims.<field>`.
    /// `None` fields are always valid (nothing to compute).
    ///
    /// # Errors
    /// Returns [`LibxcRsError::OutputBufferSizeMismatch`] on wrong buffer length.
    pub fn new(
        zk: Option<&'a mut [f64]>,
        vrho: Option<&'a mut [f64]>,
        v2rho2: Option<&'a mut [f64]>,
        v3rho3: Option<&'a mut [f64]>,
        v4rho4: Option<&'a mut [f64]>,
        np: usize,
        spin: Spin,
    ) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::lda(spin);
        validate_output_field(&zk, "zk", np, dims.zk as usize)?;
        validate_output_field(&vrho, "vrho", np, dims.vrho as usize)?;
        validate_output_field(&v2rho2, "v2rho2", np, dims.v2rho2 as usize)?;
        validate_output_field(&v3rho3, "v3rho3", np, dims.v3rho3 as usize)?;
        validate_output_field(&v4rho4, "v4rho4", np, dims.v4rho4 as usize)?;
        Ok(Self { zk, vrho, v2rho2, v3rho3, v4rho4 })
    }
}

// ---------------------------------------------------------------------------
// GgaOutput
// ---------------------------------------------------------------------------

/// GGA output bundle: 15 derivative fields (rho + sigma cross-derivatives).
///
/// See [`LdaOutput`] for the `Option` semantics.
#[derive(Debug, Default)]
pub struct GgaOutput<'a> {
    // Order 0
    pub zk: Option<&'a mut [f64]>,
    // Order 1
    pub vrho: Option<&'a mut [f64]>,
    pub vsigma: Option<&'a mut [f64]>,
    // Order 2
    pub v2rho2: Option<&'a mut [f64]>,
    pub v2rhosigma: Option<&'a mut [f64]>,
    pub v2sigma2: Option<&'a mut [f64]>,
    // Order 3
    pub v3rho3: Option<&'a mut [f64]>,
    pub v3rho2sigma: Option<&'a mut [f64]>,
    pub v3rhosigma2: Option<&'a mut [f64]>,
    pub v3sigma3: Option<&'a mut [f64]>,
    // Order 4
    pub v4rho4: Option<&'a mut [f64]>,
    pub v4rho3sigma: Option<&'a mut [f64]>,
    pub v4rho2sigma2: Option<&'a mut [f64]>,
    pub v4rhosigma3: Option<&'a mut [f64]>,
    pub v4sigma4: Option<&'a mut [f64]>,
}

impl<'a> GgaOutput<'a> {
    /// Create and validate a GGA output bundle.
    ///
    /// # Errors
    /// Returns [`LibxcRsError::OutputBufferSizeMismatch`] on wrong buffer length.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        zk: Option<&'a mut [f64]>,
        vrho: Option<&'a mut [f64]>,
        vsigma: Option<&'a mut [f64]>,
        v2rho2: Option<&'a mut [f64]>,
        v2rhosigma: Option<&'a mut [f64]>,
        v2sigma2: Option<&'a mut [f64]>,
        v3rho3: Option<&'a mut [f64]>,
        v3rho2sigma: Option<&'a mut [f64]>,
        v3rhosigma2: Option<&'a mut [f64]>,
        v3sigma3: Option<&'a mut [f64]>,
        v4rho4: Option<&'a mut [f64]>,
        v4rho3sigma: Option<&'a mut [f64]>,
        v4rho2sigma2: Option<&'a mut [f64]>,
        v4rhosigma3: Option<&'a mut [f64]>,
        v4sigma4: Option<&'a mut [f64]>,
        np: usize,
        spin: Spin,
    ) -> Result<Self, LibxcRsError> {
        let dims = Dimensions::gga(spin);
        validate_output_field(&zk, "zk", np, dims.zk as usize)?;
        validate_output_field(&vrho, "vrho", np, dims.vrho as usize)?;
        validate_output_field(&vsigma, "vsigma", np, dims.vsigma as usize)?;
        validate_output_field(&v2rho2, "v2rho2", np, dims.v2rho2 as usize)?;
        validate_output_field(&v2rhosigma, "v2rhosigma", np, dims.v2rhosigma as usize)?;
        validate_output_field(&v2sigma2, "v2sigma2", np, dims.v2sigma2 as usize)?;
        validate_output_field(&v3rho3, "v3rho3", np, dims.v3rho3 as usize)?;
        validate_output_field(&v3rho2sigma, "v3rho2sigma", np, dims.v3rho2sigma as usize)?;
        validate_output_field(&v3rhosigma2, "v3rhosigma2", np, dims.v3rhosigma2 as usize)?;
        validate_output_field(&v3sigma3, "v3sigma3", np, dims.v3sigma3 as usize)?;
        validate_output_field(&v4rho4, "v4rho4", np, dims.v4rho4 as usize)?;
        validate_output_field(&v4rho3sigma, "v4rho3sigma", np, dims.v4rho3sigma as usize)?;
        validate_output_field(&v4rho2sigma2, "v4rho2sigma2", np, dims.v4rho2sigma2 as usize)?;
        validate_output_field(&v4rhosigma3, "v4rhosigma3", np, dims.v4rhosigma3 as usize)?;
        validate_output_field(&v4sigma4, "v4sigma4", np, dims.v4sigma4 as usize)?;
        Ok(Self {
            zk, vrho, vsigma,
            v2rho2, v2rhosigma, v2sigma2,
            v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3,
            v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4,
        })
    }
}

// ---------------------------------------------------------------------------
// MggaOutput
// ---------------------------------------------------------------------------

/// MGGA output bundle: all 70 derivative fields through 4th order.
///
/// Fields are `pub` to allow struct-literal construction with `..Default::default()`
/// for the common case where only a few derivative orders are needed.
/// Call [`MggaOutput::validate`] after construction to check buffer sizes.
///
/// Field counts by order: 1 (order 0) + 4 (order 1) + 10 (order 2) + 20 (order 3) + 35 (order 4) = 70.
#[derive(Debug, Default)]
pub struct MggaOutput<'a> {
    // Order 0 (1 field)
    pub zk: Option<&'a mut [f64]>,

    // Order 1 (4 fields)
    pub vrho: Option<&'a mut [f64]>,
    pub vsigma: Option<&'a mut [f64]>,
    pub vlapl: Option<&'a mut [f64]>,
    pub vtau: Option<&'a mut [f64]>,

    // Order 2 (10 fields)
    pub v2rho2: Option<&'a mut [f64]>,
    pub v2rhosigma: Option<&'a mut [f64]>,
    pub v2rholapl: Option<&'a mut [f64]>,
    pub v2rhotau: Option<&'a mut [f64]>,
    pub v2sigma2: Option<&'a mut [f64]>,
    pub v2sigmalapl: Option<&'a mut [f64]>,
    pub v2sigmatau: Option<&'a mut [f64]>,
    pub v2lapl2: Option<&'a mut [f64]>,
    pub v2lapltau: Option<&'a mut [f64]>,
    pub v2tau2: Option<&'a mut [f64]>,

    // Order 3 (20 fields)
    pub v3rho3: Option<&'a mut [f64]>,
    pub v3rho2sigma: Option<&'a mut [f64]>,
    pub v3rho2lapl: Option<&'a mut [f64]>,
    pub v3rho2tau: Option<&'a mut [f64]>,
    pub v3rhosigma2: Option<&'a mut [f64]>,
    pub v3rhosigmalapl: Option<&'a mut [f64]>,
    pub v3rhosigmatau: Option<&'a mut [f64]>,
    pub v3rholapl2: Option<&'a mut [f64]>,
    pub v3rholapltau: Option<&'a mut [f64]>,
    pub v3rhotau2: Option<&'a mut [f64]>,
    pub v3sigma3: Option<&'a mut [f64]>,
    pub v3sigma2lapl: Option<&'a mut [f64]>,
    pub v3sigma2tau: Option<&'a mut [f64]>,
    pub v3sigmalapl2: Option<&'a mut [f64]>,
    pub v3sigmalapltau: Option<&'a mut [f64]>,
    pub v3sigmatau2: Option<&'a mut [f64]>,
    pub v3lapl3: Option<&'a mut [f64]>,
    pub v3lapl2tau: Option<&'a mut [f64]>,
    pub v3lapltau2: Option<&'a mut [f64]>,
    pub v3tau3: Option<&'a mut [f64]>,

    // Order 4 (35 fields)
    pub v4rho4: Option<&'a mut [f64]>,
    pub v4rho3sigma: Option<&'a mut [f64]>,
    pub v4rho3lapl: Option<&'a mut [f64]>,
    pub v4rho3tau: Option<&'a mut [f64]>,
    pub v4rho2sigma2: Option<&'a mut [f64]>,
    pub v4rho2sigmalapl: Option<&'a mut [f64]>,
    pub v4rho2sigmatau: Option<&'a mut [f64]>,
    pub v4rho2lapl2: Option<&'a mut [f64]>,
    pub v4rho2lapltau: Option<&'a mut [f64]>,
    pub v4rho2tau2: Option<&'a mut [f64]>,
    pub v4rhosigma3: Option<&'a mut [f64]>,
    pub v4rhosigma2lapl: Option<&'a mut [f64]>,
    pub v4rhosigma2tau: Option<&'a mut [f64]>,
    pub v4rhosigmalapl2: Option<&'a mut [f64]>,
    pub v4rhosigmalapltau: Option<&'a mut [f64]>,
    pub v4rhosigmatau2: Option<&'a mut [f64]>,
    pub v4rholapl3: Option<&'a mut [f64]>,
    pub v4rholapl2tau: Option<&'a mut [f64]>,
    pub v4rholapltau2: Option<&'a mut [f64]>,
    pub v4rhotau3: Option<&'a mut [f64]>,
    pub v4sigma4: Option<&'a mut [f64]>,
    pub v4sigma3lapl: Option<&'a mut [f64]>,
    pub v4sigma3tau: Option<&'a mut [f64]>,
    pub v4sigma2lapl2: Option<&'a mut [f64]>,
    pub v4sigma2lapltau: Option<&'a mut [f64]>,
    pub v4sigma2tau2: Option<&'a mut [f64]>,
    pub v4sigmalapl3: Option<&'a mut [f64]>,
    pub v4sigmalapl2tau: Option<&'a mut [f64]>,
    pub v4sigmalapltau2: Option<&'a mut [f64]>,
    pub v4sigmatau3: Option<&'a mut [f64]>,
    pub v4lapl4: Option<&'a mut [f64]>,
    pub v4lapl3tau: Option<&'a mut [f64]>,
    pub v4lapl2tau2: Option<&'a mut [f64]>,
    pub v4lapltau3: Option<&'a mut [f64]>,
    pub v4tau4: Option<&'a mut [f64]>,
}

impl<'a> MggaOutput<'a> {
    /// Validate all `Some` buffers against the expected sizes for the given
    /// number of grid points and spin mode.
    ///
    /// # Errors
    /// Returns [`LibxcRsError::OutputBufferSizeMismatch`] on wrong buffer length.
    pub fn validate(&self, np: usize, spin: Spin) -> Result<(), LibxcRsError> {
        let d = Dimensions::mgga(spin);

        // Order 0
        validate_output_field(&self.zk, "zk", np, d.zk as usize)?;

        // Order 1
        validate_output_field(&self.vrho, "vrho", np, d.vrho as usize)?;
        validate_output_field(&self.vsigma, "vsigma", np, d.vsigma as usize)?;
        validate_output_field(&self.vlapl, "vlapl", np, d.vlapl as usize)?;
        validate_output_field(&self.vtau, "vtau", np, d.vtau as usize)?;

        // Order 2
        validate_output_field(&self.v2rho2, "v2rho2", np, d.v2rho2 as usize)?;
        validate_output_field(&self.v2rhosigma, "v2rhosigma", np, d.v2rhosigma as usize)?;
        validate_output_field(&self.v2rholapl, "v2rholapl", np, d.v2rholapl as usize)?;
        validate_output_field(&self.v2rhotau, "v2rhotau", np, d.v2rhotau as usize)?;
        validate_output_field(&self.v2sigma2, "v2sigma2", np, d.v2sigma2 as usize)?;
        validate_output_field(&self.v2sigmalapl, "v2sigmalapl", np, d.v2sigmalapl as usize)?;
        validate_output_field(&self.v2sigmatau, "v2sigmatau", np, d.v2sigmatau as usize)?;
        validate_output_field(&self.v2lapl2, "v2lapl2", np, d.v2lapl2 as usize)?;
        validate_output_field(&self.v2lapltau, "v2lapltau", np, d.v2lapltau as usize)?;
        validate_output_field(&self.v2tau2, "v2tau2", np, d.v2tau2 as usize)?;

        // Order 3
        validate_output_field(&self.v3rho3, "v3rho3", np, d.v3rho3 as usize)?;
        validate_output_field(&self.v3rho2sigma, "v3rho2sigma", np, d.v3rho2sigma as usize)?;
        validate_output_field(&self.v3rho2lapl, "v3rho2lapl", np, d.v3rho2lapl as usize)?;
        validate_output_field(&self.v3rho2tau, "v3rho2tau", np, d.v3rho2tau as usize)?;
        validate_output_field(&self.v3rhosigma2, "v3rhosigma2", np, d.v3rhosigma2 as usize)?;
        validate_output_field(&self.v3rhosigmalapl, "v3rhosigmalapl", np, d.v3rhosigmalapl as usize)?;
        validate_output_field(&self.v3rhosigmatau, "v3rhosigmatau", np, d.v3rhosigmatau as usize)?;
        validate_output_field(&self.v3rholapl2, "v3rholapl2", np, d.v3rholapl2 as usize)?;
        validate_output_field(&self.v3rholapltau, "v3rholapltau", np, d.v3rholapltau as usize)?;
        validate_output_field(&self.v3rhotau2, "v3rhotau2", np, d.v3rhotau2 as usize)?;
        validate_output_field(&self.v3sigma3, "v3sigma3", np, d.v3sigma3 as usize)?;
        validate_output_field(&self.v3sigma2lapl, "v3sigma2lapl", np, d.v3sigma2lapl as usize)?;
        validate_output_field(&self.v3sigma2tau, "v3sigma2tau", np, d.v3sigma2tau as usize)?;
        validate_output_field(&self.v3sigmalapl2, "v3sigmalapl2", np, d.v3sigmalapl2 as usize)?;
        validate_output_field(&self.v3sigmalapltau, "v3sigmalapltau", np, d.v3sigmalapltau as usize)?;
        validate_output_field(&self.v3sigmatau2, "v3sigmatau2", np, d.v3sigmatau2 as usize)?;
        validate_output_field(&self.v3lapl3, "v3lapl3", np, d.v3lapl3 as usize)?;
        validate_output_field(&self.v3lapl2tau, "v3lapl2tau", np, d.v3lapl2tau as usize)?;
        validate_output_field(&self.v3lapltau2, "v3lapltau2", np, d.v3lapltau2 as usize)?;
        validate_output_field(&self.v3tau3, "v3tau3", np, d.v3tau3 as usize)?;

        // Order 4
        validate_output_field(&self.v4rho4, "v4rho4", np, d.v4rho4 as usize)?;
        validate_output_field(&self.v4rho3sigma, "v4rho3sigma", np, d.v4rho3sigma as usize)?;
        validate_output_field(&self.v4rho3lapl, "v4rho3lapl", np, d.v4rho3lapl as usize)?;
        validate_output_field(&self.v4rho3tau, "v4rho3tau", np, d.v4rho3tau as usize)?;
        validate_output_field(&self.v4rho2sigma2, "v4rho2sigma2", np, d.v4rho2sigma2 as usize)?;
        validate_output_field(&self.v4rho2sigmalapl, "v4rho2sigmalapl", np, d.v4rho2sigmalapl as usize)?;
        validate_output_field(&self.v4rho2sigmatau, "v4rho2sigmatau", np, d.v4rho2sigmatau as usize)?;
        validate_output_field(&self.v4rho2lapl2, "v4rho2lapl2", np, d.v4rho2lapl2 as usize)?;
        validate_output_field(&self.v4rho2lapltau, "v4rho2lapltau", np, d.v4rho2lapltau as usize)?;
        validate_output_field(&self.v4rho2tau2, "v4rho2tau2", np, d.v4rho2tau2 as usize)?;
        validate_output_field(&self.v4rhosigma3, "v4rhosigma3", np, d.v4rhosigma3 as usize)?;
        validate_output_field(&self.v4rhosigma2lapl, "v4rhosigma2lapl", np, d.v4rhosigma2lapl as usize)?;
        validate_output_field(&self.v4rhosigma2tau, "v4rhosigma2tau", np, d.v4rhosigma2tau as usize)?;
        validate_output_field(&self.v4rhosigmalapl2, "v4rhosigmalapl2", np, d.v4rhosigmalapl2 as usize)?;
        validate_output_field(&self.v4rhosigmalapltau, "v4rhosigmalapltau", np, d.v4rhosigmalapltau as usize)?;
        validate_output_field(&self.v4rhosigmatau2, "v4rhosigmatau2", np, d.v4rhosigmatau2 as usize)?;
        validate_output_field(&self.v4rholapl3, "v4rholapl3", np, d.v4rholapl3 as usize)?;
        validate_output_field(&self.v4rholapl2tau, "v4rholapl2tau", np, d.v4rholapl2tau as usize)?;
        validate_output_field(&self.v4rholapltau2, "v4rholapltau2", np, d.v4rholapltau2 as usize)?;
        validate_output_field(&self.v4rhotau3, "v4rhotau3", np, d.v4rhotau3 as usize)?;
        validate_output_field(&self.v4sigma4, "v4sigma4", np, d.v4sigma4 as usize)?;
        validate_output_field(&self.v4sigma3lapl, "v4sigma3lapl", np, d.v4sigma3lapl as usize)?;
        validate_output_field(&self.v4sigma3tau, "v4sigma3tau", np, d.v4sigma3tau as usize)?;
        validate_output_field(&self.v4sigma2lapl2, "v4sigma2lapl2", np, d.v4sigma2lapl2 as usize)?;
        validate_output_field(&self.v4sigma2lapltau, "v4sigma2lapltau", np, d.v4sigma2lapltau as usize)?;
        validate_output_field(&self.v4sigma2tau2, "v4sigma2tau2", np, d.v4sigma2tau2 as usize)?;
        validate_output_field(&self.v4sigmalapl3, "v4sigmalapl3", np, d.v4sigmalapl3 as usize)?;
        validate_output_field(&self.v4sigmalapl2tau, "v4sigmalapl2tau", np, d.v4sigmalapl2tau as usize)?;
        validate_output_field(&self.v4sigmalapltau2, "v4sigmalapltau2", np, d.v4sigmalapltau2 as usize)?;
        validate_output_field(&self.v4sigmatau3, "v4sigmatau3", np, d.v4sigmatau3 as usize)?;
        validate_output_field(&self.v4lapl4, "v4lapl4", np, d.v4lapl4 as usize)?;
        validate_output_field(&self.v4lapl3tau, "v4lapl3tau", np, d.v4lapl3tau as usize)?;
        validate_output_field(&self.v4lapl2tau2, "v4lapl2tau2", np, d.v4lapl2tau2 as usize)?;
        validate_output_field(&self.v4lapltau3, "v4lapltau3", np, d.v4lapltau3 as usize)?;
        validate_output_field(&self.v4tau4, "v4tau4", np, d.v4tau4 as usize)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── LdaOutput tests ────────────────────────────────────────────

    #[test]
    fn lda_output_all_none_ok() {
        let out = LdaOutput::new(None, None, None, None, None, 10, Spin::Unpolarized);
        assert!(out.is_ok());
    }

    #[test]
    fn lda_output_zk_correct_size() {
        let mut zk = vec![0.0; 10];
        let out = LdaOutput::new(Some(&mut zk), None, None, None, None, 10, Spin::Unpolarized);
        assert!(out.is_ok());
    }

    #[test]
    fn lda_output_zk_wrong_size() {
        let mut zk = vec![0.0; 5];
        let err = LdaOutput::new(Some(&mut zk), None, None, None, None, 10, Spin::Unpolarized)
            .unwrap_err();
        match err {
            LibxcRsError::OutputBufferSizeMismatch { field, expected, actual } => {
                assert_eq!(field, "zk");
                assert_eq!(expected, 10);
                assert_eq!(actual, 5);
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn lda_output_polarized_vrho() {
        // Polarized: vrho dim = 2
        let mut vrho = vec![0.0; 20];
        let out = LdaOutput::new(None, Some(&mut vrho), None, None, None, 10, Spin::Polarized);
        assert!(out.is_ok());
    }

    #[test]
    fn lda_output_polarized_v2rho2() {
        // Polarized: v2rho2 dim = 3
        let mut v2 = vec![0.0; 30];
        let out = LdaOutput::new(None, None, Some(&mut v2), None, None, 10, Spin::Polarized);
        assert!(out.is_ok());
    }

    #[test]
    fn lda_output_all_fields_polarized() {
        let np = 10;
        let mut zk = vec![0.0; np];        // zk=1
        let mut vrho = vec![0.0; np * 2];   // vrho=2
        let mut v2 = vec![0.0; np * 3];     // v2rho2=3
        let mut v3 = vec![0.0; np * 4];     // v3rho3=4
        let mut v4 = vec![0.0; np * 5];     // v4rho4=5
        let out = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2),
            Some(&mut v3), Some(&mut v4), np, Spin::Polarized,
        );
        assert!(out.is_ok());
    }

    // ── GgaOutput tests ────────────────────────────────────────────

    #[test]
    fn gga_output_all_none_ok() {
        let out = GgaOutput::new(
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            10, Spin::Unpolarized,
        );
        assert!(out.is_ok());
    }

    #[test]
    fn gga_output_mixed_some_none_unpolarized() {
        let np = 10;
        let mut zk = vec![0.0; np];
        let mut vsigma = vec![0.0; np]; // unpolarized sigma dim = 1
        let out = GgaOutput::new(
            Some(&mut zk), None, Some(&mut vsigma),
            None, None, None, None, None, None, None,
            None, None, None, None, None,
            np, Spin::Unpolarized,
        );
        assert!(out.is_ok());
    }

    #[test]
    fn gga_output_vsigma_wrong_size() {
        let np = 10;
        let mut vsigma = vec![0.0; 5]; // wrong
        let err = GgaOutput::new(
            None, None, Some(&mut vsigma),
            None, None, None, None, None, None, None,
            None, None, None, None, None,
            np, Spin::Unpolarized,
        ).unwrap_err();
        match err {
            LibxcRsError::OutputBufferSizeMismatch { field, .. } => {
                assert_eq!(field, "vsigma");
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn gga_output_polarized_v4sigma4() {
        let np = 10;
        let mut v4 = vec![0.0; np * 15]; // v4sigma4 polarized = 15
        let out = GgaOutput::new(
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, Some(&mut v4),
            np, Spin::Polarized,
        );
        assert!(out.is_ok());
    }

    // ── MggaOutput tests ───────────────────────────────────────────

    #[test]
    fn mgga_output_default_all_none_validates() {
        let out = MggaOutput::default();
        assert!(out.validate(10, Spin::Unpolarized).is_ok());
    }

    #[test]
    fn mgga_output_with_zk_and_vrho() {
        let np = 5;
        let mut zk = vec![0.0; np];
        let mut vrho = vec![0.0; np];
        let out = MggaOutput {
            zk: Some(&mut zk),
            vrho: Some(&mut vrho),
            ..Default::default()
        };
        assert!(out.validate(np, Spin::Unpolarized).is_ok());
    }

    #[test]
    fn mgga_output_wrong_vtau_size() {
        let np = 5;
        let mut vtau = vec![0.0; 3]; // wrong: should be 5 for unpolarized
        let out = MggaOutput {
            vtau: Some(&mut vtau),
            ..Default::default()
        };
        let err = out.validate(np, Spin::Unpolarized).unwrap_err();
        match err {
            LibxcRsError::OutputBufferSizeMismatch { field, expected, actual } => {
                assert_eq!(field, "vtau");
                assert_eq!(expected, 5);
                assert_eq!(actual, 3);
            }
            _ => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn mgga_output_polarized_v4rho4() {
        let np = 10;
        let mut v4 = vec![0.0; np * 5]; // v4rho4 polarized = 5
        let out = MggaOutput {
            v4rho4: Some(&mut v4),
            ..Default::default()
        };
        assert!(out.validate(np, Spin::Polarized).is_ok());
    }

    #[test]
    fn mgga_has_70_option_fields() {
        // Verify the field count by counting the fields in the default value.
        // We can check by ensuring all 70 fields are None in the default.
        let out = MggaOutput::default();
        // Order 0: 1
        assert!(out.zk.is_none());
        // Order 1: 4
        assert!(out.vrho.is_none());
        assert!(out.vsigma.is_none());
        assert!(out.vlapl.is_none());
        assert!(out.vtau.is_none());
        // Order 2: 10
        assert!(out.v2rho2.is_none());
        assert!(out.v2rhosigma.is_none());
        assert!(out.v2rholapl.is_none());
        assert!(out.v2rhotau.is_none());
        assert!(out.v2sigma2.is_none());
        assert!(out.v2sigmalapl.is_none());
        assert!(out.v2sigmatau.is_none());
        assert!(out.v2lapl2.is_none());
        assert!(out.v2lapltau.is_none());
        assert!(out.v2tau2.is_none());
        // Order 3: 20
        assert!(out.v3rho3.is_none());
        assert!(out.v3rho2sigma.is_none());
        assert!(out.v3rho2lapl.is_none());
        assert!(out.v3rho2tau.is_none());
        assert!(out.v3rhosigma2.is_none());
        assert!(out.v3rhosigmalapl.is_none());
        assert!(out.v3rhosigmatau.is_none());
        assert!(out.v3rholapl2.is_none());
        assert!(out.v3rholapltau.is_none());
        assert!(out.v3rhotau2.is_none());
        assert!(out.v3sigma3.is_none());
        assert!(out.v3sigma2lapl.is_none());
        assert!(out.v3sigma2tau.is_none());
        assert!(out.v3sigmalapl2.is_none());
        assert!(out.v3sigmalapltau.is_none());
        assert!(out.v3sigmatau2.is_none());
        assert!(out.v3lapl3.is_none());
        assert!(out.v3lapl2tau.is_none());
        assert!(out.v3lapltau2.is_none());
        assert!(out.v3tau3.is_none());
        // Order 4: 35
        assert!(out.v4rho4.is_none());
        assert!(out.v4rho3sigma.is_none());
        assert!(out.v4rho3lapl.is_none());
        assert!(out.v4rho3tau.is_none());
        assert!(out.v4rho2sigma2.is_none());
        assert!(out.v4rho2sigmalapl.is_none());
        assert!(out.v4rho2sigmatau.is_none());
        assert!(out.v4rho2lapl2.is_none());
        assert!(out.v4rho2lapltau.is_none());
        assert!(out.v4rho2tau2.is_none());
        assert!(out.v4rhosigma3.is_none());
        assert!(out.v4rhosigma2lapl.is_none());
        assert!(out.v4rhosigma2tau.is_none());
        assert!(out.v4rhosigmalapl2.is_none());
        assert!(out.v4rhosigmalapltau.is_none());
        assert!(out.v4rhosigmatau2.is_none());
        assert!(out.v4rholapl3.is_none());
        assert!(out.v4rholapl2tau.is_none());
        assert!(out.v4rholapltau2.is_none());
        assert!(out.v4rhotau3.is_none());
        assert!(out.v4sigma4.is_none());
        assert!(out.v4sigma3lapl.is_none());
        assert!(out.v4sigma3tau.is_none());
        assert!(out.v4sigma2lapl2.is_none());
        assert!(out.v4sigma2lapltau.is_none());
        assert!(out.v4sigma2tau2.is_none());
        assert!(out.v4sigmalapl3.is_none());
        assert!(out.v4sigmalapl2tau.is_none());
        assert!(out.v4sigmalapltau2.is_none());
        assert!(out.v4sigmatau3.is_none());
        assert!(out.v4lapl4.is_none());
        assert!(out.v4lapl3tau.is_none());
        assert!(out.v4lapl2tau2.is_none());
        assert!(out.v4lapltau3.is_none());
        assert!(out.v4tau4.is_none());
        // Total: 1 + 4 + 10 + 20 + 35 = 70
    }
}
=======
//! Placeholder module generated from README tree for src/output.

pub mod bundle;
pub mod gga;
pub mod lda;
pub mod mgga;
pub mod request;
pub mod resident;
>>>>>>> origin/main
