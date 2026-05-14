//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1310/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1310<F: Float>(t1858: F, t8843: F, t2174: F, t7945: F, t34175: F, t580: F, t2169: F, t7961: F, t34194: F, t576: F, t117418: F, t117430: F, t1396: F, t1404: F, t2105: F, t27241: F, t27908: F, t34176: F, t5364: F, t5381: F, t7223: F, t7240: F, t7426: F, t7946: F, t8111: F, t8119: F, t8844: F, t8852: F) -> (F,) {
    let t125065 = t8843 * t1858;
    let t125067 = t7945 * t2174;
    let t125068 = t34175 * t580;
    let t125069 = t2169 * t7961;
    let t125071 = t576 * t34194;
    let t125073 = t1396 * t34194 + t1404 * t34176 + t2105 * t27908 + t2174 * t27241 + t5364 * t8852 + t5381 * t8844 + t7223 * t8119 + t7240 * t8111 + t7426 * t7946 + t117418 + t117430 + t125065 + t125067 + t125068 + t125069 + t125071;
    (t125073,)
}
