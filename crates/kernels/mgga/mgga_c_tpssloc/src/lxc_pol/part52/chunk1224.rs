//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1224/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1224<F: Float>(t31918: F, t4028: F, t26114: F, t8675: F, t26179: F, t31908: F, t7458: F, t113: F, t119792: F, t119858: F, t119862: F, t123044: F, t123074: F, t123088: F, t1774: F, t1976: F, t27371: F, t31877: F, t31880: F, t32674: F, t4073: F, t5107: F, t574: F, t8667: F) -> (F,) {
    let t123091 = t4028 * t31918;
    let t123093 = t26114 * t8675;
    let t123095 = t26179 * t8675;
    let t123097 = t7458 * t31908;
    let t123101 = t119858 - t113 * (t123044 + t119792) - t31877 * t1774 - t8667 * t5107 - t27371 * t1976 + (t123074 + t123088) * t574 - 2.0 * t123091 - 2.0 * t123093 - 2.0 * t123095 - 2.0 * t123097 - 2.0 * t31880 * t4073 - t119862 - t32674;
    (t123101,)
}
