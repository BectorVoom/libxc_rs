//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 961/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk961<F: Float>(t2098: F, t8119: F, t1851: F, t8852: F, t1858: F, t8843: F, t2174: F, t7945: F, t34175: F, t580: F, t2169: F, t7961: F) -> (F, F, F, F, F, F) {
    let t125050 = t2098 * t8119;
    let t125053 = t1851 * t8852;
    let t125065 = t8843 * t1858;
    let t125067 = t7945 * t2174;
    let t125068 = t34175 * t580;
    let t125069 = t2169 * t7961;
    (t125050, t125053, t125065, t125067, t125068, t125069)
}
