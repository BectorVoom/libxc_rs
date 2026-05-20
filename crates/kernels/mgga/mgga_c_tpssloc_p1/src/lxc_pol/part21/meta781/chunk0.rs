//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2711/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2711<F: Float>(t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t39360: F, t56140: F, t56141: F, t56147: F, t56149: F, t56150: F, t56151: F, t56152: F, t56159: F, t56160: F) -> F {
    let t57194 = t39316 + t39320 - t39324 - t56140 + t56141 - t56147 + t39327 + t56149 + t56150 - t39338 + t56151 - t56152 + t39346 + t39349 - t56159 + t39356 - t56160 + t39360;
    t57194
}
