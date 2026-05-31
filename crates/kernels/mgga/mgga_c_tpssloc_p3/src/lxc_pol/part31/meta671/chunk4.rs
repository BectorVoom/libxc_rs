//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2005/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2005<F: Float>(t84242: F, t84248: F, t84280: F, t91961: F, t91980: F, t91996: F, t92001: F, t92003: F, t92008: F, t92012: F, t92031: F, t92034: F) -> F {
    let t102284 = t91961 + t91980 + F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t91996 - t92001 + F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t92003 - t92008 - t92012 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t84242 - F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t84248 - t84280 - t92031 - t92034;
    t102284
}
