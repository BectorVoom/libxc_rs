//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 747/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk747<F: Float>(t26361: F, t225: F, t7919: F, t1824: F, t2085: F, t26393: F, t26406: F, t26429: F, t1338: F, t7918: F, t26127: F, t111: F, t7786: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27067 = F::cast_from(0.38381794893125283518e-1_f64) * t26361;
    let t27068 = t7919 * t225;
    let t27074 = t2085 * t1824;
    let t27082 = F::cast_from(0.16449340668482264365e-1_f64) * t26393;
    let t27088 = F::cast_from(0.38381794893125283518e-1_f64) * t26406;
    let t27096 = F::cast_from(0.38381794893125283518e-1_f64) * t26429;
    let t27097 = t1338 * t7918;
    let t27166 = F::new(2.0) / F::new(3.0) * t26127;
    let t27188 = t7786 * t111;
    (t27067, t27068, t27074, t27082, t27088, t27096, t27097, t27166, t27188)
}
