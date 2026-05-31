//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1033/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1033<F: Float>(t1937: F, t25628: F, t1618: F, t1622: F, t1935: F, t23433: F, t23443: F, t23447: F, t23449: F, t23463: F, t23469: F, t23529: F, t25609: F, t25616: F, t25618: F, t25622: F, t25625: F, t378: F, t6730: F, t7578: F) -> F {
    let t25629 = t25628 * t1937;
    let t25631 = -F::cast_from(0.10093189023535097714e-3_f64) * t6730 * t7578 - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t25609 + F::cast_from(0.10093189023535097714e-3_f64) * t23443 - t23447 - F::cast_from(0.80745512188280781712e-3_f64) * t23449 - t23529 * t1622 / F::cast_from(432.0_f64) + t25616 / F::cast_from(3456.0_f64) + t25618 / F::cast_from(2304.0_f64) + t23433 * t1618 / F::cast_from(1536.0_f64) - t25622 * t378 / F::cast_from(288.0_f64) + t25625 / F::cast_from(2304.0_f64) - t23463 / F::cast_from(108.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t25629 - t23469;
    t25631
}
