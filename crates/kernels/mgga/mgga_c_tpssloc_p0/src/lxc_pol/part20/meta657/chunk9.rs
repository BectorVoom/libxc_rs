//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2437/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2437<F: Float>(t1041: F, t4589: F, t49850: F, t10969: F, t41687: F, t42600: F, t42721: F, t42729: F, t42731: F, t4582: F, t4583: F, t4588: F, t45993: F, t4600: F, t46006: F, t48497: F, t49827: F, t49829: F, t49832: F, t49846: F) -> F {
    let t49852 = t1041 * t49850 * t4589;
    let t49853 = F::new(5.0) / F::new(20736.0) * t49852;
    let t49854 = t10969 * t41687;
    let t49860 = -t42721 / F::new(2304.0) + t42729 / F::new(2304.0) + F::new(19.0) / F::new(1296.0) * t49827 - t49829 / F::new(216.0) + t49832 - t1041 * t4582 * t4583 * t46006 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t1041 * t4582 * t4588 * t46006 - t1041 * t4582 * t4583 * t45993 / F::new(2304.0) - F::new(5.0) / F::new(1152.0) * t49846 - F::new(19.0) / F::new(576.0) * t42600 * t4600 - t49853 - F::new(5.0) / F::new(432.0) * t1041 * t4582 * t49854 * t48497 + t42731 / F::new(288.0);
    t49860
}
