//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 881/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk881<F: Float>(t114617: F, t114764: F, t114802: F, t114838: F, t114870: F, t114902: F, t114934: F, t114967: F, t870: F, t1914: F, t2379: F, t2745: F, t113070: F, t113086: F, t113124: F, t1877: F, t2249: F, t22960: F, t22968: F, t23299: F, t23302: F, t24191: F, t24339: F, t25: F, t2522: F, t25373: F, t26563: F, t26756: F, t31430: F, t31434: F, t31449: F, t31451: F, t606: F, t6542: F, t7114: F, t8566: F, t92271: F) -> (F, F, F, F, F) {
    let t114970 = t114617 + t114764 + t114802 + t114838 + t114870 + t114902 + t114934 + t114967;
    let t114971 = t114970 * t870;
    let t114977 = t1914 * t2379;
    let t114988 = t1914 * t2745;
    let t114991 = -t1877 * t7114 * t113086 + t1877 * t8566 * t2249 / 2.0 + 2.0 * t92271 * t31449 - t1877 * t31434 * t23299 - t1877 * t31434 * t23302 / 2.0 + 3.0 / 2.0 * t2522 * t8566 * t22968 - 3.0 * t24191 * t113070 + t1877 * t114971 * t25 / 2.0 + t1877 * t31430 * t606 - 3.0 * t26563 * t22960 * t114977 + 2.0 * t26756 * t113124 - t1877 * t24339 * t31451 + 3.0 * t2522 * t31430 * t6542 + t26756 * t25373 * t114988;
    (t114970, t114971, t114977, t114988, t114991)
}
