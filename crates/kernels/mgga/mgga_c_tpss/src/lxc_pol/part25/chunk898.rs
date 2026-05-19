//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 898/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk898<F: Float>(t8660: F, t2529: F, t844: F, t269: F, t2532: F, t284: F, t2480: F, t841: F, t2617: F, t894: F, t2620: F, t317: F) -> (F, F, F, F, F, F, F) {
    let t8687 = F::new(28.0) / F::new(27.0) * t8660;
    let t8709 = F::new(1.0) / t2529 / t844;
    let t8710 = t269 * t8709;
    let t8712 = F::new(1.0) / t2532 / t284;
    let t8723 = F::cast_from(0.55403703703703703703e-1_f64) * t8660;
    let t8737 = t841 * t2480;
    let t8749 = F::new(1.0) / t2617 / t894;
    let t8752 = F::new(1.0) / t2620 / t317;
    (t8687, t8710, t8712, t8723, t8737, t8749, t8752)
}
