//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 735/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk735<F: Float>(t2162: F, t782: F, t3629: F, t3628: F, t1381: F, t2169: F, t2177: F, t2175: F, t2224: F, t2281: F, t2285: F, t3546: F, t3547: F, t3559: F, t3562: F, t3563: F, t3568: F, t3571: F, t3574: F, t3592: F) -> (F, F, F, F, F) {
    let t3630 = t2162 * t782;
    let t3631 = t3629 * t3630;
    let t3632 = t3628 * t3631;
    let t3635 = t2169 * t1381;
    let t3637 = t3629 * t2177;
    let t3638 = t2175 * t3637;
    let t3641 = t3546 + t3547 - t3559 - t3562 + t2224 - t2285 - t3563 + t3568 - t2281 + t3571 + t3574 + t3592;
    (t3630, t3632, t3635, t3638, t3641)
}
