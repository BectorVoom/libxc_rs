//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1474/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1474<F: Float>(t11678: F, t1214: F, t1735: F, t19083: F, t21776: F, t22012: F, t22185: F, t22309: F, t248: F, t3577: F, t3578: F, t44725: F, t44863: F, t45250: F, t4889: F, t5024: F, t53238: F, t53440: F, t5979: F, t6203: F, t6225: F, t66545: F, t73084: F, t73096: F, t73099: F, t73102: F, t79018: F) -> F {
    let t79349 = -t73084 / F::new(576.0) - F::new(2.0) / F::new(81.0) * t66545 - t73096 / F::new(384.0) + F::new(5.0) / F::new(1728.0) * t73099 - F::new(5.0) / F::new(216.0) * t19083 * t6203 - F::new(5.0) / F::new(108.0) * t5024 * t22185 + t53238 * t22309 / F::new(128.0) + t44863 * t248 * t1214 * t79018 * t44725 / F::new(128.0) - t73102 / F::new(72.0) - t3577 * t3578 * t1735 * t21776 / F::new(1152.0) - t11678 * t3578 * t6225 * t5979 / F::new(384.0) - t45250 - F::new(5.0) / F::new(972.0) * t53440 + F::new(28.0) / F::new(243.0) * t4889 * t22012;
    t79349
}
