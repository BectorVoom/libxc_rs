//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 691/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk691<F: Float>(t109: F, t652: F, t6525: F, t107: F, t625: F, t63: F, t656: F, t666: F) -> (F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t6527 = F::cast_from(2.0_f64) * t652 * t6525;
    let t6528 = t625 * t107;
    let t6529 = t6528 / F::cast_from(3.0_f64);
    let t6530 = t63 * t656;
    let t6531 = t6530 * t666;
    let t6534 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t6529 - t6531 / F::cast_from(8.0_f64));
    (t6527, t6530, t6534)
}
