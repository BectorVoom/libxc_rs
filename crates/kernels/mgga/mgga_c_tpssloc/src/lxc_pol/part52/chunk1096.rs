//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1096/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1096<F: Float>(t1527: F, t30633: F, t23270: F, t1888: F, t6571: F, t7537: F, t6553: F, t1880: F, t25224: F, t8335: F, t1492: F, t8347: F, t218: F, t32849: F, t1528: F, t1912: F, t25188: F, t25348: F, t259: F, t30655: F, t30662: F, t30741: F, t30748: F, t4147: F, t6627: F, t7538: F, t8363: F) -> (F, F, F, F, F, F, F, F) {
    let t32862 = t30633 * t1527;
    let t32863 = t23270 * t32862;
    let t32865 = 0.3289868133696452873e-1 * t1888 * t32863;
    let t32866 = t6571 * t7537;
    let t32867 = t6553 * t32866;
    let t32869 = 0.16449340668482264365e-1 * t1880 * t32867;
    let t32875 = t25224 * t8335;
    let t32877 = 0.16449340668482264365e-1 * t1880 * t32875;
    let t32878 = t1492 * t8347;
    let t32880 = t218 * t32849;
    let t32884 = -t1528 * t30741 - 2.0 * t1912 * t25188 - 2.0 * t1912 * t25348 + t259 * t32878 + t259 * t32880 - t4147 * t8363 - 2.0 * t6627 * t7538 - t30655 + t30662 + t30748 + t32865 - t32869 - t32877;
    (t32862, t32863, t32866, t32867, t32875, t32878, t32880, t32884)
}
