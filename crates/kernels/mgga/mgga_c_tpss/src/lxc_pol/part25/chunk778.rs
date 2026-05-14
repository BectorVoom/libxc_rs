//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 778/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk778<F: Float>(t1250: F, t5728: F, t1253: F, t1705: F, t935: F, t1771: F, t5570: F) -> (F, F, F, F) {
    let t5729 = t5728 * t1250;
    let t5736 = t1705 * t1253;
    let t5737 = t5736 * t935;
    let t5739 = t1771 * t5570;
    (t5729, t5736, t5737, t5739)
}
