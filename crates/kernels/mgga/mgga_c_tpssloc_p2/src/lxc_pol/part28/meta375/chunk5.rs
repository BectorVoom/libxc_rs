//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1439/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1439<F: Float>(t11137: F, t11139: F, t11141: F, t11143: F, t11247: F, t14702: F, t14708: F, t14721: F, t14723: F, t14724: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> F {
    let t14758 = -t11247 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11137 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11139 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11141 - t11143 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14702 + t14721 - t14723 - t14724 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t14728 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14733 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14738 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14742 + F::cast_from(2.0_f64) * t14746 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14751 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14755 + t14708 / F::cast_from(3.0_f64);
    t14758
}
