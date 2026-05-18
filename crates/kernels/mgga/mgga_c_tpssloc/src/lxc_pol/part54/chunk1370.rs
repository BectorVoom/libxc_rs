//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1370/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1370<F: Float>(t22986: F, t23270: F, t31332: F, t98960: F, t114770: F, t25054: F, t25038: F, t25040: F, t114606: F, t118488: F, t118491: F, t118498: F, t118499: F, t24297: F, t26690: F, t2718: F, t31311: F, t31400: F, t4147: F, t4268: F, t4300: F, t6627: F, t7517: F, t855: F, t8562: F) -> F {
    let t121326 = t22986 * t23270 * t31332 * t98960;
    let t121336 = t22986 * t114770 * t25054;
    let t121339 = t25038 * t114770 * t25040;
    let t121343 = -t118488 - t4268 * t31400 + t118491 + F::new(2.0) * t6627 * t26690 + t118498 + t118499 - F::new(0.3289868133696452873e-1) * t121326 - F::new(0.38381794893125283518e-1) * t114606 + F::new(2.0) * t855 * t2718 * t8562 * t4300 + F::new(2.0) * t24297 * t7517 + F::new(0.16449340668482264365e-1) * t121336 + F::new(0.49348022005446793095e-1) * t121339 + F::new(2.0) * t4147 * t31311;
    t121343
}
