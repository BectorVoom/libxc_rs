//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1373/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1373<F: Float>(t25992: F, t8607: F, t102344: F, t1874: F, t27188: F, t6525: F, t92090: F, t33603: F, t6876: F, t31297: F, t7685: F, t114360: F, t121181: F, t121184: F, t121190: F, t121192: F, t121194: F, t26974: F, t31055: F, t8329: F) -> F {
    let t121195 = t8607 * t25992;
    let t121197 = F::cast_from(2.0_f64) * t102344 * t1874;
    let t121199 = F::cast_from(2.0_f64) * t27188 * t6525;
    let t121201 = F::cast_from(2.0_f64) * t92090 * t1874;
    let t121203 = F::cast_from(3.0_f64) * t6876 * t33603;
    let t121204 = t7685 * t31297;
    let t121205 = -F::cast_from(3.0_f64) * t114360 * t26974 - t121181 + t121184 - t121190 - t121192 - t121194 - t121195 - t121197 - t121199 - t121201 + t121203 - t121204 - t31055 - t8329;
    t121205
}
