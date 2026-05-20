//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1295/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295<F: Float>(t2859: F, t2884: F, t302: F, t41642: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41831: F, t41833: F, t41836: F, t41839: F, t41842: F, t41845: F) -> (F, F) {
    let t42154 = t302 / t2884 / t2859;
    let t42172 = F::cast_from(0.13892666666666666667e1_f64) * t41831 + F::new(0.166712e1) * t41833 - F::new(0.125034e1) * t41836 - F::new(0.104195e0) * t41839 + F::new(0.250068e1) * t41842 + F::new(0.62517e0) * t41845 + F::new(0.309885e1) * t41642 - F::cast_from(0.13772666666666666666e1_f64) * t41656 - F::cast_from(0.91817777777777777776e0_f64) * t41658 + F::cast_from(0.76514814814814814814e0_f64) * t41660 + F::cast_from(0.68863333333333333332e0_f64) * t41662 - F::cast_from(0.15302962962962962963e1_f64) * t41669 - F::new(0.516475e0) * t41673 + F::cast_from(0.27545333333333333333e1_f64) * t41675;
    (t42154, t42172)
}
