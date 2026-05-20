//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2370/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2370<F: Float>(t10375: F, t1612: F, t1041: F, t1539: F, t248: F, t42749: F, t14473: F, t2952: F, t10633: F, t4483: F, t47705: F, t47707: F) -> (F, F, F, F, F, F) {
    let t48670 = t1612 * t10375;
    let t48674 = t1041 * t248 * t42749 * t1539;
    let t48679 = F::cast_from(0.51947577317044391276e2_f64) * t14473 * t2952;
    let t48681 = F::cast_from(0.10254018858216406658e4_f64) * t4483 * t10633;
    let t48688 = F::cast_from(0.47488888888888888888e-1_f64) * t47705;
    let t48689 = F::cast_from(0.15829629629629629629e-1_f64) * t47707;
    (t48670, t48674, t48679, t48681, t48688, t48689)
}
