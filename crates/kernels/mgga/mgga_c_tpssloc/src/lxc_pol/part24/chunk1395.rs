//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1395/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1395<F: Float>(t10993: F, t6717: F, t10981: F, t10984: F, t10325: F, t10932: F, t10944: F, t10988: F, t1935: F, t23422: F, t23489: F, t23504: F, t3143: F, t3148: F, t3153: F, t343: F, t360: F, t6734: F, t82987: F, t82990: F, t83134: F, t83139: F, t83142: F, t83153: F, t83157: F) -> F {
    let t83159 = t6717 * t10993;
    let t83165 = t6717 * t10981;
    let t83167 = t6717 * t10984;
    let t83171 = -t6717 * t10932 / F::new(36.0) + F::new(0.48447307312968469026e-2) * t83134 + F::new(7.0) / F::new(648.0) * t6717 * t10944 - F::new(0.60559134141210586284e-3) * t83139 + F::new(0.10093189023535097714e-3) * t82987 * t83142 * t82990 * t360 + F::new(0.30279567070605293142e-3) * t23489 * t23504 - F::new(0.10093189023535097714e-3) * t1935 * t10325 * t343 * t6734 - t83153 / F::new(54.0) + t23422 * t3153 / F::new(18.0) - t83157 / F::new(432.0) - t83159 / F::new(144.0) - t23422 * t3143 / F::new(36.0) - t23422 * t3148 / F::new(27.0) + t83165 / F::new(288.0) + t83167 / F::new(216.0) + t6717 * t10988 / F::new(288.0);
    t83171
}
