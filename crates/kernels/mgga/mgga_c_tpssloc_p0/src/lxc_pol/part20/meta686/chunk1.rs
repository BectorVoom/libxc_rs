//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2600/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2600<F: Float>(t1174: F, t14726: F, t44562: F, t3577: F, t44951: F, t4953: F, t11677: F, t15245: F, t11665: F, t11668: F, t11670: F, t11694: F, t1177: F, t11853: F, t1227: F, t1230: F, t15569: F, t15714: F, t248: F, t3243: F, t3515: F, t44851: F, t44871: F, t4582: F, t4977: F, t5012: F, t50830: F, t50929: F) -> F {
    let t52751 = t1174 * t44562 * t14726;
    let t52758 = t3577 * t44951 * t4953;
    let t52759 = t52758 / F::cast_from(6912.0_f64);
    let t52766 = t15245 * t11677;
    let t52769 = -t1227 * t248 * t1230 * t50830 / F::cast_from(4608.0_f64) + t44851 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3577 * t11668 * t5012 * t3243 + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t11665 * t15714 - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t52751 - t1174 * t1177 * t50929 / F::cast_from(48.0_f64) + t44871 / F::cast_from(768.0_f64) + t52759 - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t15569 * t11670 - t3515 * t4582 * t4977 * t11853 / F::cast_from(3072.0_f64) + t52766 * t11694 / F::cast_from(1536.0_f64);
    t52769
}
