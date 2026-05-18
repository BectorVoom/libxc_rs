//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 472/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk472<F: Float>(t1603: F, t381: F, t1409: F, t998: F, t974: F, t225: F, t68: F) -> (F, F, F, F, F) {
    let t1604 = t1603 * t381;
    let t1606 = t998 * t1409;
    let t1607 = t974 * t1606;
    let t1610 = t1603 * t225;
    let t1611 = t1610 * t68;
    (t1604, t1606, t1607, t1610, t1611)
}
