//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1127/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1127<F: Float>(t120: F, t9660: F, t10003: F, t9638: F, t10009: F, t2617: F, t9600: F, t849: F, t2707: F, t9993: F, t2642: F, t9612: F, t9649: F, t9957: F, t13262: F, t2623: F, t2643: F, t2645: F, t2649: F, t40848: F, t40951: F, t4178: F, t4180: F, t820: F, t829: F, t843: F, t847: F, t9623: F, t9626: F, t9627: F, t9642: F, t9997: F) -> (F, F, F) {
    let t41039 = t120 * t9660;
    let t41048 = t9638 * t10003;
    let t41050 = t9638 * t10009;
    let t41052 = t2617 * t9600;
    let t41053 = t41052 * t849;
    let t41055 = t9993 * t2707;
    let t41063 = t9612 * t2642;
    let t41066 = t9638 * t9649;
    let t41072 = t120 * t9957;
    let t41077 = -t4178 * t2645 * t41039 * t9627 / 32.0 - 3.0 / 256.0 * t13262 * t4180 * t9626 * t40951 - 7.0 / 96.0 * t41048 - 7.0 / 96.0 * t41050 - 119.0 / 288.0 * t41053 + 7.0 / 96.0 * t41055 - t2623 * t9997 / 192.0 - t843 * t847 * t820 * t40848 / 768.0 + t41063 * t2649 / 64.0 + 35.0 / 96.0 * t41066 - 5.0 / 64.0 * t9642 * t9649 - t9642 * t9623 / 256.0 - t2643 * t4180 * t41072 * t829 / 768.0;
    (t41039, t41072, t41077)
}
