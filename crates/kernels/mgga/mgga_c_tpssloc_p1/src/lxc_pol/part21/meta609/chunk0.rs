//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2375/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2375<F: Float>(t3684: F, t39500: F, t2393: F, t2528: F, t677: F, t9722: F, t118: F, t2375: F, t3681: F, t12110: F, t9888: F, t9467: F) -> (F, F, F, F, F, F, F, F) {
    let t39502 = F::cast_from(0.86748650402413918736e-1_f64) * t3684 * t39500;
    let t39503 = t2393 * t2528;
    let t39505 = F::cast_from(0.12842595503380418954e1_f64) * t3684 * t39503;
    let t39506 = t677 * t9722;
    let t39508 = F::cast_from(0.38527786510141256862e1_f64) * t3684 * t39506;
    let t39510 = t3681 * t118 * t2375;
    let t39512 = t12110 * t9888;
    let t39514 = t12110 * t9467;
    (t39502, t39503, t39505, t39506, t39508, t39510, t39512, t39514)
}
