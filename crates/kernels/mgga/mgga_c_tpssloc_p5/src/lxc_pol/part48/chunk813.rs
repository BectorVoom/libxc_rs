//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 813/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk813<F: Float>(t28: F, t2161: F, t2250: F, t23820: F, t24916: F, t52: F, t607: F, t7402: F, t24562: F, t111: F, t7263: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t24923 = piecewise3::<F>(t401, t23820, t24916 * t52 / F::cast_from(2.0_f64) - t7402 * t607 - t2161 * t2250 / F::cast_from(2.0_f64));
    let t24924 = t24562 + t24923;
    let t24932 = t7263 * t111;
    (t24924, t24932)
}
