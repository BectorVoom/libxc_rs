//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1125/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1125<F: Float>(t3540: F, t6158: F, t15730: F, t5002: F, t15734: F, t5024: F, t11818: F, t248: F, t3515: F, t6230: F, t11789: F, t1227: F, t5979: F, t6165: F, t5975: F, t15437: F, t15502: F) -> (F, F, F, F, F, F, F, F) {
    let t65600 = t6158 * t3540;
    let t65605 = t5002 * t15730;
    let t65628 = t5024 * t15734;
    let t65632 = t3515 * t248 * t11818 * t6230;
    let t65647 = t1227 * t248 * t11789 * t5979;
    let t65664 = t6165 * t3540;
    let t65689 = t1227 * t248 * t11789 * t5975;
    let t65703 = t15437 * t15502;
    (t65600, t65605, t65628, t65632, t65647, t65664, t65689, t65703)
}
