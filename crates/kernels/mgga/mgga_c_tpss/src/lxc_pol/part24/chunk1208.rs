//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1208/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1208<F: Float>(t1760: F, t21018: F, t1683: F, t4637: F, t13627: F, t1778: F, t1270: F, t5366: F, t5708: F, t6243: F, t6275: F, t18439: F, t5373: F, t5377: F, t5716: F, t18446: F, t5383: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21020 = 6.0 * t1760 * t21018;
    let t21021 = t1683 * t4637;
    let t21024 = t1778 * t13627;
    let t21026 = 2.0 * t1760 * t21024;
    let t21027 = t1270 * t5366;
    let t21028 = t5708 * t21027;
    let t21030 = 3.0 * t1760 * t21028;
    let t21035 = 2.0 * t6243 * t6275;
    let t21036 = t18439 * t5373;
    let t21038 = t5716 * t5377;
    let t21040 = t18446 * t5383;
    (t21020, t21021, t21024, t21026, t21027, t21028, t21030, t21035, t21036, t21038, t21040)
}
