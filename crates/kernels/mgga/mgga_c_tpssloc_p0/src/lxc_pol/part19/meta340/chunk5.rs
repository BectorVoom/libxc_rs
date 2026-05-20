//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1214/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214<F: Float>(t9638: F, t9649: F, t120: F, t9957: F, t13262: F, t2623: F, t2643: F, t2645: F, t2649: F, t40848: F, t40951: F, t41039: F, t41048: F, t41050: F, t41053: F, t41055: F, t41063: F, t4178: F, t4180: F, t820: F, t829: F, t843: F, t847: F, t9623: F, t9626: F, t9627: F, t9642: F, t9997: F) -> (F, F) {
    let t41066 = t9638 * t9649;
    let t41072 = t120 * t9957;
    let t41077 = -t4178 * t2645 * t41039 * t9627 / F::new(32.0) - F::new(3.0) / F::new(256.0) * t13262 * t4180 * t9626 * t40951 - F::new(7.0) / F::new(96.0) * t41048 - F::new(7.0) / F::new(96.0) * t41050 - F::new(119.0) / F::new(288.0) * t41053 + F::new(7.0) / F::new(96.0) * t41055 - t2623 * t9997 / F::new(192.0) - t843 * t847 * t820 * t40848 / F::new(768.0) + t41063 * t2649 / F::new(64.0) + F::new(35.0) / F::new(96.0) * t41066 - F::new(5.0) / F::new(64.0) * t9642 * t9649 - t9642 * t9623 / F::new(256.0) - t2643 * t4180 * t41072 * t829 / F::new(768.0);
    (t41072, t41077)
}
