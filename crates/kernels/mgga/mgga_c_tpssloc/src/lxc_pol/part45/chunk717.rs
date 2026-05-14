//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 717/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk717<F: Float>(t23556: F, t354: F, t1036: F, t6759: F, t3: F, t6740: F, t23476: F, t343: F, t1046: F, t1935: F, t23533: F, t23537: F, t23541: F, t23544: F, t23548: F, t23551: F, t23554: F, t3043: F, t3134: F, t3153: F, t378: F, t6717: F, t6747: F) -> (F,) {
    let t23557 = t354 * t23556;
    let t23560 = t6759 * t1036;
    let t23562 = t6740 * t3;
    let t23563 = t23476 * t343;
    let t23564 = t23562 * t23563;
    let t23569 = t23533 / 1728.0 + t23537 * t3134 / 768.0 - t23541 * t3043 / 1536.0 + t23544 * t1046 / 1152.0 - 0.10093189023535097714e-3 * t1935 * t23548 - t23551 * t378 / 144.0 + t23554 / 1152.0 + 19.0 / 864.0 * t23557 * t378 - t23560 / 216.0 - 0.20186378047070195428e-3 * t23564 * t6747 - t6717 * t3153 / 144.0;
    (t23569,)
}
