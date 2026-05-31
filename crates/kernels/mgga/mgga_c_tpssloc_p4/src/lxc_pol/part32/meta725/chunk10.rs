//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2339/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2339<F: Float>(t27817: F, t7999: F, t1238: F, t14972: F, t15797: F, t1716: F, t1751: F, t17635: F, t19209: F, t19220: F, t19232: F, t24589: F, t24601: F, t27444: F, t27453: F, t27766: F, t29795: F, t3593: F, t3598: F, t460: F, t4940: F, t498: F, t6267: F, t7283: F, t7286: F, t7351: F, t7391: F, t7392: F, t8054: F, t8061: F, t86473: F, t95834: F) -> F {
    let t104589 = t7999 * t27817;
    let t104596 = F::cast_from(2.0_f64) * t1238 * t3598 * t7391 * t6267 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t27444 * t17635 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27453 * t460 * t1751 * t7286 - t95834 - t19232 * t7392 + F::cast_from(2.0_f64) * t4940 * t8054 * t498 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t27766 + F::cast_from(2.0_f64) * t7351 * t19220 + F::cast_from(4.0_f64) * t14972 * t8061 - F::cast_from(0.14621636149762012769e-1_f64) * t104589 - t3593 * t29795 + F::cast_from(4.0_f64) * t15797 * t8061 + F::cast_from(0.6092348395734171987e-3_f64) * t86473 - t7351 * t19209;
    t104596
}
