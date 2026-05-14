//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1255/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1255<F: Float>(t11645: F, t5605: F, t11588: F, t18069: F, t19898: F, t5570: F, t18155: F, t6171: F, t1729: F, t8547: F, t2715: F, t27754: F, t940: F, t2712: F, t9066: F, t2710: F) -> (F, F, F, F, F, F, F) {
    let t64483 = t5605 * t11645 / 432.0;
    let t64487 = t18069 * t11588 / 1728.0;
    let t64515 = t19898 * t5570;
    let t64529 = t6171 * t18155;
    let t64546 = t1729 * t8547;
    let t64548 = t64546 * t27754 * t2715;
    let t64557 = t64546 * t27754 * t940;
    let t64563 = t2712 * t9066;
    let t64565 = t1729 * t2710 * t64563 * t940;
    (t64483, t64487, t64515, t64529, t64548, t64557, t64565)
}
