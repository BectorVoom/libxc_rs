//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1148/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1148<F: Float>(t13258: F, t9634: F, t9629: F, t2379: F, t2632: F, t776: F, t9975: F, t6589: F, t67: F, t246: F, t232: F, t9458: F, t10007: F, t119: F, t120: F, t13262: F, t210: F, t2571: F, t2643: F, t2645: F, t2646: F, t2647: F, t40972: F, t40977: F, t41039: F, t41072: F, t41161: F, t4178: F, t829: F, t9516: F, t9621: F, t9626: F, t9642: F, t9646: F, t9647: F, t9653: F) -> (F,) {
    let t41435 = t13258 * t9634;
    let t41437 = t13258 * t9629;
    let t41448 = t2632 * t2379;
    let t41453 = t9975 * t776;
    let t41466 = t6589 * t67;
    let t41467 = t41466 * t246;
    let t41468 = t232 * t9458;
    let t41487 = -7.0 / 192.0 * t41435 + 7.0 / 48.0 * t41437 + t2643 * t2645 * t120 * t9516 * t829 / 192.0 + t2643 * t2645 * t41039 * t2647 / 192.0 + 5.0 / 64.0 * t4178 * t9646 * t9626 * t41448 + t13262 * t2645 * t41039 * t41453 / 32.0 - 5.0 / 128.0 * t2643 * t9646 * t9626 * t9647 + t2643 * t2645 * t41072 * t2647 / 192.0 + 5.0 / 32.0 * t2643 * t41467 * t2646 * t41468 + t9642 * t9653 / 64.0 + t2643 * t2645 * t9621 * t10007 / 128.0 + 5.0 / 4.0 * t41161 * t210 * t119 * t40972 + 3.0 / 16.0 * t2571 * t210 * t119 * t40977;
    (t41487,)
}
