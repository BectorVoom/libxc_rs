//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1887/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1887<F: Float>(t13953: F, t14004: F, t14050: F, t14074: F, t14120: F, t14170: F, t14233: F, t14523: F, t349: F, t225: F, t4658: F, t1625: F, t3020: F) -> (F, F, F, F) {
    let t14526 = t13953 + t14004 + t14050 + t14074 + t14120 + t14170 + t14233 + t14523;
    let t14527 = t349 * t14526;
    let t14529 = t4658 * t225;
    let t14532 = t3020 * t1625;
    (t14526, t14527, t14529, t14532)
}
