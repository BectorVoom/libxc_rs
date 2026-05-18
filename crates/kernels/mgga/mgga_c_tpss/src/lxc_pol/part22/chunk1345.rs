//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1345/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1345<F: Float>(t63928: F, t63945: F, t61034: F, t61051: F, t61054: F, t61058: F, t61060: F, t62690: F, t63930: F, t63932: F, t63939: F, t63941: F, t63943: F) -> F {
    let t66399 = F::new(7.0) / F::new(576.0) * t63928;
    let t66410 = F::new(119.0) / F::new(3456.0) * t63945;
    let t66411 = t66399 + t63930 / F::new(96.0) - t62690 - F::new(5.0) / F::new(96.0) * t63932 - F::new(7.0) / F::new(144.0) * t61034 - F::new(119.0) / F::new(1728.0) * t61051 + F::new(7.0) / F::new(1152.0) * t61054 - F::new(7.0) / F::new(576.0) * t61058 + F::new(7.0) / F::new(1152.0) * t61060 + t63939 / F::new(192.0) - F::new(5.0) / F::new(192.0) * t63941 - t63943 / F::new(96.0) - t66410;
    t66411
}
