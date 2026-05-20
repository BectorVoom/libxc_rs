//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 824/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk824<F: Float>(t3062: F, t820: F, t10402: F, t3200: F, t3051: F, t121: F, t3061: F, t1008: F) -> (F, F, F, F, F) {
    let t10408 = t820 * t3062;
    let t10413 = t3200 * t10402;
    let t10422 = t820 * t3051;
    let t10457 = t121 * t3061;
    let t10468 = t1008 * t1008;
    let t10469 = F::new(1.0) / t10468;
    (t10408, t10413, t10422, t10457, t10469)
}
