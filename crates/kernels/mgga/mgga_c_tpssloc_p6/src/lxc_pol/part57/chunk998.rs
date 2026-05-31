//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 998/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk998<F: Float>(t22986: F, t33447: F, t86873: F, t1880: F, t28294: F, t31366: F, t112936: F, t112942: F, t114933: F, t114944: F, t121405: F, t126427: F, t126497: F, t127952: F, t127955: F, t127979: F, t127990: F, t127998: F, t128020: F, t1528: F, t25168: F, t25188: F, t259: F, t28310: F, t33399: F, t33452: F, t4147: F, t4268: F, t5558: F, t7842: F, t8543: F, t855: F, t858: F, t92394: F) -> F {
    let t128035 = t22986 * t86873 * t33447;
    let t128040 = t1880 * t31366 * t28294;
    let t128042 = F::cast_from(4.0_f64) * t4147 * t33452 + F::cast_from(0.3289868133696452873e-1_f64) * t127952 - F::cast_from(0.16449340668482264365e-1_f64) * t127955 - t855 * t858 * (t127979 + t127990 + t127998 + t128020) - t126427 + t112936 - F::cast_from(2.0_f64) * t4268 * t33399 + t5558 * t8543 * t259 - t114933 - t112942 + F::cast_from(24.0_f64) * t25168 * t92394 * t28310 + t126497 - F::cast_from(2.0_f64) * t25188 * t7842 + F::cast_from(0.3289868133696452873e-1_f64) * t128035 - F::cast_from(2.0_f64) * t121405 * t1528 + F::cast_from(0.16449340668482264365e-1_f64) * t128040 + t114944;
    t128042
}
