//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 831/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk831<F: Float>(t2141: F, t77698: F, t75638: F, t75640: F, t75644: F, t1986: F, t2469: F, t7720: F, t71366: F, t9222: F, t71154: F, t8571: F, t71340: F, t3219: F, t9090: F, t75662: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77699 = t77698 * t2141;
    let t77700 = 0.13637330827122670864e-1 * t77699;
    let t77703 = 0.14967802127329760705e-1 * t75638;
    let t77704 = 0.10227998120342003148e-1 * t75640;
    let t77705 = 0.10227998120342003148e-1 * t75644;
    let t77711 = t1986 * t2469;
    let t77712 = t7720 * t77711;
    let t77713 = 0.85129199786595678796e-5 * t77712;
    let t77714 = t9222 * t71366;
    let t77715 = 0.53205749866622299248e-5 * t77714;
    let t77716 = t8571 * t71154;
    let t77717 = 0.42564599893297839398e-5 * t77716;
    let t77718 = t8571 * t71340;
    let t77719 = 0.12769379967989351819e-4 * t77718;
    let t77723 = t9090 * t3219;
    let t77724 = 0.99317399751028291929e-5 * t77723;
    let t77725 = 0.3830813990396805546e-4 * t75662;
    (t77700, t77703, t77704, t77705, t77713, t77715, t77717, t77719, t77724, t77725)
}
