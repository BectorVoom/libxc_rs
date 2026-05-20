//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2276/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2276<F: Float>(t11153: F, t497: F, t27654: F, t491: F, t1235: F, t8034: F, t27434: F, t85639: F, t27821: F, t24600: F, t7301: F, t27798: F, t4935: F) -> (F, F, F, F, F, F, F) {
    let t94349 = t497 * t11153;
    let t94354 = t27654 * t491;
    let t94358 = t8034 * t1235;
    let t94363 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27434;
    let t94365 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27821;
    let t94369 = t24600 * t7301;
    let t94374 = t4935 * t27798;
    (t94349, t94354, t94358, t94363, t94365, t94369, t94374)
}
