//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2148/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2148<F: Float>(t13353: F, t23146: F, t13225: F, t23069: F, t4159: F, t23062: F, t25106: F, t13176: F, t6613: F, t831: F, t25146: F, t2681: F) -> (F, F, F, F, F, F) {
    let t87287 = t23146 * t13353;
    let t87289 = t23146 * t13225;
    let t87291 = t23069 * t4159;
    let t87292 = F::new(7.0) / F::new(72.0) * t87291;
    let t87293 = t23062 * t25106;
    let t87295 = t13176 * t6613;
    let t87296 = t87295 * t831;
    let t87298 = t25146 * t2681;
    (t87287, t87289, t87292, t87293, t87296, t87298)
}
