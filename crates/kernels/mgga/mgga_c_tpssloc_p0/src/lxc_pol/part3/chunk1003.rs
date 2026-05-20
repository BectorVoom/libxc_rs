//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1003/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1003<F: Float>(t1484: F, t212: F, t9523: F, t2586: F, t213: F, t4119: F, t221: F, t776: F, t2553: F, t4128: F, t2570: F, t67: F) -> (F, F, F, F, F) {
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    let t12986 = t2586 * t12985;
    let t12988 = t213 * t4119;
    let t12990 = t221 * t12988 * t776;
    let t12994 = t221 * t4128 * t2553;
    let t12997 = t2570 * t67;
    (t12984, t12986, t12990, t12994, t12997)
}
