//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 843/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk843<F: Float>(t1174: F, t1726: F, t1737: F, t3577: F, t488: F, t4889: F, t4957: F, t4959: F, t4994: F, t4998: F, t5002: F, t6158: F, t6165: F, t6170: F, t6178: F, t6184: F, t6188: F, t6192: F) -> F {
    let t6197 = -t6158 * t488 / F::new(288.0) + F::new(19.0) / F::new(1728.0) * t6165 * t488 + t6170 * t488 / F::new(3072.0) + t4957 / F::new(2304.0) - t4959 / F::new(432.0) - t4994 / F::new(3456.0) + t4998 / F::new(2304.0) + t1174 * t6178 / F::new(216.0) + t4889 * t1726 / F::new(54.0) - t1174 * t6184 / F::new(288.0) - t1174 * t6188 / F::new(144.0) - t3577 * t6192 / F::new(2304.0) + t5002 * t1737 / F::new(1536.0);
    t6197
}
