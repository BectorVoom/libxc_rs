//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 561/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk561<F: Float>(t14516: F, t2136: F, t14328: F, t14330: F, t14336: F, t14338: F, t14349: F, t14351: F, t14444: F, t333: F, t5266: F, t14333: F, t14335: F, t14354: F) -> (F, F, F, F) {
    let t14517 = t14516 * t2136;
    let t14518 = F::cast_from(0.10227998120342003148e-1_f64) * t14517;
    let t14519 = F::cast_from(0.79828278012425390427e-1_f64) * t14328;
    let t14520 = F::cast_from(0.14967802127329760705e-1_f64) * t14330;
    let t14521 = F::cast_from(0.54549323308490683456e-1_f64) * t14336;
    let t14522 = F::cast_from(0.16566831523319392755e-1_f64) * t14338;
    let t14523 = F::cast_from(0.44903406381989282115e-1_f64) * t14349;
    let t14524 = F::cast_from(0.14967802127329760705e-1_f64) * t14351;
    let t14525 = t14444 * t333;
    let t14527 = F::cast_from(0.11974241701863808564e0_f64) * t5266 * t14525;
    let t14528 = t14518 + t14519 - t14520 - t14333 + t14335 - t14521 + t14522 + t14523 + t14524 + t14354 + t14527;
    (t14519, t14521, t14522, t14528)
}
