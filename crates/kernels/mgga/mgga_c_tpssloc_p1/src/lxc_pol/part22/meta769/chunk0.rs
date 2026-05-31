//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2610/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2610<F: Float>(t1174: F, t18577: F, t3440: F, t3447: F, t4889: F, t4980: F, t52953: F, t52974: F, t52988: F, t52992: F, t52994: F, t53187: F, t65703: F, t66153: F, t66155: F, t66165: F, t68513: F, t71181: F, t71185: F) -> F {
    let t72842 = t66153 / F::cast_from(216.0_f64) + t66155 / F::cast_from(216.0_f64) - t65703 * t4980 / F::cast_from(48.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t4889 * t18577 + t1174 * t3440 * t71181 / F::cast_from(72.0_f64) + t1174 * t3440 * t71185 / F::cast_from(72.0_f64) + t66165 / F::cast_from(144.0_f64) + t52953 - t52974 + t52988 - t52992 - t52994 - t3447 * t53187 * t68513 / F::cast_from(16.0_f64);
    t72842
}
