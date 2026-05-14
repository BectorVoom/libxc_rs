//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 938/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk938<F: Float>(t1378: F, t806: F, t246: F, t3664: F, t1388: F, t2157: F, t3692: F, t768: F, t10765: F, t10780: F, t10818: F, t10845: F, t1379: F, t2163: F, t220: F, t229: F, t2365: F, t2370: F, t2415: F, t339: F, t3630: F, t3665: F, t3703: F, t3704: F, t3713: F, t3716: F, t783: F, t813: F, t8330: F, t8372: F) -> (F,) {
    let t10849 = t806 * t1378;
    let t10853 = t246 * t3664;
    let t10880 = t2157 * t1388;
    let t10884 = t768 * t3692;
    let t10894 = -t10765 * t339 * t813 - 6.0 * t10780 * t10845 * t3704 + t10818 * t220 * t229 + 4.0 * t10849 * t3630 * t3703 - 2.0 * t10849 * t3713 * t783 + 4.0 * t10853 * t3630 * t3703 - 2.0 * t10853 * t3713 * t783 + 2.0 * t10880 * t2163 * t339 - 2.0 * t10884 * t339 * t783 - t1379 * t339 * t8372 + 6.0 * t2163 * t3703 * t3704 - t2365 * t339 * t3716 - t2365 * t3704 * t3713 - t2370 * t339 * t3716 - t2370 * t3704 * t3713 - 2.0 * t2415 * t339 * t3665 + 2.0 * t3703 * t3704 * t8330;
    (t10894,)
}
