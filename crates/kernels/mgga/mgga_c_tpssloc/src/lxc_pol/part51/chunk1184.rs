//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1184/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1184<F: Float>(t33114: F, t645: F, t8513: F, t7440: F, t79: F, t641: F, t33118: F, t6504: F, t26043: F, t8307: F, t26502: F, t3701: F, t26114: F, t8327: F, t191: F, t192: F, t26138: F) -> (F, F, F, F, F, F, F) {
    let t119938 = t8513 * t33114 * t645;
    let t119942 = t79 * t7440;
    let t119944 = t8513 * t119942 * t641;
    let t119952 = t8513 * t33118 * t6504;
    let t119965 = t8513 * t8307 * t26043;
    let t120016 = t3701 * t26502;
    let t120067 = 2.0 * t26114 * t8327;
    let t120071 = t26138 * t191 * t192;
    (t119938, t119944, t119952, t119965, t120016, t120067, t120071)
}
