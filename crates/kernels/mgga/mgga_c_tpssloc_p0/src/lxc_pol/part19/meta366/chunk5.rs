//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1337/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1337<F: Float>(t42841: F, t9288: F, t3014: F, t4509: F, t10273: F, t2960: F, t10231: F, t10279: F, t973: F, t10186: F, t10235: F, t10237: F, t10238: F, t10242: F, t13798: F, t2986: F, t2991: F, t41693: F, t42827: F, t42830: F, t42833: F, t42839: F) -> F {
    let t42842 = t42841 * t9288;
    let t42846 = t4509 * t3014;
    let t42855 = t2960 * t10273;
    let t42858 = t973 * t10231 * t10279;
    let t42860 = F::cast_from(0.14814814814814814815e-2_f64) * t42827 - F::cast_from(0.32592592592592592592e-1_f64) * t42830 * t2991 + F::cast_from(0.59259259259259259256e-2_f64) * t42833 + F::cast_from(0.11851851851851851852e-1_f64) * t10186 * t10238 - F::cast_from(0.14814814814814814814e-2_f64) * t42839 + F::cast_from(0.88888888888888888886e-2_f64) * t2986 * t10235 * t42842 - F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t42846 * t10237 + F::cast_from(0.88888888888888888887e-2_f64) * t10186 * t10242 + F::cast_from(0.51851851851851851851e-2_f64) * t2986 * t13798 * t41693 + F::cast_from(0.59259259259259259256e-2_f64) * t42855 - F::cast_from(0.29629629629629629628e-2_f64) * t42858;
    t42860
}
