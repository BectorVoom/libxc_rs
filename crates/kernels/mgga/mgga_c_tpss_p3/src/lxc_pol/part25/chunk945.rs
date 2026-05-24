//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 945/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk945<F: Float>(t4101: F, t673: F, t1515: F, t2202: F, t4048: F, t664: F) -> (F, F, F, F) {
    let t11844 = t673 * t4101;
    let t11845 = F::cast_from(0.10954222222222222222e0_f64) * t11844;
    let t11850 = t2202 * t1515;
    let t11873 = t664 * t4048;
    (t11844, t11845, t11850, t11873)
}
