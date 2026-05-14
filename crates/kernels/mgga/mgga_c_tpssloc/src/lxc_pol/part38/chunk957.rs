//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 957/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk957<F: Float>(t1484: F, t212: F, t9523: F, t2586: F, t213: F, t4119: F, t221: F, t776: F, t2553: F, t4128: F, t2570: F, t67: F, t792: F, t686: F, t4127: F, t9526: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F) -> (F,) {
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    let t12986 = t2586 * t12985;
    let t12988 = t213 * t4119;
    let t12990 = t221 * t12988 * t776;
    let t12994 = t221 * t4128 * t2553;
    let t12997 = t2570 * t67;
    let t12998 = t792 * t12997;
    let t13000 = t686 * t12984 * t776;
    let t13002 = 0.49999999999999999998e-2 * t12998 * t13000;
    let t13003 = 0.33333333333333333332e-2 * t9526 - t9540 - 0.25925925925925925926e-1 * t9542 + 0.38888888888888888888e-2 * t9544 - 0.10555555555555555555e-1 * t9547 - 0.25e-2 * t9552 + 0.83333333333333333332e-3 * t9556 + 0.16666666666666666666e-2 * t12986 + 0.99999999999999999996e-2 * t4127 * t12990 + 0.49999999999999999998e-2 * t4127 * t12994 - t13002;
    (t13003,)
}
