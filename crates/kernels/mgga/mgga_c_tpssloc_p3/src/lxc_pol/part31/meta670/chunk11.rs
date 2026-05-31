//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2000/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2000<F: Float>(t2031: F, t96461: F, t96469: F, t22549: F, t23963: F, t26009: F, t26016: F, t26954: F, t34125: F, t84216: F, t84229: F, t90101: F, t90104: F, t91922: F, t92040: F, t92052: F, t9239: F, t96418: F, t96458: F, t96466: F) -> F {
    let t102163 = t2031 * t96461;
    let t102168 = t2031 * t96469;
    let t102171 = -F::cast_from(880.0_f64) / F::cast_from(27.0_f64) * t91922 - F::cast_from(70.0_f64) * t84216 * t96418 - F::cast_from(40.0_f64) * t9239 * t34125 * t26009 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t84229 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t90101 * t26954 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t90104 * t26954 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t26016 * t92040 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t26016 * t92052 + F::cast_from(20.0_f64) * t23963 * t96458 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t22549 * t102163 + F::cast_from(10.0_f64) * t23963 * t96466 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t102168;
    t102171
}
