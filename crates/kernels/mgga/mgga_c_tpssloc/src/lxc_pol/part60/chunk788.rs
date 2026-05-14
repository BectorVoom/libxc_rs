//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 788/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk788<F: Float>(t23270: F, t32862: F, t1888: F, t6571: F, t7537: F, t6553: F, t1880: F, t25224: F, t8335: F, t25: F, t7540: F, t28: F, t3701: F, t7752: F, t4028: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32863 = t23270 * t32862;
    let t32865 = 0.3289868133696452873e-1 * t1888 * t32863;
    let t32866 = t6571 * t7537;
    let t32867 = t6553 * t32866;
    let t32869 = 0.16449340668482264365e-1 * t1880 * t32867;
    let t32875 = t25224 * t8335;
    let t32877 = 0.16449340668482264365e-1 * t1880 * t32875;
    let t32899 = t25 * t7540;
    let t33065 = t28 * t7540;
    let t33136 = t3701 * t7752;
    let t33151 = t4028 * t8326;
    (t32863, t32865, t32866, t32867, t32869, t32875, t32877, t32899, t33065, t33136, t33151)
}
