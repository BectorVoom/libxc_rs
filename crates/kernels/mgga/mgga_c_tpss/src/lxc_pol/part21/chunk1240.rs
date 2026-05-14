//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1240/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1240<F: Float>(t19868: F, t19888: F, t219: F, t6168: F, t1705: F, t3987: F, t935: F, t5570: F, t6171: F, t5638: F, t6174: F, t990: F, t18150: F, t1482: F, t5640: F, t5642: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19889 = t19868 + t19888;
    let t19890 = param_beta * t19889;
    let t19892 = t6168 * t219;
    let t19898 = t1705 * t3987;
    let t19899 = t19898 * t935;
    let t19901 = t6171 * t5570;
    let t19904 = t6171 * t5638;
    let t19909 = t6174 * t990;
    let t19910 = t18150 * t19909;
    let t19913 = t5640 * t1482;
    let t19914 = t19913 * t5642;
    (t19889, t19890, t19892, t19898, t19899, t19901, t19904, t19909, t19910, t19913, t19914)
}
