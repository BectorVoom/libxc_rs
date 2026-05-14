//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 828/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk828<F: Float>(t77613: F, t1494: F, t1970: F, t1971: F, t209: F, t515: F, t698: F, t75446: F, t75448: F, t75450: F, t75452: F, t75465: F, t75469: F, t75473: F, t75477: F, t75480: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t77614 = 0.42564599893297839398e-5 * t77613;
    let t77619 = t1970 * t1971 * t515 * t698 * t1494 * t209;
    let t77620 = 0.42564599893297839398e-5 * t77619;
    let t77621 = 0.86737941314158990619e-4 * t75446;
    let t77624 = 0.68186654135613354325e-2 * t75448;
    let t77625 = 0.20455996240684006296e-1 * t75450;
    let t77626 = 0.40911992481368012592e-1 * t75452;
    let t77630 = 0.2553875993597870364e-4 * t75465;
    let t77631 = 0.5107751987195740728e-4 * t75469;
    let t77633 = 0.7661627980793611092e-4 * t75473;
    let t77634 = 0.5107751987195740728e-4 * t75477;
    let t77635 = 0.2553875993597870364e-4 * t75480;
    (t77614, t77620, t77621, t77624, t77625, t77626, t77630, t77631, t77633, t77634, t77635)
}
