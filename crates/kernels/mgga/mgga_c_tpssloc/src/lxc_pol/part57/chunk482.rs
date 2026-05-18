//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 482/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk482<F: Float>(t3242: F, t5392: F, t3240: F, t123: F, t3247: F, t1088: F, t1089: F, t5398: F, t3237: F, t4721: F, t423: F, t1671: F, t4740: F) -> (F, F, F, F, F, F, F, F) {
    let t5971 = t3242 * t5392;
    let t5972 = t3240 * t5971;
    let t5973 = t123 * t5972;
    let t5975 = t3247 * t5392;
    let t5976 = t1088 * t5975;
    let t5977 = t123 * t5976;
    let t5979 = t1089 * t5398;
    let t5980 = t1088 * t5979;
    let t5981 = t123 * t5980;
    let t5983 = t3237 - F::new(0.11872222222222222222e-1) * t4721 - F::new(0.11872222222222222222e-1) * t5973 + F::new(0.35616666666666666666e-1) * t5977 + F::new(0.17808333333333333333e-1) * t5981;
    let t5985 = F::new(0.621814e-1) * t5983 * t423;
    let t5987 = F::new(2.0) * t4740 * t1671;
    (t5971, t5973, t5975, t5977, t5979, t5981, t5985, t5987)
}
