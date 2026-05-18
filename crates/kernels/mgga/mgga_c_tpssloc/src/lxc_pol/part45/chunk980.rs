//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 980/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk980<F: Float>(t113023: F, t113032: F, t114699: F, t114704: F, t114708: F, t114741: F, t114746: F, t114750: F, t114752: F, t226: F, t235: F, t2613: F, t31397: F, t808: F, t8560: F) -> F {
    let t114754 = -F::new(0.3289868133696452873e-1) * t114699 - t113023 + F::new(0.3289868133696452873e-1) * t114704 + F::new(0.82246703342411321825e-2) * t114708 + t2613 * t8560 + F::new(2.0) * t808 * t31397 + t226 * t235 * t114741 + F::new(0.49348022005446793095e-1) * t114746 - F::new(0.82246703342411321825e-2) * t114750 + F::new(0.38381794893125283518e-1) * t114752 - t113032;
    t114754
}
