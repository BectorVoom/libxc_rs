//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1126/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1126<F: Float>(t9573: F, t9657: F, t2559: F, t2570: F, t2606: F, t782: F, t9558: F, t10033: F, t2632: F, t9957: F, t9638: F, t9653: F, t9623: F, t10007: F, t10009: F, t13350: F, t210: F, t2553: F, t2571: F, t2605: F, t2643: F, t2645: F, t2646: F, t2684: F, t2707: F, t4178: F, t4180: F, t804: F, t829: F, t9516: F, t9559: F, t9616: F, t9621: F, t9626: F, t9642: F, t9990: F) -> (F, F, F) {
    let t40998 = t9573 * t9657;
    let t41008 = t2559 * t2570;
    let t41009 = t41008 * t2606;
    let t41011 = t782 * t9558;
    let t41012 = t41011 * t10033;
    let t41014 = t2632 * t9957;
    let t41025 = t9638 * t9653;
    let t41031 = t9638 * t9623;
    let t41037 = -t9990 * t2707 / 128.0 - 7.0 / 4.0 * t40998 - 3.0 / 2.0 * t9559 * t210 * t2605 * t2553 + t2571 * t210 * t804 * t9516 / 4.0 + 35.0 / 12.0 * t41009 + 7.0 / 3.0 * t41012 + t4178 * t4180 * t2646 * t41014 / 384.0 + t9642 * t10009 / 64.0 + t2643 * t2645 * t9626 * t10007 / 128.0 - 7.0 / 96.0 * t41025 - t2643 * t4180 * t9621 * t2684 / 512.0 + 7.0 / 384.0 * t41031 - 5.0 / 64.0 * t2643 * t13350 * t829 * t9616;
    (t41008, t41011, t41037)
}
