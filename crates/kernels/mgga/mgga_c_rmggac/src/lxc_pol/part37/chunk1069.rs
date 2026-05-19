//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1069/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1069<F: Float>(t570: F, t73450: F, t1356: F, t15014: F, t534: F, t71300: F, t72: F, t75002: F, t75003: F, t75005: F, t77357: F, t77361: F, t77362: F, t77363: F, t77365: F, t77367: F, t77370: F, t77372: F, t77374: F, t77376: F, t77379: F) -> (F, F) {
    let t80192 = t73450 * t570;
    let t80197 = t77357 - t77361 + t77362 + t77363 + t77365 + t77367 + t77370 + t77372 + t77374 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t80192 + t72 * t534 * t15014 - t75002 - t71300 - t77376 - t77379 - t75003 + t75005;
    (t80192, t80197)
}
