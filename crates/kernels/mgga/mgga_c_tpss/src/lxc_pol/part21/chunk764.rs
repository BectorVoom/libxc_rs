//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 764/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk764<F: Float>(t1474: F, t948: F, t1477: F, t220: F, t2782: F, t2798: F, t368: F, t3987: F, t3997: F, t4001: F, t4004: F, t4008: F, t983: F, t985: F) -> (F,) {
    let t4011 = t1474 * t948;
    let t4016 = 2.0 * t1477 * t2782 * t3997 - t1477 * t2798 * t4008 + t220 * t368 * t3987 + t4001 * t983 * t985 + t4004 * t983 * t985 + t4011 * t983 * t985;
    (t4016,)
}
