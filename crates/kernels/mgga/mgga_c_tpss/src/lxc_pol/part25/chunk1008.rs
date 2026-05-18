//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1008/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1008<F: Float>(t1259: F, t13940: F, t1256: F, t1266: F, t13035: F, t13867: F, t13869: F, t13880: F, t13884: F, t13889: F, t1657: F, t3360: F, t4490: F, t4494: F, t4517: F, t538: F, t5433: F, t5449: F) -> (F, F) {
    let t13941 = t1259 * t13940;
    let t13943 = -F::new(6.0) * t1256 * t13880 + F::new(4.0) * t1256 * t13884 + F::new(2.0) * t1256 * t13889 - t1256 * t13941 - t1266 * t13869 - F::new(2.0) * t13035 * t1657 + t13867 * t538 + F::new(2.0) * t3360 * t5433 - t3360 * t5449 + F::new(4.0) * t4490 * t4494 - F::new(2.0) * t4490 * t4517;
    (t13941, t13943)
}
