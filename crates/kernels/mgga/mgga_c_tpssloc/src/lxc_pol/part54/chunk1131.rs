//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1131/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1131<F: Float>(t225: F, t26329: F, t26229: F, t1324: F, t254: F, t22573: F, t7684: F, t6875: F, t8944: F, t111: F, t26966: F, t2094: F, t40611: F, t12461: F, t7216: F, t193: F, t7125: F) -> (F, F, F, F, F, F, F, F, F) {
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t91505 = t1324 * t254;
    let t91655 = t7684 * t22573;
    let t91669 = t6875 * t8944;
    let t92090 = t26966 * t111;
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    let t92271 = t193 * t7125;
    (t91488, t91491, t91505, t91655, t91669, t92090, t92169, t92200, t92271)
}
