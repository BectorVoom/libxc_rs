//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1170/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1170<F: Float>(t5640: F, t990: F, t5642: F, t5623: F, t5632: F, t1726: F, t2804: F, t8547: F, t8549: F, t980: F, t1729: F) -> (F, F, F, F, F, F) {
    let t18157 = t5640 * t990;
    let t18158 = t18157 * t5642;
    let t18162 = t5632 * t5623 * t990;
    let t18166 = t5632 * t1726 * t2804;
    let t18169 = t8547 * t8549;
    let t18170 = t18169 * t980;
    let t18171 = t1729 * t18170;
    (t18158, t18162, t18166, t18169, t18170, t18171)
}
