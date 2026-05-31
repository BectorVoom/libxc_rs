//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1067/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1067<F: Float>(t11678: F, t2741: F, t1465: F, t2460: F, t8523: F, t242: F, t8469: F, t946: F, t1407: F, t8951: F, t967: F, t2748: F, t3969: F) -> (F, F, F, F, F) {
    let t11679 = t2741 * t11678;
    let t11682 = t1465 * t2460;
    let t11683 = t8523 * t11682;
    let t11687 = t242 * t8469 * t1465;
    let t11688 = t946 * t11687;
    let t11691 = t242 * t8951 * t1407;
    let t11692 = t967 * t11691;
    let t11697 = t2748 * t3969 / F::cast_from(648.0_f64);
    (t11679, t11683, t11688, t11692, t11697)
}
