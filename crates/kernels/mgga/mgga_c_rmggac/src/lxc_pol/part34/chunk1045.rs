//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1045/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1045<F: Float>(t78049: F, t76323: F, t25820: F, t77085: F, t27101: F, t77088: F, t352: F, t76319: F, t76322: F, t76326: F, t77999: F, t78036: F, t78038: F, t78039: F, t78040: F, t78046: F, t78048: F, t8940: F) -> F {
    let t78050 = F::new(0.8980681276397856423e-1) * t78049;
    let t78051 = F::new(0.14967802127329760705e-1) * t76323;
    let t78052 = t25820 * t77085;
    let t78053 = F::new(0.8980681276397856423e-1) * t78052;
    let t78054 = t27101 * t77088;
    let t78055 = F::new(0.5987120850931904282e-1) * t78054;
    let t78056 = -t78036 - t78038 + t78039 + t78040 + t76319 + t76322 + F::new(0.11974241701863808564e0) * t8940 * t77999 * t352 + t78046 - t78048 - t78050 - t78051 + t76326 + t78053 + t78055;
    t78056
}
