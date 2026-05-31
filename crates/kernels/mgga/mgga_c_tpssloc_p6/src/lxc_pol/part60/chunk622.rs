//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 622/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk622<F: Float>(t1268: F, t1458: F, t2039: F, t4028: F, t7042: F, t7676: F, t7787: F, t7801: F, t7170: F, t7687: F, t1807: F, t2085: F) -> (F, F, F) {
    let t7900 = F::cast_from(2.0_f64) * t1268 * t7801 + F::cast_from(2.0_f64) * t1458 * t7042 + F::cast_from(2.0_f64) * t2039 * t4028 + F::cast_from(2.0_f64) * t2039 * t7676 + t7787;
    let t7904 = t7170 * t7687;
    let t7910 = t1807 * t2085;
    (t7900, t7904, t7910)
}
