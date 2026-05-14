//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1232/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1232<F: Float>(t114792: F, t118791: F, t118792: F, t118802: F, t121413: F, t121419: F, t121426: F, t121429: F, t121431: F, t121435: F, t121437: F, t1911: F, t26679: F, t2718: F, t31311: F, t4268: F, t855: F) -> (F,) {
    let t121440 = 0.16449340668482264365e-1 * t121413 + 2.0 * t4268 * t31311 - 0.3289868133696452873e-1 * t121419 + 2.0 * t855 * t2718 * t26679 * t1911 + 0.16449340668482264365e-1 * t121426 + 0.16449340668482264365e-1 * t121429 + 0.19190897446562641759e-1 * t121431 + t118791 + t118792 + t118802 + 0.16449340668482264365e-1 * t121435 - 0.38381794893125283518e-1 * t121437 + 0.41123351671205660912e-2 * t114792;
    (t121440,)
}
