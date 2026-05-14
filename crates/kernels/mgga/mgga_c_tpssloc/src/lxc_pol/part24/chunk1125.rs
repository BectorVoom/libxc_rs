//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1125/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1125<F: Float>(t3127: F, t381: F, t23602: F, t1014: F, t1936: F, t362: F, t2775: F, t23509: F, t3: F, t23470: F, t3030: F, t1022: F, t23678: F, t1011: F, t360: F, t23478: F, t6785: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25483 = t3127 * t381;
    let t25484 = t23602 * t25483;
    let t25490 = t1014 * t381;
    let t25491 = t23602 * t25490;
    let t25510 = t1936 * t362;
    let t25511 = t381 * t2775;
    let t25650 = t23509 * t3;
    let t25651 = t23470 * t3030;
    let t25652 = t25650 * t25651;
    let t25654 = t23678 * t1022;
    let t25659 = t1011 * t1022;
    let t25660 = t25659 * t360;
    let t25713 = t23478 * t6785;
    (t25484, t25490, t25491, t25510, t25511, t25650, t25651, t25652, t25654, t25660, t25713)
}
