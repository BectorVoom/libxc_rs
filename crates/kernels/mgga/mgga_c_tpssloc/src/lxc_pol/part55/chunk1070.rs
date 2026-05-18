//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1070/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1070<F: Float>(t7313: F, t8875: F, t2147: F, t7319: F, t7327: F, t7330: F, t1201: F, t8878: F, t1209: F, t483: F, t1017: F, t1207: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32425 = F::new(0.40372756094140390856e-3) * t7313 * t8875;
    let t32428 = t2147 * sigma2;
    let t32429 = t7319 * t32428;
    let t32432 = t7327 * sigma2;
    let t32433 = t32432 * t7330;
    let t32436 = t1201 * t8878;
    let t32439 = t1209 * t483;
    let t32440 = t32439 * t1017;
    let t32441 = t1207 * t32440;
    (t32425, t32428, t32429, t32432, t32433, t32436, t32439, t32440, t32441)
}
