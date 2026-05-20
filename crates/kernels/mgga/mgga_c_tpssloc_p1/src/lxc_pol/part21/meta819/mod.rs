//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta819 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2883;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta819<F: Float>(t136: F, t2826: F, t59668: F, t59672: F, t10304: F, t59725: F, t59755: F, t59746: F, t908: F, t4370: F, t896: F, t13634: F, t13637: F, t41959: F, t41962: F, t59680: F, t59684: F, t59688: F, t59692: F, t59694: F) -> (F, F, F, F, F, F, F, F) {
        let (t60223, t60226, t60229, t60232, t60235, t60237, t60238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2883::<F>(t136, t2826, t59668, t59672, t10304, t59725, t59755, t59746, t908, t4370, t896, t13634);
        let (t60240, t60242) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2884::<F>(t13637, t60237, t41959, t41962, t59680, t59684, t59688, t59692, t59694, t60223, t60226, t60229, t60232, t60235, t60238);
    (t60223, t60226, t60229, t60232, t60235, t60238, t60240, t60242)
}
