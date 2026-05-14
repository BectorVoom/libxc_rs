//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 878/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk878<F: Float>(t76340: F, t76343: F, t69404: F, t352: F, t5148: F, t72038: F, t77894: F, t78060: F, t78061: F, t78062: F, t78065: F, t78067: F, t78069: F, t78072: F, t78073: F, t570: F, t71916: F, t8940: F) -> (F, F) {
    let t78077 = 0.13637330827122670865e0 * t76340;
    let t78078 = 0.5454932330849068346e-1 * t76343;
    let t78079 = 0.79828278012425390427e-1 * t69404;
    let t78080 = -t78060 + t78061 + t78062 + t78065 - t78067 - t78069 + t78072 + t78073 - 0.11974241701863808564e0 * t5148 * t77894 * t352 + t78077 + t78078 - t78079 - t72038;
    let t78083 = 0.11974241701863808564e0 * t8940 * t71916 * t570;
    (t78080, t78083)
}
