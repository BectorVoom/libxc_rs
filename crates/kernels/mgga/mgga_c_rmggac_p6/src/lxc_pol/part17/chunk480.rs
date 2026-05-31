//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 480/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk480<F: Float>(t53: F, t1797: F, t983: F, t1375: F, t280: F, t437: F, t5860: F, t6042: F, t815: F, t1802: F, t4408: F, t1805: F, t990: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t6047 = t983 * t1797;
    let t6053 = piecewise3::<F>(t54, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t6042 * t280 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1375 * t815 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6047 * t280 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t437 * t5860);
    let t6054 = t4408 * t1802;
    let t6059 = t990 * t1805;
    (t6053, t6054, t6059)
}
