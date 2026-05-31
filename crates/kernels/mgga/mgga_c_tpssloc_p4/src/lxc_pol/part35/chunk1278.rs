//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1278/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1278<F: Float>(t11713: F, t11717: F, t24727: F, t7337: F, t11647: F, t2141: F, t10469: F, t11715: F, t478: F, t3502: F, t24658: F, t27635: F) -> (F, F, F, F, F, F) {
    let t86164 = t11713 * t24727 * t11717;
    let t86171 = t11713 * t7337 * t11717;
    let t86191 = t2141 * t11647 / F::cast_from(5184.0_f64);
    let t86208 = t10469 * t11715 * t478;
    let t86214 = t10469 * t3502 * t478;
    let t86264 = t24658 * t27635;
    (t86164, t86171, t86191, t86208, t86214, t86264)
}
