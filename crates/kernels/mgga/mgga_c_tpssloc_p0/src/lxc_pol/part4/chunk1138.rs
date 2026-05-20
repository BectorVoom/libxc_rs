//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1138/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1138<F: Float>(t18710: F, t449: F, t11137: F, t11247: F, t14702: F, t14721: F, t14723: F, t14724: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F) -> (F, F) {
    let t18711 = t18710 * t449;
    let t18730 = -t11247 + F::new(4.0) / F::new(27.0) * t11137 + F::new(8.0) / F::new(27.0) * t14702 + t14721 - t14723 - t14724 + F::new(2.0) / F::new(27.0) * t18203 + F::new(10.0) / F::new(27.0) * t18208 - F::new(4.0) / F::new(3.0) * t18213 - F::new(4.0) / F::new(9.0) * t18217 - F::new(2.0) / F::new(9.0) * t18219 + F::new(2.0) * t18223 + F::new(4.0) / F::new(3.0) * t18227 - t18229 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t18234 + F::new(2.0) / F::new(3.0) * t18239 + t18243 / F::new(3.0);
    (t18711, t18730)
}
