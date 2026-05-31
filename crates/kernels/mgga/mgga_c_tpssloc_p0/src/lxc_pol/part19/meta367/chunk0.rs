//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1344/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1344<F: Float>(t41961: F, t41845: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F, t41882: F, t41885: F, t41973: F) -> F {
    let t43002 = F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t41961;
    let t43012 = -t41845 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t41973 - t43002 - F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t41863 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41865 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41868 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t41870 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t41872 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41874 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t41876 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t41882 + t41885 / F::cast_from(6.0_f64);
    t43012
}
