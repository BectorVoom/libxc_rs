//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1123/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1123<F: Float>(t3640: F, t5091: F, t3415: F, t4869: F, t1654: F, t2394: F) -> (F, F, F) {
    let t14696 = t5091 * t3640;
    let t14701 = F::cast_from(0.11696447245269292414e1_f64) * t4869 * t3415;
    let t14702 = t2394 * t1654;
    (t14696, t14701, t14702)
}
