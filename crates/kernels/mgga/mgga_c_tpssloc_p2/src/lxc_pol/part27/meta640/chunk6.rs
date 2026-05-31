//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2169/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2169<F: Float>(t81633: F, t13453: F, t1499: F, t23151: F, t25261: F, t25281: F, t2684: F, t4291: F, t81623: F, t81630: F, t81642: F, t81653: F, t87527: F, t87531: F, t87534: F, t87536: F, t87538: F, t87541: F, t87545: F, t87547: F, t87554: F) -> F {
    let t87559 = F::cast_from(0.25587863262083522346e0_f64) * t81633;
    let t87562 = -F::cast_from(0.16449340668482264365e-1_f64) * t87527 - F::cast_from(0.6579736267392905746e-1_f64) * t87531 + t87534 + t87536 - F::cast_from(0.82246703342411321825e-2_f64) * t87538 + F::cast_from(0.3289868133696452873e-1_f64) * t87541 - t87545 - t87547 + F::cast_from(4.0_f64) * t13453 * t25281 - t4291 * t25261 * t2684 - F::cast_from(0.16449340668482264365e-1_f64) * t87554 + t1499 * t23151 + F::cast_from(0.76763589786250567036e-1_f64) * t81623 + F::cast_from(0.82246703342411321824e-2_f64) * t81630 - t87559 - F::cast_from(0.24674011002723396547e-1_f64) * t81642 - F::cast_from(0.16449340668482264365e-1_f64) * t81653;
    t87562
}
