//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3120/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3120<F: Float>(t1164: F, t4883: F, t64525: F, t15044: F, t4869: F, t18910: F, t3378: F, t63446: F, t63449: F, t63451: F, t63557: F, t63560: F, t63563: F, t64514: F, t64517: F, t64520: F, t64522: F, t64524: F) -> (F, F, F, F) {
    let t64528 = F::cast_from(0.34631718211362927518e2_f64) * t1164 * t64525 * t4883;
    let t64530 = F::cast_from(0.23392894490538584828e1_f64) * t4869 * t15044;
    let t64533 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t18910 * t3378;
    let t64534 = t63446 - t63449 + t63451 + t64514 - t64517 - t64520 + t64522 - t64524 - t64528 + t64530 - t64533 + t63557 + t63560 - t63563;
    (t64528, t64530, t64533, t64534)
}
