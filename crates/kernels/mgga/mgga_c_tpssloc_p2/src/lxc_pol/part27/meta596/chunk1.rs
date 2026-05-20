//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2060/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2060<F: Float>(t23165: F, t82038: F, t1879: F, t80845: F, t1906: F, t23229: F, t81715: F, t225: F, t23226: F, t23228: F) -> (F, F, F, F, F, F) {
    let t82039 = t82038 * t23165;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    let t82047 = F::cast_from(0.27720185200590482541e0_f64) * t82046;
    let t82069 = t81715 * t23229;
    let t82070 = F::cast_from(0.98696044010893586188e-1_f64) * t82069;
    let t82071 = t23226 * t225;
    let t82074 = t23228 * t225;
    (t82039, t82045, t82047, t82070, t82071, t82074)
}
