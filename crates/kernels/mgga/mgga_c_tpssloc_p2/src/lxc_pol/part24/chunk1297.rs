//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1297/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1297<F: Float>(t22574: F, t26162: F, t55183: F, t6535: F, t9348: F, t12734: F, t12823: F, t107: F, t835: F, t240: F, t656: F, t666: F) -> (F, F, F, F, F, F) {
    let t81426 = F::cast_from(18.0_f64) * t22574 * t26162 * t55183;
    let t81430 = F::cast_from(6.0_f64) * t9348 * t6535;
    let t81432 = F::cast_from(12.0_f64) * t12734 * t6535;
    let t81434 = F::cast_from(6.0_f64) * t12823 * t6535;
    let t81437 = t835 * t107;
    let t81438 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t81437;
    let t81439 = t240 * t656;
    let t81440 = t81439 * t666;
    (t81426, t81430, t81432, t81434, t81438, t81440)
}
