//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 919/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk919<F: Float>(t25154: F, t25155: F, t23125: F, t23134: F, t23141: F, t23144: F, t25140: F, t25142: F, t25144: F, t25147: F, t25149: F, t25151: F) -> F {
    let t25156 = t25154 * t25155;
    let t25158 = F::cast_from(0.20186378047070195427e-3_f64) * t23125 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t25140 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t25142 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t25144 - t25147 / F::cast_from(1536.0_f64) - t25149 / F::cast_from(1536.0_f64) - t25151 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t23134 + t23141 + t23144 + t25156 / F::cast_from(16.0_f64);
    t25158
}
