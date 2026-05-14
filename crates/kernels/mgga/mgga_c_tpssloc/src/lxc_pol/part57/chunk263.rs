//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 263/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk263<F: Float>(t1173: F, t1174: F, t1706: F, t1710: F, t1717: F, t463: F, t491: F, t1196: F, t1409: F, t974: F, t225: F, t68: F, t484: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F) -> (F, F, F, F, F, F, F, F) {
    let t1720 = -0.22222222222222222222e-2 * t1706 * t463 + t1173 - 0.27777777777777777777e-3 * t1174 * t1710 - 0.83333333333333333332e-3 * t1174 * t1717;
    let t1721 = t1720 * t491;
    let t1725 = t1196 * t1409;
    let t1726 = t974 * t1725;
    let t1729 = t1720 * t225;
    let t1730 = t1729 * t68;
    let t1731 = t1730 * t484;
    let t1734 = -t1659 + t1673 + t1699 + t1701 - t1705;
    (t1720, t1721, t1725, t1726, t1729, t1730, t1731, t1734)
}
