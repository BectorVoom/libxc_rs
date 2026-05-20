//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 731/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk731<F: Float>(t478: F, t7327: F, t1215: F, t68: F, t475: F, t1202: F, t2140: F, t1209: F, sigma2: F) -> (F, F, F, F, F) {
    let t7328 = t7327 * t478;
    let t7329 = t1215 * t68;
    let t7330 = t7329 * t475;
    let t7331 = t7328 * t7330;
    let t7334 = t1202 * t2140;
    let t7337 = t1209 * sigma2;
    (t7328, t7330, t7331, t7334, t7337)
}
