//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2227/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2227<F: Float>(t23185: F, t28426: F, t81914: F, t25248: F, t776: F, t87642: F, t98336: F, t81575: F, t87067: F, t87078: F, t92492: F, t98325: F, t98328: F, t98330: F, t98334: F, t98339: F, t98342: F, t98345: F, t98349: F, t98353: F, t98356: F, t98359: F) -> F {
    let t98363 = t23185 * t81914 * t28426;
    let t98367 = t87642 * t25248 * t98336 * t776;
    let t98370 = F::cast_from(0.3289868133696452873e-1_f64) * t98325 - F::cast_from(0.9869604401089358619e-1_f64) * t98328 - F::cast_from(0.11514538467937585055e0_f64) * t98330 + F::cast_from(0.82246703342411321825e-2_f64) * t98334 - F::cast_from(0.49348022005446793095e-1_f64) * t98339 + t87067 - t92492 - F::cast_from(0.41123351671205660912e-2_f64) * t98342 + F::cast_from(0.16449340668482264365e-1_f64) * t98345 - F::cast_from(0.16449340668482264365e-1_f64) * t98349 - F::cast_from(0.16449340668482264365e-1_f64) * t98353 + F::cast_from(0.82246703342411321825e-2_f64) * t98356 - F::cast_from(0.6579736267392905746e-1_f64) * t98359 + F::cast_from(0.16449340668482264365e-1_f64) * t81575 - F::cast_from(0.82246703342411321825e-2_f64) * t98363 - F::cast_from(0.19739208802178717238e0_f64) * t98367 - F::cast_from(0.23029076935875170111e0_f64) * t87078;
    t98370
}
