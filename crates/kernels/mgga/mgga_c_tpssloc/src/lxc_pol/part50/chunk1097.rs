//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1097/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1097<F: Float>(t31246: F, t7756: F, t5107: F, t8320: F, t1774: F, t30991: F, t1799: F, t6995: F, t22574: F, t8643: F, t31048: F, t7685: F, t31033: F, t119830: F, t119831: F, t119835: F, t119837: F, t119839: F, t119841: F, t119844: F, t31224: F, t32674: F, t32676: F, t4073: F, t6515: F, t7670: F, t8313: F) -> (F,) {
    let t119845 = t31246 * t7756;
    let t119850 = 2.0 * t8320 * t5107;
    let t119852 = 2.0 * t30991 * t1774;
    let t119853 = t1799 * t6995;
    let t119856 = 6.0 * t22574 * t8643 * t119853;
    let t119858 = 3.0 * t7685 * t31048;
    let t119862 = t7685 * t31033;
    let t119863 = -2.0 * t31224 * t4073 - t5107 * t8313 - 2.0 * t6515 * t7670 - t119830 + t119831 + t119835 - t119837 - t119839 - t119841 - t119844 - 2.0 * t119845 - t119850 - t119852 - t119856 + t119858 - t119862 - t32674 - t32676;
    (t119863,)
}
