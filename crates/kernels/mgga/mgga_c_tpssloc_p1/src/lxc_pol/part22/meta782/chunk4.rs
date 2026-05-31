//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2676/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2676<F: Float>(t1358: F, t20596: F, t12283: F, t20442: F, t120: F, t20356: F, t20465: F, t1351: F, t40046: F, t12429: F, t1352: F, t16224: F, t16233: F, t16305: F, t16306: F, t16394: F, t1825: F, t19744: F, t19876: F, t19945: F, t19976: F, t19994: F, t20004: F, t20450: F, t20463: F, t3803: F, t40168: F, t5246: F, t5248: F, t5308: F, t54048: F, t54744: F, t6388: F, t74120: F) -> (F, F) {
    let t74578 = t20596 * t1358;
    let t74584 = t12283 * t20442;
    let t74592 = t120 * t20356;
    let t74597 = t12283 * t20465;
    let t74599 = t40046 * t1351;
    let t74610 = -F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t1825 * t19994 + t3803 * t16305 * t16306 * t20463 / F::cast_from(256.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t74578 - t54048 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t5246 * t16224 * t6388 * t5308 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t74584 - t16394 * t19976 / F::cast_from(1024.0_f64) - t19876 * t20004 / F::cast_from(128.0_f64) + t19876 * t19945 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t3803 * t40168 * t74592 * t1352 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t74597 + t54744 * t5248 * t74120 * t74599 / F::cast_from(128.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t16233 * t5248 * t74120 * t19744 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t12429 * t20450;
    (t74599, t74610)
}
