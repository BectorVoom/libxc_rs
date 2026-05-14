//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 838/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk838<F: Float>(t3684: F, t9467: F, t118: F, t1284: F, t2375: F, t9882: F, t9888: F, t9885: F, t3824: F, t588: F, t1287: F, t2225: F, t2516: F, t17: F, t521: F, t9861: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12109 = 0.21687162600603479684e-1 * t3684 * t9467;
    let t12110 = t1284 * t118;
    let t12111 = t12110 * t2375;
    let t12114 = 0.32530743900905219526e-1 * t3684 * t9882;
    let t12116 = 0.48159733137676571078e0 * t3684 * t9888;
    let t12118 = 0.16265371950452609763e-1 * t3684 * t9885;
    let t12120 = t588 * t3824;
    let t12123 = 60.0 * t2225 * t1287;
    let t12129 = t1284 * t2516;
    let t12130 = t17 * t12129;
    let t12132 = t521 * t9861;
    (t12109, t12111, t12114, t12116, t12118, t12120, t12123, t12130, t12132)
}
