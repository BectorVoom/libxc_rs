//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1505/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1505<F: Float>(t1213: F, t15730: F, t11789: F, t1653: F, t248: F, t1227: F, t15437: F, t3505: F) -> (F, F, F, F) {
    let t15731 = t1213 * t15730;
    let t15734 = t248 * t11789 * t1653;
    let t15735 = t1227 * t15734;
    let t15737 = t15437 * t3505;
    (t15731, t15734, t15735, t15737)
}
