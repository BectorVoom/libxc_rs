//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2202/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2202<F: Float>(t2517: F, t2658: F, t5392: F, t12923: F, t3966: F, t4194: F, t12924: F, t16693: F, t16616: F, t2528: F, t12932: F, t4205: F) -> (F, F, F, F, F) {
    let t59013 = t2658 * t2517 * t5392;
    let t59022 = t4194 * t12923 * t3966;
    let t59024 = t16693 * t12924;
    let t59028 = t16616 * t2528;
    let t59032 = t4205 * t12932;
    (t59013, t59022, t59024, t59028, t59032)
}
