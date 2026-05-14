//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 574/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk574<F: Float>(t131: F, t7371: F, t467: F, t1009: F, t461: F, t1209: F, t475: F, t68: F, t1245: F, t1235: F, t2147: F, t462: F, t1215: F, t2144: F, t1246: F, t493: F, t7348: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7372 = t7371 * t131;
    let t7373 = t7372 * t467;
    let t7374 = t461 * t1009;
    let t7375 = t7374 * t1209;
    let t7376 = t68 * t475;
    let t7377 = t1245 * t7376;
    let t7378 = t7375 * t7377;
    let t7381 = t2147 * t1235;
    let t7382 = t462 * t7381;
    let t7386 = t2144 * t1215;
    let t7387 = t7386 * t1246;
    let t7389 = t493 * t7348;
    (t7372, t7373, t7375, t7376, t7377, t7378, t7381, t7382, t7387, t7389)
}
