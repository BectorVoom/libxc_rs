//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 747/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk747<F: Float>(t2621: F, t4960: F, t912: F, t2698: F, t4573: F, t926: F, t2644: F, t4579: F, t929: F, t1464: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4961 = t4960 * t2621;
    let t4963 = F::cast_from(0.17315859105681463759e2_f64) * t912 * t4961;
    let t4965 = t2698 * t4573;
    let t4966 = t926 * t4965;
    let t4969 = t2644 * t4573;
    let t4970 = t926 * t4969;
    let t4973 = t929 * t4579;
    let t4974 = t926 * t4973;
    let t4977 = t1464 * t1464;
    (t4961, t4963, t4965, t4966, t4969, t4970, t4973, t4974, t4977)
}
