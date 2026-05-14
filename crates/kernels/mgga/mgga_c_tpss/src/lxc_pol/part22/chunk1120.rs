//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1120/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1120<F: Float>(t5895: F, t645: F, t18434: F, t18461: F, t18437: F, t18440: F, t18442: F, t18447: F, t18451: F, t18455: F, t18457: F, t18459: F, t18465: F, t18467: F, t18469: F, t219: F, t5919: F) -> (F, F, F, F, F, F) {
    let t18930 = t5895 * t645;
    let t18934 = 35.0 / 216.0 * t18434;
    let t18943 = 119.0 / 3456.0 * t18461;
    let t18947 = t18934 + 7.0 / 36.0 * t18437 + t18440 / 8.0 - t18442 / 24.0 + t18447 / 384.0 + 7.0 / 576.0 * t18451 + t18455 / 96.0 - t18457 / 768.0 - t18459 / 768.0 + t18943 + 7.0 / 144.0 * t18465 + 5.0 / 192.0 * t18467 - t18469 / 192.0;
    let t18948 = param_beta * t18947;
    let t18950 = t5919 * t219;
    (t18930, t18934, t18943, t18947, t18948, t18950)
}
