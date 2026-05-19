//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1370/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1370<F: Float>(t5: F, t122975: F, t123020: F, t112: F, t119810: F, t119811: F, t119824: F, t119826: F, t119830: F, t119831: F, t119835: F, t122914: F, t122918: F, t122921: F, t122923: F, t122925: F, t510: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t123022 = piecewise3::<F>(t8, F::new(0.0), t122975 + t123020);
    let t123023 = t123022 * t112;
    let t123025 = -t123023 * t510 - t119810 - F::new(2.0) * t119811 - t119824 - t119826 - t119830 + t119831 + t119835 + F::new(3.0) * t122914 - F::new(2.0) * t122918 - F::new(2.0) * t122921 - F::new(2.0) * t122923 - t122925;
    (t123023, t123025)
}
