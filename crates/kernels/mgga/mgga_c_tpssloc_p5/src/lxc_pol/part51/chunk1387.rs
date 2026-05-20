//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1387/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1387<F: Float>(t22986: F, t23270: F, t31337: F, t4119: F, t33458: F, t6579: F, t114792: F, t118791: F, t118792: F, t118802: F, t121413: F, t121419: F, t121426: F, t121429: F, t121431: F, t1911: F, t26679: F, t2718: F, t31311: F, t4268: F, t855: F) -> F {
    let t121435 = t22986 * t23270 * t31337 * t4119;
    let t121437 = t6579 * t33458;
    let t121440 = F::cast_from(0.16449340668482264365e-1_f64) * t121413 + F::new(2.0) * t4268 * t31311 - F::cast_from(0.3289868133696452873e-1_f64) * t121419 + F::new(2.0) * t855 * t2718 * t26679 * t1911 + F::cast_from(0.16449340668482264365e-1_f64) * t121426 + F::cast_from(0.16449340668482264365e-1_f64) * t121429 + F::cast_from(0.19190897446562641759e-1_f64) * t121431 + t118791 + t118792 + t118802 + F::cast_from(0.16449340668482264365e-1_f64) * t121435 - F::cast_from(0.38381794893125283518e-1_f64) * t121437 + F::cast_from(0.41123351671205660912e-2_f64) * t114792;
    t121440
}
