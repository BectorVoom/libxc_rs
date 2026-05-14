//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 942/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk942<F: Float>(t1351: F, t37: F, t177: F, t3590: F, t737: F, t162: F, t8087: F, t3638: F, t8313: F, t236: F, t339: F, t8276: F, t3678: F, t219: F, t3693: F, t220: F, t73: F, t8275: F) -> (F, F, F, F, F, F, F, F) {
    let t10710 = t37 * t1351;
    let t10717 = t3590 * t177;
    let t10719 = 0.11696447245269292414e1 * t10717 * t737;
    let t10728 = t8087 * t162;
    let t10777 = 7.0 / 576.0 * t8313 * t3638;
    let t10779 = t339 * t8276 * t236;
    let t10803 = 7.0 / 576.0 * t8313 * t3678;
    let t10821 = t3693 * t219;
    let t10845 = t220 * t73 * t8275;
    (t10710, t10719, t10728, t10777, t10779, t10803, t10821, t10845)
}
