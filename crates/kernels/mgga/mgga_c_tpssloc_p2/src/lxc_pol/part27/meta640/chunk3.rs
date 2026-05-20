//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2166/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2166<F: Float>(t23110: F, t25299: F, t81651: F, t23168: F, t25313: F, t13176: F, t226: F, t235: F, t25256: F, t25261: F, t2617: F, t4281: F, t6658: F, t81617: F, t87150: F, t87154: F, t87155: F, t87159: F, t87166: F, t87167: F, t87171: F, t87174: F, t87177: F, t87512: F, t87517: F, t9632: F) -> F {
    let t87520 = t81651 * t23110 * t25299;
    let t87521 = F::cast_from(0.16449340668482264365e-1_f64) * t87520;
    let t87522 = t23168 * t25313;
    let t87523 = F::cast_from(0.76763589786250567036e-1_f64) * t87522;
    let t87524 = F::cast_from(0.16449340668482264365e-1_f64) * t87150 - t87154 + F::cast_from(0.26044789391763585244e-1_f64) * t87155 + F::cast_from(0.3289868133696452873e-1_f64) * t87159 + F::new(2.0) * t4281 * t25261 * t9632 + t87166 + t87167 - F::cast_from(0.19190897446562641759e-1_f64) * t81617 + F::cast_from(0.3289868133696452873e-1_f64) * t87171 - F::cast_from(0.16449340668482264365e-1_f64) * t87174 + F::cast_from(0.82246703342411321824e-2_f64) * t87177 - F::new(2.0) * t2617 * t25256 - F::new(2.0) * t13176 * t6658 + t226 * t235 * t87512 + F::cast_from(0.16449340668482264365e-1_f64) * t87517 - t87521 + t87523;
    t87524
}
