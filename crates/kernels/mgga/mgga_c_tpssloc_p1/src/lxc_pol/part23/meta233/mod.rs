//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk883;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta233<F: Float>(t118: F, t5544: F, t794: F, t2576: F, t2563: F, t5555: F, t252: F, t5584: F, t1499: F, t4290: F, t4166: F, t4177: F, t120: F) -> (F, F, F, F, F, F, F) {
        let (t16791, t16792, t16794, t16815, t16830, t16836) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk883::<F>(t118, t5544, t794, t2576, t2563, t5555, t252, t5584, t1499, t4290, t4166, t4177);
        let t16839 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk884::<F>(t120, t5584);
    (t16791, t16792, t16794, t16815, t16830, t16836, t16839)
}
