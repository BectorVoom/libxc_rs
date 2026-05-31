//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1018/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1018<F: Float>(t128296: F, t2040: F, t33211: F, t7796: F, t102386: F, t1874: F, t28239: F, t8607: F, t22574: F, t28830: F, t36740: F, t33610: F, t7685: F) -> (F, F, F, F, F, F) {
    let t128298 = F::cast_from(4.0_f64) * t128296 * t2040;
    let t128300 = F::cast_from(4.0_f64) * t33211 * t7796;
    let t128302 = F::cast_from(2.0_f64) * t102386 * t1874;
    let t128303 = t8607 * t28239;
    let t128306 = F::cast_from(6.0_f64) * t22574 * t36740 * t28830;
    let t128375 = F::cast_from(2.0_f64) * t7685 * t33610;
    (t128298, t128300, t128302, t128303, t128306, t128375)
}
