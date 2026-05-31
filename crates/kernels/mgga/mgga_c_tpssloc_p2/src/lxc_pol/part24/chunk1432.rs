//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1432/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1432<F: Float>(t1873: F, t45814: F, t12739: F, t6534: F, t22479: F, t5113: F, t22461: F, t2363: F, t26103: F, t6517: F, t671: F, t83853: F, t83889: F, t83935: F, t83946: F, t83948: F, t83952: F, t83956: F, t83958: F, t83960: F, t83962: F, t9416: F) -> F {
    let t83964 = F::cast_from(2.0_f64) * t45814 * t1873;
    let t83966 = F::cast_from(6.0_f64) * t12739 * t6534;
    let t83968 = F::cast_from(6.0_f64) * t5113 * t22479;
    let t83969 = F::cast_from(6.0_f64) * t22461 * t2363 + F::cast_from(6.0_f64) * t2363 * t26103 + F::cast_from(2.0_f64) * t6517 * t9416 + F::cast_from(6.0_f64) * t671 * t83935 + t83853 + F::cast_from(6.0_f64) * t83889 + t83946 + t83948 + t83952 + t83956 + t83958 + t83960 + t83962 + t83964 + t83966 + t83968;
    t83969
}
