//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2358/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2358<F: Float>(t13042: F, t13053: F, t13065: F, t1492: F, t1519: F, t1528: F, t16804: F, t17022: F, t17056: F, t17090: F, t20936: F, t21034: F, t21050: F, t218: F, t25168: F, t259: F, t2597: F, t2713: F, t4265: F, t4301: F, t46488: F, t5558: F, t5637: F, t5658: F, t58143: F, t68211: F, t852: F) -> F {
    let t68365 = F::new(3.0) * t1492 * t17022 * t259 + F::new(3.0) * t1519 * t16804 * t259 - F::new(18.0) * t17056 * t25168 * t46488 + t20936 * t259 * t852 + t218 * t259 * t68211 + F::new(3.0) * t259 * t4265 * t5558 - F::new(3.0) * t13042 * t5658 + F::new(6.0) * t13053 * t5637 - F::new(3.0) * t13053 * t5658 - F::new(3.0) * t13065 * t5658 - F::new(3.0) * t1528 * t58143 - F::new(3.0) * t17090 * t4301 - t21034 * t2597 - F::new(6.0) * t21050 * t2713;
    t68365
}
