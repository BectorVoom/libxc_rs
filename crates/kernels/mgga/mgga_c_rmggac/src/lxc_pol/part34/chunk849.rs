//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 849/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk849<F: Float>(t75450: F, t75452: F, t75465: F, t75469: F, t530: F, t71486: F, t72132: F, t75455: F, t75458: F, t75461: F, t77605: F, t77606: F, t77608: F, t77614: F, t77620: F, t77621: F, t77624: F) -> (F,) {
    let t77625 = 0.20455996240684006296e-1 * t75450;
    let t77626 = 0.40911992481368012592e-1 * t75452;
    let t77630 = 0.2553875993597870364e-4 * t75465;
    let t77631 = 0.5107751987195740728e-4 * t75469;
    let t77632 = -t77605 - t77606 + t77608 + t77614 + t77620 - t71486 + t77621 - 0.2363e1 * t530 * t72132 - t77624 - t77625 + t77626 - 0.70077224371605468752e-6 * t75455 + 0.10511583655740820313e-5 * t75458 - 0.10511583655740820313e-5 * t75461 - t77630 - t77631;
    (t77632,)
}
