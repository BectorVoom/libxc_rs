//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1212/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1212<F: Float>(t9623: F, t9638: F, t10007: F, t10009: F, t13350: F, t210: F, t2553: F, t2571: F, t2605: F, t2643: F, t2645: F, t2646: F, t2684: F, t2707: F, t40998: F, t41009: F, t41012: F, t41014: F, t41025: F, t4178: F, t4180: F, t804: F, t829: F, t9516: F, t9559: F, t9616: F, t9621: F, t9626: F, t9642: F, t9990: F) -> F {
    let t41031 = t9638 * t9623;
    let t41037 = -t9990 * t2707 / F::cast_from(128.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t40998 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t9559 * t210 * t2605 * t2553 + t2571 * t210 * t804 * t9516 / F::cast_from(4.0_f64) + F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t41009 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t41012 + t4178 * t4180 * t2646 * t41014 / F::cast_from(384.0_f64) + t9642 * t10009 / F::cast_from(64.0_f64) + t2643 * t2645 * t9626 * t10007 / F::cast_from(128.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t41025 - t2643 * t4180 * t9621 * t2684 / F::cast_from(512.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t41031 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2643 * t13350 * t829 * t9616;
    t41037
}
