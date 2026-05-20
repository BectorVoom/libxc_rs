//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 326/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk326<F: Float>(t381: F, t990: F, t221: F, t967: F, t339: F, t883: F, t976: F, t607: F, t974: F, t225: F) -> (F, F, F, F, F, F, F) {
    let t991 = t990 * t381;
    let t995 = t221 * t967;
    let t997 = t339 * t995 / F::new(288.0);
    let t998 = t976 * t883;
    let t999 = t998 * t607;
    let t1000 = t974 * t999;
    let t1003 = t990 * t225;
    (t991, t995, t997, t998, t999, t1000, t1003)
}
