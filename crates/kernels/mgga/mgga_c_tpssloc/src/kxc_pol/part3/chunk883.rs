//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 883/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk883<F: Float>(t12202: F, t12204: F, t118: F, t3719: F, t794: F, t3739: F, t3732: F, t782: F, t3736: F, t1365: F, t154: F, t205: F, t116: F, t547: F, t1307: F, t212: F) -> (F, F, F, F, F, F, F, F) {
    let t12205 = t12202 * t12204;
    let t12208 = t118 * t794 * t3719;
    let t12209 = t3739 * t12208;
    let t12211 = t782 * t3732;
    let t12212 = t12211 * t3736;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    let t12225 = t547 * t116;
    let t12226 = t212 * t1307;
    (t12205, t12209, t12211, t12212, t12214, t12215, t12225, t12226)
}
