//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 848/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk848<F: Float>(t1550: F, t699: F, t8708: F, t75443: F, t15450: F, t7255: F, t1970: F, t1971: F, t209: F, t2227: F, t515: F, t605: F, t1494: F, t698: F, t75446: F, t75448: F) -> (F, F, F, F, F, F, F) {
    let t77604 = t1550 * t699 * t8708;
    let t77605 = 0.79828278012425390427e-1 * t77604;
    let t77606 = 0.54549323308490683456e-1 * t75443;
    let t77607 = t7255 * t15450;
    let t77608 = 0.42564599893297839398e-5 * t77607;
    let t77613 = t1970 * t1971 * t515 * t2227 * t605 * t209;
    let t77614 = 0.42564599893297839398e-5 * t77613;
    let t77619 = t1970 * t1971 * t515 * t698 * t1494 * t209;
    let t77620 = 0.42564599893297839398e-5 * t77619;
    let t77621 = 0.86737941314158990619e-4 * t75446;
    let t77624 = 0.68186654135613354325e-2 * t75448;
    (t77605, t77606, t77608, t77614, t77620, t77621, t77624)
}
