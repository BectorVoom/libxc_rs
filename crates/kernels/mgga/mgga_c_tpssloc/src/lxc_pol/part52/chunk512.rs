//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 512/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk512<F: Float>(t706: F, t717: F, t607: F, t751: F, t707: F, t195: F, t197: F, t676: F, t724: F, t164: F, t723: F, t159: F) -> (F, F, F, F, F, F) {
    let t2427 = t706 * t717;
    let t2430 = t751 * t607;
    let t2431 = t707 * t2430;
    let t2433 = F::new(1.0) / t195;
    let t2440 = F::new(1.0) / t197;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = F::new(1.0) / t2458;
    let t2460 = t159 * t2459;
    (t2427, t2431, t2433, t2440, t2454, t2460)
}
