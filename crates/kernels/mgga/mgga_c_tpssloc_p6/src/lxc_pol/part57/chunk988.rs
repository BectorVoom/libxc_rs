//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 988/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk988<F: Float>(t1888: F, t23270: F, t31332: F, t5657: F, t101593: F, t114815: F, t121431: F, t121437: F, t121444: F, t121464: F, t121469: F, t126363: F, t126368: F, t1911: F, t1912: F, t2054: F, t26700: F, t2718: F, t28307: F, t29055: F, t29056: F, t33405: F, t6627: F, t7087: F, t7538: F, t855: F, t98239: F, t98975: F, t99010: F) -> F {
    let t127874 = t1888 * t23270 * t31332 * t5657;
    let t127883 = F::cast_from(0.38381794893125283518e-1_f64) * t121431 - F::cast_from(0.76763589786250567036e-1_f64) * t121437 - F::cast_from(0.16449340668482264365e-1_f64) * t121444 - t114815 - F::cast_from(12.0_f64) * t98975 * t33405 - F::cast_from(2.0_f64) * t26700 * t7538 + F::cast_from(0.16449340668482264365e-1_f64) * t121464 + F::cast_from(2.0_f64) * t855 * t2718 * t29055 * t1911 + F::cast_from(0.76763589786250567036e-1_f64) * t121469 + F::cast_from(0.16449340668482264365e-1_f64) * t127874 - t99010 * t2054 + F::cast_from(4.0_f64) * t7087 * t28307 - t126363 - t6627 * t29056 + t126368 - F::cast_from(2.0_f64) * t98239 * t2054 - t101593 * t1912;
    t127883
}
