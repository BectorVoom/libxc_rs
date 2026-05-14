//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1022/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1022<F: Float>(t3845: F, t884: F, t1437: F, t2569: F, t2551: F, t3848: F, t2577: F, t3844: F, t11245: F, t11248: F, t11251: F, t11255: F, t11258: F, t11262: F, t11265: F, t2550: F, t2575: F, t3827: F, t3849: F, t8842: F, t8847: F, t8899: F) -> (F,) {
    let t11418 = t3845 * t884;
    let t11421 = t1437 * t2569;
    let t11424 = t3848 * t2551;
    let t11427 = t3844 * t2577;
    let t11428 = t11427 * t884;
    let t11431 = t11245 + t11248 + t11251 - t11255 - t11258 - t11262 - t11265 - 4.0 * t8899 * t3827 + 0.64327917994770140268e2 * t8842 * t3849 - 4.0 * t2550 * t11418 - 2.0 * t2550 * t11421 - 0.19298375398431042081e3 * t8847 * t11424 + 0.64327917994770140268e2 * t2575 * t11428;
    (t11431,)
}
