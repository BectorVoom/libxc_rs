//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 890/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk890<F: Float>(t1401: F, t1458: F, t2199: F, t3941: F, t5371: F, t577: F, t8207: F, t8273: F, t8283: F, t8294: F, t590: F, t60: F) -> (F, F) {
    let t8299 = F::new(0.45e1) * t8283 * t577 + F::new(0.135e2) * t8207 * t1458 + F::new(0.135e2) * t5371 * t2199 + F::new(27.0) * t3941 * t8294 + F::new(0.135e2) * t1401 * t8273;
    let t8705 = F::new(1.0) / t60 / t590;
    (t8299, t8705)
}
