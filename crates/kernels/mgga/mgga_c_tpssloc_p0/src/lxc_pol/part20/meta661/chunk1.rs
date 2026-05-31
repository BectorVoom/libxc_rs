//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2477/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2477<F: Float>(t25: F, t10150: F, t1074: F, t11105: F, t12606: F, t13493: F, t1408: F, t1409: F, t14675: F, t1534: F, t1642: F, t2249: F, t2250: F, t3220: F, t396: F, t3966: F, t40: F, t4324: F, t45872: F, t4705: F, t47655: F, t47668: F, t47670: F, t47672: F, t47674: F, t47676: F, t50785: F, t606: F, t607: F, t9257: F, t9258: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t50803 = piecewise3::<F>(t115, t47655 * t25 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t13493 * t606 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4324 * t2249 + t1534 * t9257 / F::cast_from(2.0_f64) + t10150 * t1408 / F::cast_from(2.0_f64) + t47668 + t47670 - t47672 - t47674 + t47676, t50785 * t40 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t14675 * t607 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4705 * t2250 + t1642 * t9258 / F::cast_from(2.0_f64) + t11105 * t1409 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3220 * t3966 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1074 * t12606 + t396 * t45872 / F::cast_from(2.0_f64));
    t50803
}
