//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2407/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2407<F: Float>(t13515: F, t2837: F, t2841: F, t4351: F, t2845: F, t10697: F, t4354: F, t10701: F, t1543: F, t10705: F, t1557: F, t41618: F) -> (F, F, F, F, F) {
    let t49268 = F::new(3.0) * t13515 * t2837;
    let t49269 = t4351 * t2841;
    let t49271 = F::cast_from(0.48245938496077605201e2_f64) * t49269 * t2845;
    let t49273 = F::new(1.0) * t4354 * t10697;
    let t49274 = t1543 * t10701;
    let t49276 = F::cast_from(0.51726012919273400301e3_f64) * t49274 * t10705;
    let t49278 = F::new(1.0) * t41618 * t1557;
    (t49268, t49271, t49273, t49276, t49278)
}
