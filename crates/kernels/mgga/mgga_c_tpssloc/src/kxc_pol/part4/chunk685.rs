//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 685/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk685<F: Float>(t1629: F, t4673: F, t1049: F, t1615: F, t1060: F, t381: F, t4649: F, t1022: F, t1932: F, t360: F, t1625: F, t383: F, t4657: F) -> (F, F, F, F, F, F, F) {
    let t4674 = t1629 * t4673;
    let t4677 = t1049 * t1615;
    let t4678 = t4677 * t1060;
    let t4680 = t381 * t4649;
    let t4681 = t4680 * t1060;
    let t4684 = t1932 * t1022 * t360;
    let t4685 = t1629 * t4684;
    let t4688 = t1625 * t1022;
    let t4689 = t4688 * t1060;
    let t4691 = t383 * t4657;
    (t4674, t4678, t4681, t4684, t4685, t4689, t4691)
}
