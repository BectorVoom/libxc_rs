//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2411/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2411<F: Float>(t68785: F, t68798: F, t68812: F, t68825: F, t68839: F, t68851: F, t68864: F, t68877: F, t893: F, t913: F, t21303: F, t42023: F) -> (F, F) {
    let t68883 = F::cast_from(1.0_f64) * t893 * (t68785 + t68798 + t68812 + t68825 + t68839 + t68851 + t68864 + t68877) * t913;
    let t68885 = F::cast_from(0.51726012919273400301e3_f64) * t42023 * t21303;
    (t68883, t68885)
}
