//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1086/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1086<F: Float>(t16100: F, t205: F, t1345: F, t68: F, t12418: F, t820: F, t12289: F, t242: F, t1336: F, t3804: F, t3788: F, t836: F) -> (F, F, F, F, F, F) {
    let t16101 = t205 * t16100;
    let t16186 = t1345 * t68;
    let t16224 = t12418 * t820;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16305 = t3804 * t820;
    let t16397 = t3788 * t836;
    (t16101, t16186, t16224, t16233, t16305, t16397)
}
