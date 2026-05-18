//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1228/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1228<F: Float>(t5895: F, t645: F, t18434: F, t18461: F, t18437: F, t18440: F, t18442: F, t18447: F, t18451: F, t18455: F, t18457: F, t18459: F, t18465: F, t18467: F, t18469: F) -> (F, F, F, F) {
    let t18930 = t5895 * t645;
    let t18934 = F::new(35.0) / F::new(216.0) * t18434;
    let t18943 = F::new(119.0) / F::new(3456.0) * t18461;
    let t18947 = t18934 + F::new(7.0) / F::new(36.0) * t18437 + t18440 / F::new(8.0) - t18442 / F::new(24.0) + t18447 / F::new(384.0) + F::new(7.0) / F::new(576.0) * t18451 + t18455 / F::new(96.0) - t18457 / F::new(768.0) - t18459 / F::new(768.0) + t18943 + F::new(7.0) / F::new(144.0) * t18465 + F::new(5.0) / F::new(192.0) * t18467 - t18469 / F::new(192.0);
    (t18930, t18934, t18943, t18947)
}
