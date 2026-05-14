//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 942/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk942<F: Float>(t11687: F, t946: F, t1407: F, t242: F, t8951: F, t967: F, t2748: F, t3969: F, t2675: F, t3950: F, t219: F, t3988: F, t4101: F, t673: F, t1515: F, t2202: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11688 = t946 * t11687;
    let t11691 = t242 * t8951 * t1407;
    let t11692 = t967 * t11691;
    let t11697 = t2748 * t3969 / 648.0;
    let t11701 = t242 * t2675 * t3950;
    let t11703 = t946 * t11701 / 2304.0;
    let t11710 = t3988 * t219;
    let t11844 = t673 * t4101;
    let t11845 = 0.10954222222222222222e0 * t11844;
    let t11850 = t2202 * t1515;
    (t11688, t11691, t11692, t11697, t11701, t11703, t11710, t11844, t11845, t11850)
}
