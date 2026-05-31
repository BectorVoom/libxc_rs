//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1216/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216<F: Float>(t2617: F, t9637: F, t2649: F, t2691: F, t812: F, t815: F, t10003: F, t119: F, t13222: F, t13254: F, t210: F, t2633: F, t2643: F, t2647: F, t40848: F, t41078: F, t41084: F, t41086: F, t41088: F, t41090: F, t41096: F, t4178: F, t4180: F, t4182: F, t787: F, t9621: F, t9629: F, t9642: F, t9646: F, t9647: F) -> F {
    let t41107 = t2617 * t9637;
    let t41108 = t41107 * t2649;
    let t41115 = t812 * t815 * t2691;
    let t41116 = t41115 * t2649;
    let t41120 = t2643 * t13222 * t41078 * t2647 / F::cast_from(64.0_f64) + F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t41084 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t41086 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t41088 - t4178 * t13222 * t4182 * t41090 / F::cast_from(32.0_f64) + t41096 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t2643 * t9646 * t9621 * t9647 + t9642 * t10003 / F::cast_from(64.0_f64) + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t4178 * t4180 * t9621 * t2633 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t41108 - t787 * t210 * t119 * t40848 / F::cast_from(48.0_f64) + F::cast_from(119.0_f64) / F::cast_from(288.0_f64) * t41116 - t13254 * t9629 / F::cast_from(32.0_f64);
    t41120
}
