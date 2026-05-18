//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1434/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1434<F: Float>(t114064: F, t115397: F, t115409: F, t115415: F, t115423: F, t115486: F, t120468: F, t120469: F, t120471: F, t122471: F, t122475: F, t122483: F, t122488: F, t1336: F, t1352: F, t1825: F, t31637: F, t33289: F, t3777: F, t5230: F, t5234: F, t5250: F, t5334: F, t8634: F) -> F {
    let t122495 = F::new(2.0) * t5334 * t122471 * t5250 - t1336 * t122475 * t1352 + F::new(0.38381794893125283518e-1) * t115397 + F::new(0.82246703342411321824e-2) * t115409 + t5230 * t8634 + F::new(0.82246703342411321825e-2) * t122483 + F::new(0.19190897446562641759e-1) * t115415 + t120468 + t120469 + t120471 - t114064 - F::new(0.82246703342411321825e-2) * t122488 + F::new(0.41123351671205660912e-2) * t115423 - t5234 * t31637 - t3777 * t33289 - t1336 * t115486 * t1825;
    t122495
}
