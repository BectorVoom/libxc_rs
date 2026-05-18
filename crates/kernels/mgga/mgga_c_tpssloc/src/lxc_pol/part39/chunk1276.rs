//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1276/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1276<F: Float>(t16225: F, t3807: F, t16224: F, t12289: F, t242: F, t1336: F, t16048: F, t5248: F, t5249: F, t12283: F, t5259: F, t5293: F) -> (F, F, F, F, F) {
    let t16226 = t16225 * t3807;
    let t16227 = t16224 * t16226;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16235 = t5248 * t5249 * t16048;
    let t16239 = F::new(7.0) / F::new(576.0) * t12283 * t5259;
    let t16241 = F::new(7.0) / F::new(2304.0) * t12283 * t5293;
    (t16227, t16233, t16235, t16239, t16241)
}
