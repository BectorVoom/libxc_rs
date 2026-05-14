//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1114/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1114<F: Float>(t12521: F, t12524: F, t12813: F, t1401: F, t1458: F, t16506: F, t16521: F, t16524: F, t16535: F, t16538: F, t16541: F, t2319: F, t2363: F, t3938: F, t3941: F, t4072: F, t5371: F, t5376: F, t577: F, t671: F) -> (F,) {
    let t16546 = 0.45e1 * t16506 * t577 + 27.0 * t16521 * t671 + 27.0 * t16524 * t2319 + 0.135e2 * t5371 * t2363 + 0.135e2 * t12521 * t1458 + 54.0 * t12524 * t5376 + 27.0 * t3938 * t4072 + 27.0 * t16535 * t1458 + 54.0 * t3941 * t16538 + 27.0 * t3941 * t16541 + 0.135e2 * t1401 * t12813;
    (t16546,)
}
