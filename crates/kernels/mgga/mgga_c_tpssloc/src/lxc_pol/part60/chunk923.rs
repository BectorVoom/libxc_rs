//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 923/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk923<F: Float>(t23139: F, t8339: F, t23171: F, t23228: F, t8335: F, t1902: F, t213: F, t225: F, t23030: F, t30638: F, t212: F, t6554: F) -> (F, F, F, F, F) {
    let t112855 = t23139 * t8339;
    let t112863 = F::cast_from(0.16449340668482264365e-1_f64) * t23171 * t23228 * t8335;
    let t112899 = t213 * t1902 * t225;
    let t112936 = F::cast_from(0.52089578783527170489e-1_f64) * t23030 * t30638;
    let t112942 = F::cast_from(0.16449340668482264365e-1_f64) * t23171 * t212 * t1902 * t6554;
    (t112855, t112863, t112899, t112936, t112942)
}
