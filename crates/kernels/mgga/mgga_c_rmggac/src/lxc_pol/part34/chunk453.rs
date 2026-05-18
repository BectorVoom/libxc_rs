//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 453/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk453<F: Float>(t1664: F, t702: F, t1614: F, t699: F, t2471: F, t333: F, t321: F, t570: F, t8264: F, t2228: F, t551: F, t8710: F) -> (F, F, F, F, F, F, F) {
    let t9343 = t1664 * t702;
    let t9352 = t699 * t1614;
    let t9370 = t2471 * t333;
    let t9383 = t2471 * t321;
    let t9427 = t8264 * t570;
    let t9437 = t2228 * t551;
    let t9445 = F::new(0.4838420607177634088e-3) * t8710;
    (t9343, t9352, t9370, t9383, t9427, t9437, t9445)
}
