//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1272/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1272<F: Float>(t82218: F, t10109: F, t225: F, t1914: F, t40772: F, t3034: F, t336: F, t221: F, t697: F, t1016: F, t835: F, t39063: F, t7245: F) -> (F, F, F, F, F, F, F, F) {
    let t82219 = F::new(0.27720185200590482541e0) * t82218;
    let t82252 = t225 * t10109;
    let t82312 = t1914 * t40772;
    let t82510 = F::new(1.0) / t3034 / t336;
    let t82631 = t221 * t697;
    let t82985 = F::new(1.0) / t3034 / t1016;
    let t83803 = F::new(1232.0) / F::new(27.0) * t835;
    let t85501 = t39063 * t7245;
    (t82219, t82252, t82312, t82510, t82631, t82985, t83803, t85501)
}
