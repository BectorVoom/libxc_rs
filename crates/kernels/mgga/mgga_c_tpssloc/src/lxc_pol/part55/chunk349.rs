//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 349/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk349<F: Float>(t1568: F, t932: F, t1541: F, t936: F, t324: F, t1548: F, t1551: F, t1554: F, t945: F, t948: F) -> (F, F, F, F) {
    let t1569 = t1568 * t932;
    let t1573 = -t936 - 0.92708333333333333333e-2 * t1541;
    let t1574 = t1573 * t324;
    let t1580 = 0.258925e1 * t1548 - t945 - 0.301925e0 * t1541 + 0.16504875e0 * t1551 - t948 - 0.82785e-1 * t1554;
    (t1569, t1573, t1574, t1580)
}
