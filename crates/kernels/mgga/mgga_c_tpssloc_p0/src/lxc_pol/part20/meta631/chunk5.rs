//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2301/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2301<F: Float>(t193: F, t2379: F, t1484: F, t2522: F, t40622: F, t4320: F, t47166: F, t47168: F, t47171: F, t47174: F, t47175: F, t47178: F, t47181: F, t47183: F, t47186: F) -> F {
    let t47645 = t193 * t2379;
    let t47651 = F::cast_from(3.0_f64) * t1484 * t2522 * t40622 + F::cast_from(18.0_f64) * t4320 * t47645 + t47166 + t47168 + t47171 + t47174 + t47175 + t47178 + t47181 + t47183 + t47186;
    t47651
}
