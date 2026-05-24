//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 860/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk860<F: Float>(t27: F, t559: F, t20: F, t571: F, t12: F, t558: F, t1965: F, t1971: F, t567: F, t1970: F, t3: F, t25: F) -> (F, F, F, F, F, F) {
    let t7662 = t559 * t27;
    let t7665 = F::new(120.0) * t20 * t571;
    let t7666 = t12 * t558;
    let t7668 = F::new(120.0) * t7666 * t27;
    let t7669 = t1965 * t571;
    let t7671 = t567 * t1971;
    let t7673 = t1970 * t3;
    let t7674 = F::new(1.0) / t7673;
    let t7676 = F::new(336.0) * t25 * t7674;
    (t7662, t7665, t7668, t7669, t7671, t7676)
}
