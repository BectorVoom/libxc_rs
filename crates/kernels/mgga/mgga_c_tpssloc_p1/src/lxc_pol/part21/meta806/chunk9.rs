//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2807/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2807<F: Float>(t16888: F, t9638: F, t12971: F, t13005: F, t13191: F, t13198: F, t13350: F, t1495: F, t1510: F, t17003: F, t210: F, t221: F, t2553: F, t2571: F, t2643: F, t41410: F, t4172: F, t47333: F, t47353: F, t5567: F, t5571: F, t5587: F, t59198: F, t59279: F, t59282: F, t59288: F, t59298: F, t59308: F, t59310: F, t776: F, t9559: F, t9642: F) -> F {
    let t59322 = t9638 * t16888;
    let t59324 = F::new(35.0) / F::new(96.0) * t59279 + F::new(7.0) / F::new(2304.0) * t59282 + F::new(5.0) / F::new(384.0) * t4172 * t13198 + t41410 * t5587 / F::new(1536.0) + F::new(119.0) / F::new(13824.0) * t59288 - t9559 * t210 * t5567 * t2553 / F::new(4.0) + t2571 * t210 * t1495 * t12971 / F::new(8.0) - F::new(7.0) / F::new(24.0) * t59298 + t2571 * t210 * t17003 * t776 / F::new(8.0) + t2571 * t210 * t5571 * t2553 / F::new(16.0) - F::new(7.0) / F::new(2304.0) * t59308 - F::new(7.0) / F::new(12.0) * t59310 - F::new(7.0) / F::new(24.0) * t47333 - F::new(5.0) / F::new(192.0) * t9642 * t16888 - F::new(5.0) / F::new(192.0) * t2643 * t13350 * t1510 * t13191 + F::new(35.0) / F::new(576.0) * t47353 - t13005 * t221 * t59198 + F::new(35.0) / F::new(288.0) * t59322;
    t59324
}
