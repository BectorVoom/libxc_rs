//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 946/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk946<F: Float>(t1888: F, t23270: F, t2742: F, t31332: F, t232: F, t6646: F, t84842: F, t112955: F, t112959: F, t112962: F, t112967: F, t112969: F, t112973: F, t112975: F, t112980: F, t2617: F, t2679: F, t31394: F, t31395: F, t812: F) -> (F, F) {
    let t114632 = t1888 * t23270 * t31332 * t2742;
    let t114642 = t1888 * t6646 * t84842 * t232;
    let t114648 = -t112955 - t112959 + t112962 + t112967 - F::cast_from(0.82246703342411321825e-2_f64) * t114642 + t112969 + t112973 + t112975 + t112980 - t812 * t31394 * t2679 - F::new(2.0) * t2617 * t31395;
    (t114632, t114648)
}
