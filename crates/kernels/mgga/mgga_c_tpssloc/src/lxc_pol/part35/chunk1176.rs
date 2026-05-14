//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1176/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1176<F: Float>(t2121: F, t3427: F, t8077: F, t24771: F, t7999: F, t24585: F, t11947: F, t8090: F, t27331: F, t9239: F, t45844: F, t7245: F, t1419: F, t2274: F, t111: F, t8110: F) -> (F, F, F, F, F, F, F, F) {
    let t95726 = t2121 * t3427 * t8077;
    let t95768 = t7999 * t24771;
    let t95824 = t7999 * t24585;
    let t95925 = t8090 * t11947;
    let t96045 = t9239 * t27331;
    let t96120 = t45844 * t7245;
    let t96157 = t1419 * t2274;
    let t96334 = t8110 * t111;
    (t95726, t95768, t95824, t95925, t96045, t96120, t96157, t96334)
}
