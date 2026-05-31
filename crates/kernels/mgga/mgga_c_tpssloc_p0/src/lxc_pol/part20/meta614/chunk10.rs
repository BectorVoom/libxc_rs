//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2214/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2214<F: Float>(t12620: F, t12630: F, t1427: F, t1434: F, t2244: F, t2245: F, t2284: F, t2304: F, t33: F, t3997: F, t3998: F, t4018: F, t45892: F, t45931: F, t45977: F, t629: F, t642: F, t66: F, t72: F, t80: F, t9251: F, t9313: F, t9339: F) -> F {
    let t45986 = t2284 * t4018 / F::cast_from(8.0_f64) + t629 * t12620 / F::cast_from(8.0_f64) + t66 * t72 * t45892 / F::cast_from(24.0_f64) + t9313 * t1434 / F::cast_from(24.0_f64) - t9251 * t1434 / F::cast_from(4.0_f64) - t2245 * t4018 / F::cast_from(4.0_f64) - t2244 * t3997 * t80 / F::cast_from(4.0_f64) - t12630 * t642 / F::cast_from(4.0_f64) + t33 * (t45931 + t45977) * t80 / F::cast_from(24.0_f64) + t3998 * t2304 / F::cast_from(8.0_f64) + t1427 * t9339 / F::cast_from(24.0_f64);
    t45986
}
