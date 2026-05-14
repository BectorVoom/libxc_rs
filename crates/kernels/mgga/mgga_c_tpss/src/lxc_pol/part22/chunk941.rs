//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 941/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk941<F: Float>(t2116: F, t256: F, t3724: F, t823: F, t10704: F, t10706: F, t10709: F, t10712: F, t10716: F, t10719: F, t10721: F, t10724: F, t10727: F, t10731: F, t1364: F, t1692: F, t198: F, t2428: F, t2439: F, t2440: F, t3610: F, t3728: F, t750: F, t7979: F, t7988: F, t7992: F, t8045: F, t8231: F, t8234: F) -> (F,) {
    let t10919 = t2116 * t256;
    let t10923 = t3724 * t823;
    let t10935 = 6.0 * t10919 * t1364 * t198 + 6.0 * t10923 * t2439 * t750 + 3.0 * t1364 * t2439 * t8045 - t1692 * t2428 * t3728 + 6.0 * t2439 * t2440 * t3610 + t10704 + t10706 + t10709 + t10712 + t10716 - t10719 + t10721 + t10724 + t10727 + t10731 + t7979 + t7988 + t7992 - t8231 - t8234;
    (t10935,)
}
