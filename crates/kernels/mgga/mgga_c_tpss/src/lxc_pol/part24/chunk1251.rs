//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1251/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1251<F: Float>(t63960: F, t1385: F, t61086: F, t17946: F, t3622: F, t17960: F, t3667: F, t17974: F, t3685: F, t19695: F, t19697: F, t5543: F, t136: F, t1693: F, t799: F, t19725: F, t219: F) -> (F, F, F, F, F, F, F, F) {
    let t63961 = 7.0 / 288.0 * t63960;
    let t63964 = t61086 * t1385;
    let t63966 = t17946 * t3622;
    let t63967 = 7.0 / 72.0 * t63966;
    let t63973 = t17960 * t3667;
    let t63974 = 7.0 / 1152.0 * t63973;
    let t63977 = t17974 * t3685;
    let t63978 = 35.0 / 288.0 * t63977;
    let t63990 = t5543 * t19695 * t19697;
    let t63991 = 7.0 / 24.0 * t63990;
    let t63993 = t1693 * t799 * t136;
    let t64016 = t19725 * t219;
    (t63961, t63964, t63967, t63974, t63978, t63991, t63993, t64016)
}
