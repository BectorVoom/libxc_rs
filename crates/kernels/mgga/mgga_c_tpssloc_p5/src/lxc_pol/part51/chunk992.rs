//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 992/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk992<F: Float>(t1527: F, t6662: F, t2718: F, t225: F, t7492: F, t1484: F, t857: F, t865: F, t23270: F, t22986: F, t13065: F, t13463: F, t1528: F, t1912: F, t23206: F, t23209: F, t23231: F, t23232: F, t23278: F, t4268: F, t4273: F, t6627: F, t6632: F, t6663: F, t855: F, t866: F) -> (F, F, F, F, F, F) {
    let t25183 = t6662 * t1527;
    let t25184 = t2718 * t25183;
    let t25188 = t7492 * t225;
    let t25191 = t857 * t1484;
    let t25192 = t25191 * t865;
    let t25193 = t23270 * t25192;
    let t25194 = t22986 * t25193;
    let t25196 = -t13463 * t1912 - t4268 * t6663 + F::cast_from(0.82246703342411321824e-2_f64) * t23206 + F::cast_from(0.41123351671205660912e-2_f64) * t23209 - t23278 * t1528 + F::cast_from(2.0_f64) * t4268 * t6632 + F::cast_from(2.0_f64) * t6627 * t4273 + F::cast_from(2.0_f64) * t855 * t25184 - t23231 - t13065 * t1912 - t25188 * t866 + F::cast_from(0.38381794893125283518e-1_f64) * t23232 + F::cast_from(0.16449340668482264365e-1_f64) * t25194;
    (t25183, t25184, t25188, t25192, t25194, t25196)
}
