//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 987/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk987<F: Float>(t13594: F, t13607: F, t162: F, t189: F, t489: F, t5343: F, t724: F, t1206: F, t12688: F, t13568: F, t13570: F, t13572: F, t13573: F, t13574: F, t13575: F, t13576: F, t198: F, t4532: F, t5371: F, t541: F, t7929: F, t7932: F, t7936: F, t7945: F, t9839: F, t9844: F, t9846: F, t9848: F, t9854: F) -> (F, F, F, F) {
    let t13609 = (t13594 + t13607) * t162;
    let t13610 = t13609 * t189;
    let t13611 = t489 * t13610;
    let t13612 = t5343 * t724;
    let t13613 = t489 * t13612;
    let t13614 = F::cast_from(6.0_f64) * t1206 * t198 * t5371 * t541 + F::cast_from(6.0_f64) * t13576 * t4532 - t12688 + t13568 + t13570 - t13572 - t13573 + t13574 + t13575 + t13611 + t13613 + t7929 - t7932 - t7936 + t7945 - t9839 + t9844 + t9846 - t9848 + t9854;
    (t13609, t13611, t13613, t13614)
}
