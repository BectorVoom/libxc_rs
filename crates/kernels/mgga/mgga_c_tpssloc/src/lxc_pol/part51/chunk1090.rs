//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1090/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1090<F: Float>(t1998: F, t7722: F, t214: F, t1985: F, t225: F, t567: F, t26193: F, t8458: F, t30663: F, t7479: F, t6552: F, t7488: F, t1880: F, t258: F, t7510: F, t1484: F, t30622: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32748 = t1998 * t7722;
    let t32749 = t214 * t32748;
    let t32751 = 0.16449340668482264365e-1 * t1985 * t32749;
    let t32761 = t7722 * t225 * t567;
    let t32762 = t214 * t32761;
    let t32764 = 0.16449340668482264365e-1 * t1985 * t32762;
    let t32769 = t26193 * t8458;
    let t32771 = 0.16449340668482264365e-1 * t1985 * t32769;
    let t32789 = t30663 * t7479;
    let t32791 = 0.3289868133696452873e-1 * t6552 * t32789;
    let t32792 = t30663 * t7488;
    let t32794 = 0.16449340668482264365e-1 * t1880 * t32792;
    let t32808 = t7510 * t225 * t258;
    let t32809 = t214 * t32808;
    let t32811 = 0.16449340668482264365e-1 * t1880 * t32809;
    let t32814 = t30622 * t1484;
    (t32748, t32749, t32751, t32761, t32762, t32764, t32769, t32771, t32789, t32791, t32792, t32794, t32808, t32809, t32811, t32814)
}
