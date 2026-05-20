//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2168/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2168<F: Float>(t54555: F, t12289: F, t1336: F, t836: F, t1811: F, t40005: F, t40281: F, t5259: F, t1361: F, t242: F, t12189: F, t5206: F) -> (F, F, F, F, F, F) {
    let t54556 = F::new(119.0) / F::new(4608.0) * t54555;
    let t54566 = t1336 * t12289 * t836;
    let t54582 = t40005 * t1811;
    let t54611 = t40281 * t5259;
    let t54612 = F::new(119.0) / F::new(1152.0) * t54611;
    let t54614 = t1336 * t1361 * t242;
    let t54631 = t12189 * t5206;
    (t54556, t54566, t54582, t54612, t54614, t54631)
}
