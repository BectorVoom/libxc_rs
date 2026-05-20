//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1193/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1193<F: Float>(t80825: F, t80847: F, t80807: F, t80810: F, t80814: F, t80817: F, t80821: F, t80828: F, t80831: F, t80833: F, t80837: F, t80843: F, t80850: F, t80857: F, t80859: F, t80861: F, t80863: F, t80867: F, t80870: F, t80872: F) -> F {
    let t84514 = F::cast_from(0.2034786907144675699e0_f64) * t80825;
    let t84520 = F::new(455.0) / F::new(648.0) * t80847;
    let t84529 = F::cast_from(0.20186378047070195427e-3_f64) * t80807 + t80810 / F::new(768.0) + F::cast_from(0.12111826828242117256e-2_f64) * t80814 + t80817 / F::new(32.0) - F::new(7.0) / F::new(48.0) * t80821 - t84514 - F::new(7.0) / F::new(8.0) * t80828 - t80831 / F::new(2.0) + t80833 / F::new(64.0) + F::cast_from(0.60559134141210586279e-3_f64) * t80837 - F::cast_from(0.84782787797694820791e-2_f64) * t80843 - t84520 - t80850 / F::new(64.0) - F::cast_from(0.24223653656484234512e-2_f64) * t80857 - F::new(35.0) / F::new(96.0) * t80859 - F::new(5.0) / F::new(32.0) * t80861 + F::new(5.0) / F::new(64.0) * t80863 - F::new(119.0) / F::new(288.0) * t80867 + F::new(7.0) / F::new(48.0) * t80870 + F::new(7.0) / F::new(96.0) * t80872;
    t84529
}
