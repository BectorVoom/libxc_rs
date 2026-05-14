//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 986/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk986<F: Float>(t33: F, t5335: F, t9868: F, t3289: F, t5059: F, t13334: F, t1006: F, t1989: F, t4368: F, t493: F, t13594: F, t162: F, t189: F, t489: F, t5343: F, t724: F, t1206: F, t12688: F, t13568: F, t13570: F, t13572: F, t13573: F, t13574: F, t13575: F, t13576: F, t198: F, t4532: F, t5371: F, t541: F, t7929: F, t7932: F, t7936: F, t7945: F, t9839: F, t9844: F, t9846: F, t9848: F, t9854: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t13595 = t9868 * t5335;
    let t13600 = t3289 * t5059;
    let t13603 = -t13334;
    let t13607 = piecewise3(t34, 0.0, -8.0 / 27.0 * t13595 * t1006 - 16.0 / 9.0 * t4368 * t1989 + 4.0 / 9.0 * t13600 * t1006 + 4.0 / 3.0 * t493 * t13603);
    let t13609 = (t13594 + t13607) * t162;
    let t13610 = t13609 * t189;
    let t13611 = t489 * t13610;
    let t13612 = t5343 * t724;
    let t13613 = t489 * t13612;
    let t13614 = 6.0 * t1206 * t198 * t5371 * t541 + 6.0 * t13576 * t4532 - t12688 + t13568 + t13570 - t13572 - t13573 + t13574 + t13575 + t13611 + t13613 + t7929 - t7932 - t7936 + t7945 - t9839 + t9844 + t9846 - t9848 + t9854;
    (t13603, t13609, t13611, t13613, t13614)
}
