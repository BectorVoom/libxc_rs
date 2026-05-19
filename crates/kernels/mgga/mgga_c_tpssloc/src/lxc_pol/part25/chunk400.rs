//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 400/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk400<F: Float>(t1268: F, t2036: F, t2039: F, t1992: F, t2000: F, t2004: F) -> (F, F) {
    let t2079 = F::new(2.0) * t1268 * t2039 + t2036;
    let t2085 = t1992 / F::new(48.0) + F::cast_from(0.40372756094140390853e-3_f64) * t2000 + t2004 / F::new(768.0);
    (t2079, t2085)
}
