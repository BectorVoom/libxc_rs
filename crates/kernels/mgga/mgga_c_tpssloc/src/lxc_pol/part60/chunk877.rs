//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 877/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk877<F: Float>(t214: F, t33245: F, t1985: F, t1842: F, t31558: F, t22635: F, t1992: F, t1807: F, t8617: F, t31576: F, t31578: F, t31582: F, t32712: F, t32715: F, t32718: F, t32722: F, t32724: F) -> (F, F, F, F, F, F, F) {
    let t33246 = t214 * t33245;
    let t33247 = t1985 * t33246;
    let t33249 = t31558 * t1842;
    let t33250 = t22635 * t33249;
    let t33251 = t1992 * t33250;
    let t33259 = t1807 * t8617;
    let t33266 = -t31576 - F::cast_from(0.96894614625936938046e-2_f64) * t32712 - t31578 - F::cast_from(0.16149102437656156341e-2_f64) * t32715 + t32718 / F::new(768.0) - t32722 / F::new(768.0) - t31582 - t32724 / F::new(192.0);
    (t33246, t33247, t33249, t33250, t33251, t33259, t33266)
}
