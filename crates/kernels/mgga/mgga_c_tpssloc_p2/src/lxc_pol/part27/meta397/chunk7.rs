//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1637/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1637<F: Float>(t1714: F, t4899: F, t11571: F, t11545: F, t60: F, t461: F, t14726: F, t11589: F, t4904: F, t3447: F, t11588: F, t4729: F) -> (F, F, F, F) {
    let t15390 = t4899 * t1714;
    let t15391 = t15390 * t11571;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    let t15396 = t15395 * t14726;
    let t15399 = t11589 * t4904;
    let t15401 = F::cast_from(0.18518518518518518518e-3_f64) * t3447 * t15399;
    let t15402 = t11588 * t461;
    let t15403 = t15402 * t4729;
    (t15391, t15396, t15401, t15403)
}
