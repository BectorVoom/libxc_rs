//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2341/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2341<F: Float>(t225: F, t29665: F, t8006: F, t94490: F, t11606: F, t1190: F, t1238: F, t1252: F, t15797: F, t15820: F, t1716: F, t1720: F, t19208: F, t19213: F, t19219: F, t24615: F, t27721: F, t27784: F, t27785: F, t29536: F, t29664: F, t3593: F, t498: F, t6243: F, t7283: F, t7300: F, t7301: F, t7391: F, t8014: F, t8061: F, t8088: F, t86501: F, t94391: F, t94558: F, t95912: F) -> F {
    let t104635 = t29665 * t225;
    let t104647 = t94490 * t8006;
    let t104669 = F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t94391 - t104635 * t1252 + t1190 * t29664 * t498 - F::new(2.0) * t15797 * t8088 - F::new(12.0) * t27784 * t27785 * t19213 + F::new(2.0) * t3593 * t29536 - F::cast_from(0.18277045187202515961e-2_f64) * t86501 + F::cast_from(0.14621636149762012769e-1_f64) * t104647 + F::new(2.0) * t1720 * t27721 * t498 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t94558 * t8014 - F::new(6.0) * t1238 * t11606 * t7391 * t6243 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t7300 * t24615 * t19219 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t7300 * t7301 * t19208 - t95912 + F::new(4.0) * t15820 * t8061;
    t104669
}
