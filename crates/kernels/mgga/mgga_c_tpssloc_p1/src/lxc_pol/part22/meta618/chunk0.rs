//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2149/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2149<F: Float>(t3610: F, t52627: F, t1227: F, t1653: F, t248: F, t45293: F, t15730: F, t3536: F, t3577: F, t44951: F, t4953: F, t11677: F, t15245: F) -> (F, F, F, F, F) {
    let t52628 = t3610 * t52627;
    let t52680 = t1227 * t248 * t45293 * t1653;
    let t52731 = t3536 * t15730;
    let t52732 = t52731 / F::new(4608.0);
    let t52758 = t3577 * t44951 * t4953;
    let t52759 = t52758 / F::new(6912.0);
    let t52766 = t15245 * t11677;
    (t52628, t52680, t52732, t52759, t52766)
}
