//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1083/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1083<F: Float>(t16132: F, t1825: F, t1352: F, t19743: F, t19660: F, t118: F, t6330: F, t794: F, t12202: F, t19631: F, t210: F, t214: F, t6347: F, t3739: F, t12211: F, t6353: F) -> (F, F, F, F, F, F, F) {
    let t19756 = t16132 * t1825;
    let t19761 = t19743 * t1352;
    let t19763 = t19660 * t1352;
    let t19767 = t118 * t794 * t6330;
    let t19768 = t12202 * t19767;
    let t19771 = t210 * t214 * t19631;
    let t19775 = t118 * t794 * t6347;
    let t19776 = t3739 * t19775;
    let t19779 = t12211 * t6353;
    (t19756, t19761, t19763, t19768, t19771, t19776, t19779)
}
