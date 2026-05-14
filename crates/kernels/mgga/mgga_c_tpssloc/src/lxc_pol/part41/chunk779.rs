//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 779/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk779<F: Float>(t1088: F, t5979: F, t123: F, t3237: F, t4721: F, t5973: F, t5977: F, t423: F, t1671: F, t4740: F, t1670: F, t1118: F, t3264: F, t1661: F, t3270: F, t3274: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5980 = t1088 * t5979;
    let t5981 = t123 * t5980;
    let t5983 = t3237 - 0.11872222222222222222e-1 * t4721 - 0.11872222222222222222e-1 * t5973 + 0.35616666666666666666e-1 * t5977 + 0.17808333333333333333e-1 * t5981;
    let t5985 = 0.621814e-1 * t5983 * t423;
    let t5987 = 2.0 * t4740 * t1671;
    let t5988 = t1670 * t1670;
    let t5989 = t5988 * t1118;
    let t5991 = 2.0 * t3264 * t5989;
    let t5992 = t1661 * t1661;
    let t5993 = t3270 * t5992;
    let t5999 = t3274 - 2.0 / 9.0 * t4721 - 2.0 / 9.0 * t5973 + 2.0 / 3.0 * t5977 + t5981 / 3.0;
    (t5980, t5981, t5983, t5985, t5987, t5988, t5989, t5991, t5992, t5993, t5999)
}
