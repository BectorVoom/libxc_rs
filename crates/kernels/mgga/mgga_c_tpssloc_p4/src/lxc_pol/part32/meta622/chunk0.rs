//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2028/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2028<F: Float>(t11647: F, t2141: F, t24684: F, t27634: F, t461: F, t607: F, t1009: F, t7324: F, t24658: F, t27635: F, t3540: F, t7334: F) -> (F, F, F, F, F) {
    let t86191 = t2141 * t11647 / F::cast_from(5184.0_f64);
    let t86234 = t27634 * t24684;
    let t86259 = t607 * t461;
    let t86261 = t7324 * t86259 * t1009;
    let t86264 = t24658 * t27635;
    let t86275 = t7334 * t3540;
    (t86191, t86234, t86261, t86264, t86275)
}
