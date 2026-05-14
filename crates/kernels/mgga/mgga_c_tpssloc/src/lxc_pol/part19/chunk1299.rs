//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1299/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1299<F: Float>(t1119: F, t11269: F, t3264: F, t11190: F, t3307: F, t3316: F, t11185: F, t11407: F, t1117: F, t3313: F, t3315: F, t43713: F, t43717: F, t43721: F, t43725: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43748: F, t43750: F, t43754: F) -> (F, F, F, F, F) {
    let t43997 = 8.0 * t3264 * t1119 * t11269;
    let t44000 = 0.57895126195293126241e3 * t11190 * t3316 * t3307;
    let t44002 = 0.1929837539843104208e3 * t11185 * t11407;
    let t44006 = 0.64327917994770140268e2 * t3313 * t11269 * t3315 * t1117;
    let t44021 = -0.98587999999999999999e0 * t43713 - 0.10954222222222222222e0 * t43717 + 0.295764e1 * t43721 + 0.65725333333333333332e0 * t43725 + 0.79724444444444444444e0 * t43727 - 0.23917333333333333334e1 * t43729 + 0.19931111111111111111e1 * t43734 - 0.71752000000000000001e1 * t43737 - 0.79724444444444444444e0 * t43740 + 0.107628e2 * t43743 + 0.23917333333333333333e1 * t43746 - 0.5314962962962962963e0 * t43748 - 0.44291358024691358024e0 * t43750 - 0.82156666666666666668e-1 * t43754;
    (t43997, t44000, t44002, t44006, t44021)
}
