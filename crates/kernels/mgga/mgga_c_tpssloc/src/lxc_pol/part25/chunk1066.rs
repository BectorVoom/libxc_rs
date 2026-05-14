//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1066/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1066<F: Float>(t12248: F, t2085: F, t12238: F, t12251: F, t12255: F, t1332: F, t1336: F, t2089: F, t24103: F, t24117: F, t24121: F, t24127: F, t3777: F, t81187: F, t81189: F, t81193: F, t81197: F, t81209: F, t81213: F, t81216: F, t81218: F, t81222: F, t81225: F, t81230: F, t81234: F, t81238: F) -> (F,) {
    let t84627 = t12248 * t2085;
    let t84634 = t12238 * t2089 - 0.76763589786250567036e0 * t81187 + 0.46058153871750340221e0 * t81189 + 0.29608813203268075857e0 * t81193 + 0.9869604401089358619e-1 * t81197 - 0.9869604401089358619e-1 * t81209 - 0.3289868133696452873e-1 * t81213 + 0.49348022005446793095e-1 * t81216 + 0.23029076935875170111e0 * t81218 - 0.19739208802178717238e0 * t81222 - 0.49348022005446793095e-1 * t81225 + 3.0 * t1332 * t24121 - 0.9869604401089358619e-1 * t81230 + 0.19739208802178717238e0 * t81234 + 0.9869604401089358619e-1 * t81238 - 6.0 * t3777 * t24117 - 3.0 * t3777 * t24103 - 6.0 * t1336 * t84627 * t12251 + 6.0 * t1336 * t24127 * t12255;
    (t84634,)
}
