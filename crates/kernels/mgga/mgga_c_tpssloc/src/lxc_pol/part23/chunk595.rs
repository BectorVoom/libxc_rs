//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 595/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk595<F: Float>(t1409: F, t3242: F, t3247: F, t1098: F, t1657: F, t1661: F, t3270: F, t3287: F, t1667: F, t699: F, t1128: F, t1675: F, t1147: F, t1687: F, t1694: F, t3403: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4723 = t3242 * t1409;
    let t4728 = t3247 * t1409;
    let t4740 = t1657 * t1098;
    let t4748 = t3270 * t1661;
    let t4764 = t3287 * t1661;
    let t4770 = t699 * t1667;
    let t4797 = t1675 * t1128;
    let t4835 = t1687 * t1147;
    let t4861 = t1694 * t3403;
    (t4723, t4728, t4740, t4748, t4764, t4770, t4797, t4835, t4861)
}
