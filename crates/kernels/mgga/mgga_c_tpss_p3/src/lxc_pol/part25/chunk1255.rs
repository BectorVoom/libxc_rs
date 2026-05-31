//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1255/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1255<F: Float>(t1288: F, t1692: F, t1812: F, t18728: F, t18812: F, t20514: F, t21263: F, t21266: F, t21270: F, t21353: F, t21356: F, t21359: F, t21583: F, t21659: F, t2439: F, t30: F, t3552: F, t4578: F, t5853: F, t6120: F, t6153: F, t6354: F) -> F {
    let t21677 = F::cast_from(3.0_f64) * t3552 * t21583 + F::cast_from(3.0_f64) * t2439 * t6354 * t6120 - F::cast_from(3.0_f64) * t18728 * t21263 + F::cast_from(3.0_f64) * t2439 * t1812 * t21266 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2439 * t1812 * t21270 + t1692 * t21659 * t30 / F::cast_from(2.0_f64) - t1692 * t20514 * t6153 + t1692 * t6354 * t1288 + t1692 * t18812 * t21353 - t1692 * t5853 * t21356 - t1692 * t5853 * t21359 / F::cast_from(2.0_f64) + t1692 * t1812 * t4578 / F::cast_from(2.0_f64);
    t21677
}
