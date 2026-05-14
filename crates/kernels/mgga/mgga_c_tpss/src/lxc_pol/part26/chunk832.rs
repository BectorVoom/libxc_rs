//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 832/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk832<F: Float>(t5772: F, t645: F, t547: F, t117: F, t5531: F, t1859: F, t38: F) -> (F, F, F, F, F) {
    let t5773 = t5772 * t645;
    let t5775 = 6.0 * t547 * t5773;
    let t5776 = t117 * t5531;
    let t5778 = 3.0 * t547 * t5776;
    let t5965 = t38 * t1859;
    (t5773, t5775, t5776, t5778, t5965)
}
