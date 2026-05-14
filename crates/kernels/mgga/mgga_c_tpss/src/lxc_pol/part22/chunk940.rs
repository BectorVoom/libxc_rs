//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 940/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk940<F: Float>(t256: F, t3610: F, t2133: F, t3553: F, t2436: F, t3724: F, t10566: F, t10568: F, t10686: F, t10688: F, t10692: F, t10693: F, t10694: F, t10697: F, t10700: F, t10702: F, t10897: F, t1364: F, t1692: F, t198: F, t207: F, t2116: F, t2439: F, t2440: F, t3548: F, t3552: F, t3683: F, t750: F, t8012: F, t821: F, t8222: F, t8225: F, t823: F) -> (F,) {
    let t10901 = t256 * t3610;
    let t10905 = t3553 * t2133;
    let t10911 = t3724 * t2436;
    let t10918 = t10897 * t198 * t207 * t823 + 12.0 * t10901 * t3552 * t750 - 2.0 * t10911 * t1692 * t821 - 3.0 * t1364 * t2439 * t8012 + 6.0 * t2116 * t3548 * t3552 + 12.0 * t2440 * t3552 * t3683 + 6.0 * t10905 * t3552 + t10566 + t10568 - t10686 + t10688 + t10692 - t10693 + t10694 + t10697 + t10700 + t10702 + t8222 + t8225;
    (t10918,)
}
