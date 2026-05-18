//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1056/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1056<F: Float>(t5: F, t130412: F, t130439: F, t112: F, t104990: F, t124728: F, t126035: F, t126036: F, t126116: F, t129008: F, t129015: F, t130377: F, t1458: F, t2039: F, t27863: F, t28951: F, t32350: F, t33690: F, t5493: F, t7266: F, t7801: F, t8446: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t130441 = piecewise3::<f64>(t8, F::new(0.0), t130412 + t130439);
    let t130442 = t130441 * t112;
    let t130443 = F::new(2.0) * t104990 * t2039 + F::new(4.0) * t124728 * t1458 + F::new(2.0) * t129008 * t2039 + F::new(4.0) * t129015 * t2039 + F::new(4.0) * t27863 * t7801 + F::new(2.0) * t28951 * t7266 + F::new(2.0) * t32350 * t5493 + F::new(4.0) * t33690 * t7801 + t126035 + t126036 + t126116 + F::new(2.0) * t130377 + t130442 + t8446;
    (t130442, t130443)
}
