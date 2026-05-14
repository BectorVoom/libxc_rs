//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1123/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1123<F: Float>(t2144: F, t8034: F, t34339: F, t85639: F, t117803: F, t117809: F, t117813: F, t117823: F, t117834: F, t117838: F, t1653: F, t2128: F, t24589: F, t24601: F, t27432: F, t27433: F, t27437: F, t27549: F, t27751: F, t27775: F, t27820: F, t32482: F, t32510: F, t32515: F, t32529: F, t34338: F, t4930: F, t4936: F, t5089: F, t7283: F, t7287: F, t86415: F, t8871: F, t94378: F, t94558: F) -> (F,) {
    let t125148 = t8034 * t2144;
    let t125165 = t85639 * t34339;
    let t125182 = 0.54831135561607547883e-2 * t24589 * t24601 * t117813 * t1653 + 0.54831135561607547883e-2 * t24589 * t86415 * t34338 + 0.54831135561607547883e-2 * t24589 * t117809 * t27433 + 0.54831135561607547883e-2 * t24589 * t125148 * t7287 + 0.16449340668482264365e-1 * t2128 * t4936 * t32515 + 0.54831135561607547883e-2 * t24589 * t117809 * t27437 - 0.10966227112321509577e-1 * t24589 * t94378 * t117803 * t27432 + 0.73108180748810063844e-2 * t27549 * t117809 * t27775 + 0.18277045187202515961e-2 * t125165 - 0.16449340668482264365e-1 * t7283 * t27751 * t32529 - 0.16449340668482264365e-1 * t7283 * t94558 * t8871 + 0.54831135561607547883e-2 * t117823 - t32482 * t5089 - 0.54831135561607547883e-2 * t117834 + 0.16449340668482264365e-1 * t7283 * t4930 * t32515 - t117838 - 0.3289868133696452873e-1 * t2128 * t27820 * t32510;
    (t125182,)
}
