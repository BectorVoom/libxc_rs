//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1315/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1315<F: Float>(t2587: F, t81151: F, t23172: F, t23150: F, t814: F, t25084: F, t9634: F, t23097: F, t2628: F, t2632: F, t47320: F, t46519: F, t6605: F) -> (F, F, F, F, F, F) {
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81717 = F::cast_from(0.98696044010893586188e-1_f64) * t81716;
    let t81718 = t814 * t23150;
    let t81724 = t25084 * t9634;
    let t81728 = t23097 * t2628 * t47320 * t2632;
    let t81731 = t6605 * t2628 * t46519;
    (t81715, t81717, t81718, t81724, t81728, t81731)
}
