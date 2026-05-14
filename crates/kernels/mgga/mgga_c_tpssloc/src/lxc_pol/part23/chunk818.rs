//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 818/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk818<F: Float>(t3684: F, t9882: F, t9888: F, t9885: F, t3824: F, t588: F, t1287: F, t2225: F, t521: F, t9861: F, t17: F, t1294: F, t9494: F, t1995: F, t68: F, t215: F, t535: F, t9569: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12114 = 0.32530743900905219526e-1 * t3684 * t9882;
    let t12116 = 0.48159733137676571078e0 * t3684 * t9888;
    let t12118 = 0.16265371950452609763e-1 * t3684 * t9885;
    let t12120 = t588 * t3824;
    let t12121 = 12.0 * t12120;
    let t12123 = 60.0 * t2225 * t1287;
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12141 = 0.10254018858216406658e4 * t1294 * t9494;
    let t12155 = t68 * t1995;
    let t12188 = 0.28086419753086419752e-1 * t9569 * t535 * t215;
    (t12114, t12116, t12118, t12121, t12123, t12132, t12133, t12141, t12155, t12188)
}
