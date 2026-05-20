//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 956/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk956<F: Float>(t360: F, t4649: F, t1021: F, t248: F, t1020: F, t1025: F, t1041: F, t1046: F, t1618: F, t1622: F, t3104: F, t3109: F, t3114: F, t3117: F, t3140: F, t3156: F, t3160: F, t3163: F, t378: F, t4617: F, t4622: F, t4625: F, t4631: F, t4636: F, t4641: F, t4644: F) -> (F, F, F) {
    let t4650 = t4649 * t360;
    let t4652 = t248 * t1021 * t4650;
    let t4656 = t3104 / F::new(4608.0) + t4617 * t378 / F::new(3072.0) + t3140 / F::new(864.0) + t3156 / F::new(4608.0) - t4622 * t378 / F::new(576.0) + t4625 / F::new(4608.0) - t3109 * t1618 / F::new(576.0) + t4631 / F::new(4608.0) + t3117 * t1622 / F::new(4608.0) + t1041 * t4636 / F::new(4608.0) + t4641 * t1025 / F::new(3072.0) + t4644 * t1046 / F::new(4608.0) + t3114 * t1618 / F::new(3072.0) + t1020 * t4652 / F::new(3072.0) - t3160 - t3163 / F::new(108.0);
    (t4650, t4652, t4656)
}
