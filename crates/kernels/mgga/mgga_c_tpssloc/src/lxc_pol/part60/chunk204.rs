//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 204/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk204<F: Float>(t974: F, t976: F, t344: F, t883: F, t221: F, t967: F, t339: F, t191: F) -> (F, F, F, F, F, F) {
    let t977 = t974 * t976;
    let t978 = t344 * t883;
    let t995 = t221 * t967;
    let t997 = t339 * t995 / 288.0;
    let t998 = t976 * t883;
    let t1008 = t191 * t191;
    let t1009 = 1.0 / t1008;
    (t977, t978, t997, t998, t1008, t1009)
}
