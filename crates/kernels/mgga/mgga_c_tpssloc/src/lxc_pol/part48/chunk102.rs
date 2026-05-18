//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 102/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk102<F: Float>(t273: F, t276: F, t279: F, t285: F) -> (F, F, F, F, F) {
    let t302 = F::new(1.0) + F::new(0.5137e-1) * t273;
    let t307 = F::new(0.705945e1) * t276 + F::new(0.1549425e1) * t273 + F::new(0.420775e0) * t279 + F::new(0.1562925e0) * t285;
    let t310 = F::new(1.0) + F::new(0.32163958997385070134e2) / t307;
    let t311 = f64::ln(t310);
    let t315 = F::new(1.0) + F::new(0.278125e-1) * t273;
    (t302, t307, t310, t311, t315)
}
