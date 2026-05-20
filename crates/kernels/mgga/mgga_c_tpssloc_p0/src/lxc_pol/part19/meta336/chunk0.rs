//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1200/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1200<F: Float>(t39494: F, t761: F, t152: F, t185: F, t39097: F, t153: F, t157: F, t39842: F, t10140: F, t10143: F, t2374: F, t39354: F) -> (F, F, F, F, F) {
    let t40779 = F::cast_from(0.51947577317044391277e2_f64) * t761 * t39494;
    let t40782 = F::new(24.0) * t39097 * t152 * t185;
    let t40784 = t153 * t157 * t39842;
    let t40785 = t10140 * t10143;
    let t40790 = F::cast_from(0.21687162600603479684e-1_f64) * t2374 * t39354;
    (t40779, t40782, t40784, t40785, t40790)
}
