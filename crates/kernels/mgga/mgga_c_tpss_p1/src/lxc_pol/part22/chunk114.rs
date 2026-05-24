//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 114/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk114<F: Float>(t267: F, t270: F, t273: F, t279: F) -> (F, F, F) {
    let t301 = F::new(0.705945e1) * t270 + F::new(0.1549425e1) * t267 + F::new(0.420775e0) * t273 + F::new(0.1562925e0) * t279;
    let t304 = F::new(1.0) + F::cast_from(0.32163958997385070134e2_f64) / t301;
    let t305 = F::ln(t304);
    (t301, t304, t305)
}
