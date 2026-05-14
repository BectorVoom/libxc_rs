//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1202/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1202<F: Float>(t5: F, t20741: F, t20783: F, t117: F, t1163: F, t1322: F, t1339: F, t1600: F, t1865: F, t1897: F, t19261: F, t19304: F, t19307: F, t19310: F, t19312: F, t19322: F, t20706: F, t3491: F, t3502: F, t4341: F, t485: F, t5984: F, t5986: F, t6054: F, t6480: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t20785 = piecewise3(t8, 0.0, t20741 + t20783);
    let t20786 = t20785 * t117;
    let t20789 = -t1163 * t6480 - t1322 * t6054 - 2.0 * t1339 * t19261 - 2.0 * t1339 * t20706 - t1600 * t5984 - t1865 * t4341 - t1897 * t3491 - t20786 * t485 - 2.0 * t3502 * t5986 - t19304 - t19307 - t19310 - t19312 - t19322;
    (t20785, t20786, t20789)
}
