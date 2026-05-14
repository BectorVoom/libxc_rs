//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1122/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1122<F: Float>(t20177: F, t20216: F, t509: F, t1270: F, t13965: F, t18690: F, t4525: F, t5936: F, t508: F, t6435: F, t5709: F, t10292: F, t5784: F, t5791: F, t6080: F, t18670: F, t6077: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20217 = t20177 + t20216;
    let t20218 = t509 * t20217;
    let t20219 = t20218 * t1270;
    let t20221 = t18690 * t13965;
    let t20224 = t5936 * t4525;
    let t20226 = t508 * t6435;
    let t20227 = t20226 * t5709;
    let t20246 = t10292 * t5784;
    let t20255 = t6080 * t5791;
    let t20257 = t18670 * t6077;
    (t20217, t20218, t20219, t20221, t20224, t20226, t20227, t20246, t20255, t20257)
}
