//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1119/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1119<F: Float>(t112: F, t34228: F, t225: F, t497: F, t8054: F, t462: F, t1716: F, t8867: F, t27751: F, t8871: F, t32543: F, t8014: F) -> (F, F, F, F, F, F) {
    let t34229 = t34228 * t112;
    let t34237 = t8054 * t225 * t497;
    let t34238 = t462 * t34237;
    let t34241 = t1716 * t8867;
    let t34244 = t27751 * t8871;
    let t34247 = t32543 * t8014;
    (t34229, t34237, t34238, t34241, t34244, t34247)
}
