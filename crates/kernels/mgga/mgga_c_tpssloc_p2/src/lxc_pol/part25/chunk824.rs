//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 824/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk824<F: Float>(t10588: F, t901: F, t276: F, t285: F, t2799: F, t896: F, t273: F, t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F) -> (F, F, F, F) {
    let t10591 = t901 * t10588;
    let t10595 = F::cast_from(1.0_f64) / t276 / t285 / F::cast_from(4.0_f64);
    let t10596 = t2799 * t896;
    let t10597 = t10595 * t10596;
    let t10599 = F::cast_from(1.0_f64)/pow_3_2::<F>(t273);
    let t10600 = t10599 * t10596;
    let t10602 = F::cast_from(0.16557e0_f64) * t10311 - F::cast_from(0.49671e0_f64) * t10318 - F::cast_from(0.40256666666666666668e0_f64) * t10556 + F::cast_from(0.20128333333333333333e0_f64) * t10558 - F::cast_from(0.60385000000000000001e0_f64) * t10560 + F::cast_from(0.30192500000000000001e0_f64) * t10562 - F::cast_from(0.33547222222222222222e0_f64) * t10566 + F::cast_from(0.12077e1_f64) * t10569 - F::cast_from(0.181155e1_f64) * t10572 - F::cast_from(0.301925e0_f64) * t10575 + F::cast_from(0.258925e1_f64) * t10589 + F::cast_from(0.16504875e0_f64) * t10591 + F::cast_from(0.19419375e1_f64) * t10597 - F::cast_from(0.412621875e-1_f64) * t10600;
    (t10591, t10597, t10600, t10602)
}
