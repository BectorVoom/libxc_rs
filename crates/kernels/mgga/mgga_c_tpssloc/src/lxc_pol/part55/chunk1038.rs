//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1038/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1038<F: Float>(t232: F, t2646: F, t4180: F, t30714: F, t235: F, t835: F, t226: F, t8344: F, t8343: F, t849: F, t30698: F, t30701: F, t30705: F, t30707: F, t30710: F) -> (F, F, F, F, F) {
    let t30716 = t4180 * t2646 * t232;
    let t30717 = t30714 * t30716;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30722 = F::new(7.0) / F::new(2304.0) * t30721;
    let t30723 = t8343 * t849;
    let t30725 = -t30698 - F::cast_from(0.48447307312968469025e-2_f64) * t30701 - t30705 - F::cast_from(0.80745512188280781708e-3_f64) * t30707 + t30710 / F::new(1536.0) - t30717 / F::new(1536.0) - t30722 - t30723 / F::new(384.0);
    (t30716, t30719, t30720, t30722, t30725)
}
