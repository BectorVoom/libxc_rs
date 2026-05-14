//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 986/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk986<F: Float>(t1206: F, t5371: F, t774: F, t9986: F, t1625: F, t4397: F, t3348: F, t12688: F, t13568: F, t13570: F, t13572: F, t13573: F, t13574: F, t13575: F, t13611: F, t7929: F, t7932: F, t7936: F, t9839: F, t9844: F, t9846: F, t9848: F, t9854: F) -> (F, F, F, F, F) {
    let t13793 = t5371 * t1206;
    let t13795 = t9986 * t774 * t13793;
    let t13798 = t1625 * t4397;
    let t13800 = t3348 * t774 * t13798;
    let t13803 = t13568 + t13570 - t13572 - t12688 - t13573 + t13574 - t9839 + t13575 + t9844 + t9846 - t9848 + t7929 - t7932 - t7936 + t9854 + t13611;
    (t13793, t13795, t13798, t13800, t13803)
}
