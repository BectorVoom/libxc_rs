//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2284/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2284<F: Float>(t47185: F, t47149: F, t47151: F, t47153: F, t47156: F, t47159: F, t47161: F, t47162: F, t47164: F, t47166: F, t47168: F, t47171: F, t47174: F, t47175: F, t47178: F, t47181: F, t47183: F) -> (F, F) {
    let t47186 = F::cast_from(36.0_f64) * t47185;
    let t47187 = t47149 + t47151 + t47153 + t47156 + t47159 + t47161 + t47162 + t47164 + t47166 + t47168 + t47171 + t47174 + t47175 + t47178 + t47181 + t47183 + t47186;
    (t47186, t47187)
}
