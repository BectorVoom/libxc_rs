//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 943/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk943<F: Float>(t343: F, t4540: F, t6734: F, t4571: F, t6765: F, t4630: F, t6755: F, t1611: F, t6758: F, t1036: F, t7586: F, t1409: F, t1933: F, t1937: F, t1618: F, t1622: F, t1935: F, t23433: F, t23443: F, t23447: F, t23449: F, t23463: F, t23469: F, t23529: F, t378: F, t6730: F, t7578: F) -> (F,) {
    let t25608 = t4540 * t343;
    let t25609 = t25608 * t6734;
    let t25616 = t6765 * t4571;
    let t25618 = t6755 * t4630;
    let t25622 = t1611 * t6758;
    let t25625 = t7586 * t1036;
    let t25628 = t1933 * t1409;
    let t25629 = t25628 * t1937;
    let t25631 = -0.10093189023535097714e-3 * t6730 * t7578 - 0.10093189023535097714e-3 * t1935 * t25609 + 0.10093189023535097714e-3 * t23443 - t23447 - 0.80745512188280781712e-3 * t23449 - t23529 * t1622 / 432.0 + t25616 / 3456.0 + t25618 / 2304.0 + t23433 * t1618 / 1536.0 - t25622 * t378 / 288.0 + t25625 / 2304.0 - t23463 / 108.0 + 0.10093189023535097714e-3 * t25629 - t23469;
    (t25631,)
}
