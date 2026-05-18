//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1036/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1036<F: Float>(t360: F, t4649: F, t68: F, t6744: F, t344: F, t7573: F, t6740: F, t1622: F, t23489: F, t23533: F, t23537: F, t23541: F, t23544: F, t23554: F, t23560: F, t4590: F, t4596: F, t4600: F, t4636: F, t4652: F, t6723: F, t6735: F, t6742: F, t6747: F, t6755: F, t6765: F, t7574: F, t7578: F, t7583: F) -> F {
    let t25678 = t4649 * t68 * t360;
    let t25679 = t6744 * t25678;
    let t25682 = t7573 * t344;
    let t25683 = t6740 * t25682;
    let t25703 = F::new(5.0) / F::new(6912.0) * t6765 * t4590 + F::new(0.10093189023535097714e-3) * t23489 * t7583 + F::new(0.10093189023535097714e-3) * t6742 * t25679 + F::new(0.10093189023535097714e-3) * t25683 * t6747 + t23537 * t4596 / F::new(768.0) - t23541 * t4600 / F::new(1536.0) + t23533 / F::new(3456.0) + F::new(0.80745512188280781712e-3) * t6723 * t7578 - F::new(0.10093189023535097714e-3) * t7574 * t6735 + t23554 / F::new(2304.0) - t23560 / F::new(432.0) + t6755 * t4652 / F::new(1536.0) + t23544 * t1622 / F::new(2304.0) + t6765 * t4636 / F::new(2304.0);
    t25703
}
