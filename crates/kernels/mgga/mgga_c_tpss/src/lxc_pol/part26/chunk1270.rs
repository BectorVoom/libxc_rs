//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1270/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1270<F: Float>(t10292: F, t19191: F, t19345: F, t5975: F, t1860: F, t65157: F, t65165: F, t1299: F, t2016: F, t20718: F, t7682: F, t1981: F, t20767: F, t38: F, t7690: F, t116: F, t20785: F) -> (F, F, F, F, F, F, F, F, F) {
    let t67961 = t10292 * t19191;
    let t68003 = t5975 * t19345;
    let t68006 = t1860 * t65157;
    let t68009 = t1860 * t65165;
    let t68056 = t1299 * t2016;
    let t68115 = t7682 * t20718;
    let t68122 = t1981 * t38 * t20767;
    let t68127 = t7690 * t20718;
    let t68156 = t20785 * t116;
    (t67961, t68003, t68006, t68009, t68056, t68115, t68122, t68127, t68156)
}
