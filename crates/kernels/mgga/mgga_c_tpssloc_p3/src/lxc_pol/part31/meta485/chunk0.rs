//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1653/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1653<F: Float>(t235: F, t26653: F, t1509: F, t2047: F, t4182: F, t7823: F, t814: F, t829: F, t25293: F, t25317: F, t226: F, t23187: F, t25274: F, t25285: F, t25289: F, t25301: F, t25304: F, t25308: F, t25310: F, t25314: F, t25322: F, t25326: F, t26613: F, t4281: F, t4291: F, t7839: F, t808: F, t812: F) -> (F, F, F, F, F, F, F) {
    let t26654 = t235 * t26653;
    let t26656 = t2047 * t1509;
    let t26657 = t26656 * t4182;
    let t26661 = t814 * t7823;
    let t26662 = t26661 * t829;
    let t26667 = F::cast_from(0.38381794893125283518e-1_f64) * t25293;
    let t26673 = F::cast_from(0.16449340668482264365e-1_f64) * t25317;
    let t26676 = t26656 * t829;
    let t26678 = -F::cast_from(0.16449340668482264365e-1_f64) * t25274 + t26613 + t226 * t26654 + F::new(2.0) * t4281 * t26657 + F::cast_from(0.82246703342411321825e-2_f64) * t23187 - t812 * t26662 - F::cast_from(0.16449340668482264365e-1_f64) * t25285 + F::cast_from(0.3289868133696452873e-1_f64) * t25289 + t808 * t7839 - t26667 + F::cast_from(0.3289868133696452873e-1_f64) * t25301 + F::cast_from(0.3289868133696452873e-1_f64) * t25304 - F::cast_from(0.3289868133696452873e-1_f64) * t25308 + F::cast_from(0.76763589786250567037e-1_f64) * t25310 - F::cast_from(0.3289868133696452873e-1_f64) * t25314 + t26673 - F::cast_from(0.3289868133696452873e-1_f64) * t25322 + F::cast_from(0.16449340668482264365e-1_f64) * t25326 - t4291 * t26676;
    (t26654, t26656, t26657, t26661, t26662, t26676, t26678)
}
