//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 960/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk960<F: Float>(t1285: F, t2221: F, t1287: F, t522: F, t9216: F, t9218: F, t1294: F, t9713: F, t25: F, t526: F, t28: F, t528: F) -> (F, F, F, F, F, F, F) {
    let t12050 = t2221 * t1285;
    let t12052 = t2221 * t1287;
    let t12054 = t9216 * t522;
    let t12057 = F::new(120.0) * t9218 * t522;
    let t12059 = F::new(0.5848223622634646207e0) * t1294 * t9713;
    let t12061 = F::new(1.0) / t526 / t25;
    let t12072 = F::new(1.0) / t528 / t28;
    (t12050, t12052, t12054, t12057, t12059, t12061, t12072)
}
