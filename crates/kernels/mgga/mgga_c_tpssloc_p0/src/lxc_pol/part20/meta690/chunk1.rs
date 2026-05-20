//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2619/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619<F: Float>(t11797: F, t5005: F, t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t11825: F, t45167: F, t45169: F, t45171: F, t45178: F, t45181: F, t45184: F, t4974: F) -> F {
    let t53267 = t5005 * t11797;
    let t53270 = t1174 * t698 * t5045;
    let t53271 = t53270 / F::new(432.0);
    let t53272 = t4966 * t3540;
    let t53273 = t53272 / F::new(4608.0);
    let t53274 = t1744 * t11647;
    let t53276 = t45167 / F::new(1536.0) + t45169 / F::new(768.0) - t45171 / F::new(1536.0) + t45178 / F::new(216.0) - t45181 / F::new(864.0) - t45184 / F::new(144.0) - t11825 * t4974 / F::new(768.0) - t53267 / F::new(2304.0) + t53271 - t53273 - t53274 / F::new(1944.0);
    t53276
}
