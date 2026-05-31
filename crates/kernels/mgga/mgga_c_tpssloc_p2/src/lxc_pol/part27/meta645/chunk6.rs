//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2213/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2213<F: Float>(t23479: F, t25637: F, t6722: F, t1409: F, t344: F, t1009: F, t6740: F, t23473: F, t13528: F, t13542: F, t13931: F, t14130: F, t1618: F, t1920: F, t1933: F, t1934: F, t1935: F, t23414: F, t23419: F, t23495: F, t25601: F, t25609: F, t2987: F, t343: F, t4509: F, t4540: F, t6730: F, t6734: F, t6735: F, t7578: F, t82880: F, t83004: F, t83025: F, t83028: F) -> F {
    let t88440 = F::cast_from(0.16149102437656156342e-2_f64) * t6722 * t25637 * t23479;
    let t88449 = t1409 * t344;
    let t88451 = t6740 * t88449 * t1009;
    let t88453 = F::cast_from(0.20186378047070195428e-3_f64) * t88451 * t23473;
    let t88472 = t88440 - t82880 * t1618 / F::cast_from(144.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t1934 * t4540 * t6735 - F::cast_from(0.10093189023535097714e-3_f64) * t25601 * t23495 + t88453 - t1920 * t2987 * t13542 / F::cast_from(72.0_f64) + t1920 * t4509 * t13528 / F::cast_from(108.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t13931 * t343 * t6734 - F::cast_from(0.10093189023535097714e-3_f64) * t23414 * t7578 - F::cast_from(0.20186378047070195428e-3_f64) * t6730 * t25609 + t83004 / F::cast_from(1728.0_f64) - t23419 * t14130 / F::cast_from(1152.0_f64) + t83025 / F::cast_from(81.0_f64) + t83028;
    t88472
}
