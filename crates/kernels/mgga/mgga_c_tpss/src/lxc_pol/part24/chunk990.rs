//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 990/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk990<F: Float>(t13803: F, t13804: F, t13809: F, t13818: F, t219: F, t1634: F, t73: F, t3346: F, t5371: F, t1206: F, t4397: F, t4452: F, t1246: F, t5366: F, t1228: F, t13671: F) -> (F, F, F, F, F, F) {
    let t13821 = (t13803 + t13804 + t13809 + t13818) * t219;
    let t13827 = t1634 * t73;
    let t13834 = t3346 * t5371;
    let t13835 = t13834 * t1206;
    let t13838 = t4452 * t4397;
    let t13843 = t1246 * t5366;
    let t13844 = t13843 * t1206;
    let t13847 = t1228 * t13671;
    (t13821, t13827, t13835, t13838, t13844, t13847)
}
