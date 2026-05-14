//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1298/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1298<F: Float>(t1760: F, t60916: F, t6245: F, t6274: F, t9909: F, t19577: F, t5758: F, t18551: F, t19631: F, t5710: F, t18546: F, t6242: F, t18548: F, t18547: F, t19580: F, t44045: F) -> (F, F, F, F, F, F, F) {
    let t65515 = 3.0 * t1760 * t60916 * t6245;
    let t65525 = t1760 * t6274 * t9909;
    let t65527 = 2.0 * t19577 * t5758;
    let t65530 = 3.0 * t1760 * t19631 * t18551;
    let t65532 = 6.0 * t19577 * t5710;
    let t65533 = t6242 * t18546;
    let t65535 = 6.0 * t65533 * t18548;
    let t65538 = 6.0 * t18547 * t19580 * t44045;
    (t65515, t65525, t65527, t65530, t65532, t65535, t65538)
}
