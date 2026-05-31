//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1239/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1239<F: Float>(t10007: F, t119: F, t120: F, t13262: F, t210: F, t2571: F, t2643: F, t2645: F, t2646: F, t2647: F, t40972: F, t40977: F, t41039: F, t41072: F, t41161: F, t41435: F, t41437: F, t41448: F, t41453: F, t41467: F, t41468: F, t4178: F, t829: F, t9516: F, t9621: F, t9626: F, t9642: F, t9646: F, t9647: F, t9653: F) -> F {
    let t41487 = -F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t41435 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t41437 + t2643 * t2645 * t120 * t9516 * t829 / F::cast_from(192.0_f64) + t2643 * t2645 * t41039 * t2647 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t4178 * t9646 * t9626 * t41448 + t13262 * t2645 * t41039 * t41453 / F::cast_from(32.0_f64) - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t2643 * t9646 * t9626 * t9647 + t2643 * t2645 * t41072 * t2647 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t2643 * t41467 * t2646 * t41468 + t9642 * t9653 / F::cast_from(64.0_f64) + t2643 * t2645 * t9621 * t10007 / F::cast_from(128.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t41161 * t210 * t119 * t40972 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2571 * t210 * t119 * t40977;
    t41487
}
