//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1262/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1262<F: Float>(t63928: F, t1381: F, t61050: F, t1369: F, t61062: F, t17974: F, t3689: F, t1385: F, t61086: F, t17946: F, t3622: F, t17960: F, t3667: F, t3685: F, t19695: F, t19697: F, t5543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63929 = 7.0 / 1152.0 * t63928;
    let t63945 = t61050 * t1381;
    let t63957 = t61062 * t1369;
    let t63960 = t17974 * t3689;
    let t63961 = 7.0 / 288.0 * t63960;
    let t63964 = t61086 * t1385;
    let t63966 = t17946 * t3622;
    let t63967 = 7.0 / 72.0 * t63966;
    let t63973 = t17960 * t3667;
    let t63974 = 7.0 / 1152.0 * t63973;
    let t63977 = t17974 * t3685;
    let t63978 = 35.0 / 288.0 * t63977;
    let t63990 = t5543 * t19695 * t19697;
    (t63929, t63945, t63957, t63961, t63964, t63967, t63974, t63978, t63990)
}
