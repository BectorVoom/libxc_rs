//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1223/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1223<F: Float>(t1692: F, t17921: F, t17931: F, t17934: F, t17938: F, t18053: F, t18056: F, t18059: F, t1812: F, t18728: F, t18803: F, t18807: F, t18812: F, t1991: F, t2439: F, t30: F, t3552: F, t5539: F, t5591: F, t580: F, t5849: F, t5853: F) -> F {
    let t18823 = F::new(3.0) * t3552 * t1812 * t17921 + F::new(3.0) * t2439 * t5849 * t5539 - F::new(3.0) * t18728 * t17931 + F::new(3.0) * t2439 * t1812 * t17934 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t17938 + t1692 * t18803 * t30 / F::new(2.0) - t1692 * t18807 * t5591 + t1692 * t5849 * t580 + t1692 * t18812 * t18053 - t1692 * t5853 * t18056 - t1692 * t5853 * t18059 / F::new(2.0) + t1692 * t1812 * t1991 / F::new(2.0);
    t18823
}
