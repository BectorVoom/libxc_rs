//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 314/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk314<F: Float>(t607: F, t998: F, t974: F, t225: F, t990: F, t68: F, t369: F, t191: F) -> (F, F, F, F, F, F, F) {
    let t999 = t998 * t607;
    let t1000 = t974 * t999;
    let t1003 = t990 * t225;
    let t1004 = t1003 * t68;
    let t1005 = t1004 * t369;
    let t1008 = t191 * t191;
    let t1009 = F::cast_from(1.0_f64) / t1008;
    (t999, t1000, t1003, t1004, t1005, t1008, t1009)
}
