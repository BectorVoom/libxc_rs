//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 925/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk925<F: Float>(t1587: F, t3282: F, t75876: F, t75881: F, t15907: F, t504: F, t70048: F, t70050: F, t71661: F, t739: F, t75853: F, t75869: F, t75874: F, t78322: F, t78324: F, t78327: F, t78339: F, t78340: F, t78341: F, t78349: F) -> (F, F) {
    let t80341 = t3282 * t1587;
    let t80344 = 0.65053455985619242964e-5 * t75876;
    let t80345 = 0.65053455985619242964e-5 * t75881;
    let t80346 = t75853 - t78322 - t78324 + t78327 - 0.57000320883372412499e-7 * t70048 - 0.57000320883372412499e-7 * t70050 + t71661 - t78339 + t78340 + t78341 + 0.76860658247009135562e-5 * t75869 + t75874 - 0.19957069503106347607e-1 * t504 * t15907 - 0.59871208509319042821e-1 * t739 * t80341 + t80344 + t80345 - t78349;
    (t80341, t80346)
}
