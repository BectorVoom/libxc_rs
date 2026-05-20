//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2635/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2635<F: Float>(t12364: F, t5234: F, t1354: F, t16288: F, t3858: F, t12365: F, t5289: F, t1827: F, t39955: F, t16261: F, t16398: F, t12289: F, t1336: F, t836: F) -> (F, F, F, F, F, F, F) {
    let t54532 = t5234 * t12364;
    let t54533 = t54532 * t1354;
    let t54535 = t16288 * t3858;
    let t54555 = t12365 * t5289;
    let t54557 = t39955 * t1827;
    let t54561 = t16398 * t16261;
    let t54566 = t1336 * t12289 * t836;
    (t54532, t54533, t54535, t54555, t54557, t54561, t54566)
}
