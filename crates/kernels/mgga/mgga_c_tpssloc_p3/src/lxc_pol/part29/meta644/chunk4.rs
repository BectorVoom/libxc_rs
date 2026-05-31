//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2126/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2126<F: Float>(t13184: F, t841: F, t87407: F, t23083: F, t25123: F, t81912: F, t81887: F, t81889: F, t81899: F, t81903: F, t81909: F, t87379: F, t87381: F, t87387: F, t87389: F, t87391: F, t87395: F, t87399: F, t87402: F, t87403: F, t87405: F) -> F {
    let t87409 = t87407 * t841 * t13184;
    let t87411 = t23083 * t25123;
    let t87412 = F::cast_from(0.28260929265898273598e-2_f64) * t87411;
    let t87414 = F::cast_from(0.22608743412718618878e-1_f64) * t81912;
    let t87415 = t87379 / F::cast_from(384.0_f64) + t87381 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t81887 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t81889 + F::cast_from(0.20186378047070195427e-3_f64) * t81899 + F::cast_from(0.20186378047070195427e-3_f64) * t81903 - F::cast_from(0.31625325607076639502e-2_f64) * t87387 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t87389 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t87391 + F::cast_from(0.24223653656484234512e-2_f64) * t87395 + F::cast_from(0.12111826828242117256e-2_f64) * t87399 - t87402 + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t87403 - F::cast_from(0.52708876011794399171e-3_f64) * t87405 - F::cast_from(0.67826230238155856634e-1_f64) * t87409 + t87412 + F::cast_from(0.14130464632949136799e-2_f64) * t81909 - t87414;
    t87415
}
