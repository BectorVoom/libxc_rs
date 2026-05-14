//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 827/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk827<F: Float>(t2476: F, t676: F, t2504: F, t2512: F, t745: F, t747: F, t2405: F, t2411: F, t2414: F, t701: F) -> (F, F, F, F, F) {
    let t9828 = t676 * t2476;
    let t9843 = t2504 * t2512;
    let t9844 = t9843 * t745;
    let t9847 = t747 * t2504;
    let t9853 = 0.48245938496077605201e2 * t2411 * t2405 * t2414 * t701;
    (t9828, t9843, t9844, t9847, t9853)
}
